//! 渲染图（render graph）的三个小 pass 辅助（layer A4 里被 A1 调用的部分）。
//!
//! 证据（O，逐条核对机器码）：
//!  * `clear_rg_buffer`            0x1409ddbb0 + 闭包 `recorder::impl$1::render` 0x140904990
//!  * `clear_rg_color_image`       0x1409dde90 + 闭包 0x140904a30
//!  * `rg_buffer_copy_from_slice<u32>` 0x14090d9a0 + 闭包 `call_once` 0x14169a650
//!  * `RenderContext::finish`      0x1409dda50（348 B）：只做若干 `ManagedHandle::drop` 与释放，**无业务逻辑**，不还原。
//!
//! 共同模板：`PassBuilder::new(ctx, name)` → `add_resource_access(id, 0x34, 0)` → 登记一个 render 回调。
//! 渲染图本身（rhapsody::rg）是外部库，以 `rg_boundary` 占位，只复刻观察到的调用形状。

/// 占位：`rhapsody::rg::{pass_builder, pass_api, resource_registry}`。
pub mod rg_boundary {
    use alloc::sync::Arc;
    use core::{any::Any, fmt, ops::Range};

    /// Host-owned resource retained while a declaration or draw is unsubmitted.
    /// The backend transfers its GPU owners into the recorded command on use.
    #[derive(Clone)]
    pub struct ResourceLease(Arc<dyn Any + Send + Sync>);
    impl ResourceLease {
        pub fn new<T: Any + Send + Sync>(owner: Arc<T>) -> Self {
            Self(owner)
        }
        pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
            self.0.downcast_ref()
        }
        pub fn try_unwrap<T: Any + Send + Sync>(self) -> Result<T, Self> {
            match Arc::downcast::<T>(self.0) {
                Ok(owner) => Arc::try_unwrap(owner).map_err(|owner| Self(owner)),
                Err(owner) => Err(Self(owner)),
            }
        }
    }
    impl fmt::Debug for ResourceLease {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ResourceLease").finish_non_exhaustive()
        }
    }
    impl PartialEq for ResourceLease {
        fn eq(&self, other: &Self) -> bool {
            Arc::ptr_eq(&self.0, &other.0)
        }
    }
    impl Eq for ResourceLease {}
    #[cfg(test)]
    mod lease_ownership_tests {
        use super::*;
        #[test]
        fn exclusive_recovery_preserves_wrong_type_and_shared_owners() {
            let owner = Arc::new(vec![7u8; 128]);
            let pointer = owner.as_ptr();
            let shared = ResourceLease::new(owner.clone());
            let shared = shared.try_unwrap::<Vec<u8>>().unwrap_err();
            assert_eq!(shared.downcast_ref::<Vec<u8>>().unwrap().as_ptr(), pointer);
            drop(owner);
            let wrong = shared.try_unwrap::<Vec<u32>>().unwrap_err();
            let recovered = wrong.try_unwrap::<Vec<u8>>().unwrap();
            assert_eq!(recovered.as_ptr(), pointer);
            assert_eq!(recovered, vec![7u8; 128]);
        }
    }

    /// `add_resource_access` 的访问码（O：立即数 0x34；位含义 U，按“传输写”使用）。
    pub const ACCESS_TRANSFER_WRITE: u8 = 0x34;

    /// 在 pass 内执行的命令接口（`RenderPassApi`）。
    pub trait RenderPassApi {
        /// 闭包 0x140904990：`fill_buffer(buffer, range=None, value=0)`。
        fn fill_buffer(&mut self, buffer: u32, range: Option<Range<u64>>, value: u32);
        /// 闭包 0x140904a30：`clear_color_image(image)`（无颜色参数）。
        fn clear_color_image(&mut self, image: u32);
    }

    /// 动态常量（逐帧环形上传缓冲）中的一次分配。
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct StagingAlloc {
        pub buffer: u64,
        pub offset: u32,
        pub bytes: Vec<u8>,
        pub lease: Option<ResourceLease>,
    }

    /// Complete source allocation, declared before command recording. The host
    /// imports its actual buffer and declares both physical access ranges.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct StagingCopy {
        pub source: StagingAlloc,
        pub target: u32,
        pub dst_offset: u64,
    }

    /// `Send`：rhapsody 的回调可能在并行录制线程上执行；这些闭包只捕获资源号/数值。
    pub type RenderFn = Box<dyn FnOnce(&mut dyn RenderPassApi) + Send>;

    /// A version belongs to an explicit, stable CPU content source. The host
    /// also keys the complete uploaded bytes, so a missed version bump cannot
    /// return stale data. Unknown content always uses the uncached path.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum UploadContentVersion {
        #[default]
        Unknown,
        Complete {
            source: u64,
            version: u64,
        },
    }

    /// 渲染图上下文（`RenderContext`，pass 登记表在 +0x90 的 Vec 里，每项 0x40 B）。
    pub trait RenderGraph {
        /// `PassBuilder::new(name)` + `add_resource_access(id, access, 0)` + 登记回调。
        fn add_pass(&mut self, name: &'static str, resource: u32, access: u8, render: RenderFn);
        /// `DynamicConstants::allocate(size)`；失败（无该资源）原版 `Option::unwrap` panic。
        fn allocate_dynamic(&mut self, size: usize) -> rhapsody::Result<StagingAlloc>;
        fn copy_from_staging(
            &mut self,
            _name: &'static str,
            _copies: Vec<StagingCopy>,
        ) -> rhapsody::Result<()> {
            Err(rhapsody::Error::unsupported(
                "declared staging imports are not supported by this graph host",
            ))
        }
        /// Optional persistent immutable upload source. `false` explicitly
        /// selects the ordinary staging route; it does not publish a cache hit.
        fn upload_complete_bytes(
            &mut self,
            _buffer: u32,
            _bytes: &[u8],
            _source: u64,
            _version: u64,
        ) -> rhapsody::Result<bool> {
            Ok(false)
        }
    }
}

use rg_boundary::*;

/// `clear_rg_buffer`（O）：pass 名 "Clear buffer"（12 字节）；回调 `fill_buffer(buffer, None, 0)`。
pub fn clear_rg_buffer(graph: &mut dyn RenderGraph, buffer: u32) {
    graph.add_pass(
        "Clear buffer",
        buffer,
        ACCESS_TRANSFER_WRITE,
        Box::new(move |api| api.fill_buffer(buffer, None, 0)),
    );
}

/// `clear_rg_color_image`（O）：pass 名 "Clear image"（11 字节）；回调 `clear_color_image(image)`。
pub fn clear_rg_color_image(graph: &mut dyn RenderGraph, image: u32) {
    graph.add_pass(
        "Clear image",
        image,
        ACCESS_TRANSFER_WRITE,
        Box::new(move |api| api.clear_color_image(image)),
    );
}

/// `rg_buffer_copy_from_slice<u32>`（O）：
///  1. `assert!(data.len() * 4 <= buffer.desc.size)`（O：原文案 0x142aab1e0，50 字节 `assertion failed: data.len() <= buffer.desc().size`；比较的是字节数 `len*4`）；
///  2. 向 `DynamicConstants` 申请 `len*4` 字节暂存区，拷入数据（长度不符 → `copy_from_slice` panic）；
///  3. 登记 pass "Buffer memcpy"（13 字节）+ 目的缓冲访问 0x34，回调里 `copy_buffer(暂存区 → buffer)`。
pub fn rg_buffer_copy_from_slice_u32(
    graph: &mut dyn RenderGraph,
    buffer: u32,
    buffer_size: u64,
    data: &[u32],
    content: UploadContentVersion,
) -> rhapsody::Result<()> {
    let bytes = data
        .len()
        .checked_mul(4)
        .ok_or_else(|| rhapsody::Error::invalid_usage("buffer upload byte count overflows"))?;
    if bytes as u64 > buffer_size {
        return Err(rhapsody::Error::invalid_usage(
            "buffer upload exceeds the target",
        ));
    }
    if bytes == 0 {
        return Ok(());
    }
    if let UploadContentVersion::Complete { source, version } = content {
        let mut contents = Vec::new();
        contents
            .try_reserve_exact(bytes)
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        contents.extend(data.iter().flat_map(|word| word.to_ne_bytes()));
        if graph.upload_complete_bytes(buffer, &contents, source, version)? {
            return Ok(());
        }
    }
    let mut staging = graph.allocate_dynamic(bytes)?;
    if staging.bytes.len() != bytes {
        return Err(rhapsody::Error::invalid_usage(
            "staging allocation has the wrong length",
        ));
    }
    for (dst, v) in staging.bytes.chunks_exact_mut(4).zip(data) {
        dst.copy_from_slice(&v.to_ne_bytes());
    }
    graph.copy_from_staging(
        "Buffer memcpy",
        vec![StagingCopy {
            source: staging,
            target: buffer,
            dst_offset: 0,
        }],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_upload_contract_carries_every_byte_and_unknown_never_uses_cache() {
        #[derive(Default)]
        struct Probe {
            cached: usize,
            staging: usize,
        }
        impl RenderGraph for Probe {
            fn add_pass(&mut self, _: &'static str, _: u32, _: u8, _: RenderFn) {}
            fn copy_from_staging(
                &mut self,
                _: &'static str,
                copies: Vec<StagingCopy>,
            ) -> rhapsody::Result<()> {
                assert_eq!(copies.len(), 1);
                assert_eq!(copies[0].target, 7);
                assert_eq!(
                    copies[0].source.bytes,
                    [5_u32.to_ne_bytes(), 9_u32.to_ne_bytes()].concat()
                );
                Ok(())
            }
            fn allocate_dynamic(&mut self, size: usize) -> rhapsody::Result<StagingAlloc> {
                self.staging += 1;
                Ok(StagingAlloc {
                    buffer: 1,
                    offset: 0,
                    bytes: vec![0; size],
                    lease: None,
                })
            }
            fn upload_complete_bytes(
                &mut self,
                target: u32,
                bytes: &[u8],
                source: u64,
                version: u64,
            ) -> rhapsody::Result<bool> {
                assert_eq!((target, source, version), (7, 19, 3));
                assert_eq!(bytes, [5_u32.to_ne_bytes(), 9_u32.to_ne_bytes()].concat());
                self.cached += 1;
                Ok(true)
            }
        }
        let mut graph = Probe::default();
        rg_buffer_copy_from_slice_u32(
            &mut graph,
            7,
            8,
            &[5, 9],
            UploadContentVersion::Complete {
                source: 19,
                version: 3,
            },
        )
        .unwrap();
        assert_eq!((graph.cached, graph.staging), (1, 0));
        rg_buffer_copy_from_slice_u32(&mut graph, 7, 8, &[5, 9], UploadContentVersion::Unknown)
            .unwrap();
        assert_eq!((graph.cached, graph.staging), (1, 1));
    }

    #[derive(Default)]
    struct Rec {
        passes: Vec<(&'static str, u32, u8, RenderFn)>,
        copies: Vec<(&'static str, Vec<StagingCopy>)>,
    }
    impl RenderGraph for Rec {
        fn add_pass(&mut self, name: &'static str, r: u32, a: u8, f: RenderFn) {
            self.passes.push((name, r, a, f));
        }
        fn allocate_dynamic(&mut self, size: usize) -> rhapsody::Result<StagingAlloc> {
            Ok(StagingAlloc {
                buffer: 3,
                offset: 256,
                bytes: vec![0; size],
                lease: None,
            })
        }
        fn copy_from_staging(
            &mut self,
            name: &'static str,
            copies: Vec<StagingCopy>,
        ) -> rhapsody::Result<()> {
            self.copies.push((name, copies));
            Ok(())
        }
    }
    #[derive(Default)]
    struct Api(Vec<String>);
    impl RenderPassApi for Api {
        fn fill_buffer(&mut self, b: u32, r: Option<core::ops::Range<u64>>, v: u32) {
            self.0.push(format!("fill {b} {r:?} {v}"));
        }
        fn clear_color_image(&mut self, i: u32) {
            self.0.push(format!("clear {i}"));
        }
    }

    #[test]
    fn staging_failure_preserves_the_backend_error_without_registering_a_pass() {
        struct FailedGraph(Option<rhapsody::Error>);
        impl RenderGraph for FailedGraph {
            fn add_pass(&mut self, _: &'static str, _: u32, _: u8, _: RenderFn) {
                panic!("failed allocation must not declare a copy pass");
            }
            fn allocate_dynamic(&mut self, _: usize) -> rhapsody::Result<StagingAlloc> {
                Err(self.0.take().unwrap())
            }
        }
        for error in [
            rhapsody::Error::Timeout,
            rhapsody::Error::OutOfHostMemory,
            rhapsody::Error::OutOfDeviceMemory,
            rhapsody::Error::DeviceLost,
        ] {
            let discriminant = core::mem::discriminant(&error);
            let actual = rg_buffer_copy_from_slice_u32(
                &mut FailedGraph(Some(error)),
                1,
                4,
                &[7],
                UploadContentVersion::Unknown,
            )
            .unwrap_err();
            assert_eq!(core::mem::discriminant(&actual), discriminant);
        }
    }

    #[test]
    fn unsubmitted_staging_clones_retain_the_resource_until_consumed_or_cancelled() {
        use alloc::sync::Arc;
        let owner = Arc::new(17_u64);
        let weak = Arc::downgrade(&owner);
        let staging = StagingAlloc {
            buffer: 3,
            offset: 0,
            bytes: vec![1, 2, 3, 4],
            lease: Some(ResourceLease::new(owner.clone())),
        };
        assert_eq!(
            staging.lease.as_ref().unwrap().downcast_ref::<u64>(),
            Some(&17)
        );
        let retained = staging.clone();
        let packet = StagingCopy {
            source: staging,
            target: 7,
            dst_offset: 0,
        };
        drop(owner);
        assert!(weak.upgrade().is_some());
        drop(packet); // Cancellation drops its lease, but a cloned packet still owns it.
        assert!(weak.upgrade().is_some());
        drop(retained);
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn clear_passes_have_original_names_and_calls() {
        let mut g = Rec::default();
        clear_rg_buffer(&mut g, 5);
        clear_rg_color_image(&mut g, 9);
        let mut api = Api::default();
        for (n, r, a, f) in g.passes {
            assert_eq!(a, 0x34);
            f(&mut api);
            assert!(matches!((n, r), ("Clear buffer", 5) | ("Clear image", 9)));
        }
        assert_eq!(api.0, ["fill 5 None 0", "clear 9"]);
    }

    #[test]
    fn copy_stages_data_then_copies() {
        let mut g = Rec::default();
        rg_buffer_copy_from_slice_u32(&mut g, 7, 16, &[1, 2, 3], UploadContentVersion::Unknown)
            .unwrap();
        let (name, copies) = g.copies.pop().unwrap();
        assert_eq!(name, "Buffer memcpy");
        assert_eq!(copies.len(), 1);
        assert_eq!(
            (
                copies[0].target,
                copies[0].dst_offset,
                copies[0].source.offset
            ),
            (7, 0, 256)
        );
        assert_eq!(
            copies[0].source.bytes,
            [
                1_u32.to_ne_bytes(),
                2_u32.to_ne_bytes(),
                3_u32.to_ne_bytes()
            ]
            .concat()
        );
        assert!(g.passes.is_empty());
    }

    #[test]
    fn copy_rejects_oversized_data() {
        let error = rg_buffer_copy_from_slice_u32(
            &mut Rec::default(),
            7,
            8,
            &[1, 2, 3],
            UploadContentVersion::Unknown,
        )
        .unwrap_err();
        assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
    }
}
