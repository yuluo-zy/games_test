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
    use core::ops::Range;

    /// `add_resource_access` 的访问码（O：立即数 0x34；位含义 U，按“传输写”使用）。
    pub const ACCESS_TRANSFER_WRITE: u8 = 0x34;

    /// 在 pass 内执行的命令接口（`RenderPassApi`）。
    pub trait RenderPassApi {
        /// 闭包 0x140904990：`fill_buffer(buffer, range=None, value=0)`。
        fn fill_buffer(&mut self, buffer: u32, range: Option<Range<u64>>, value: u32);
        /// 闭包 0x140904a30：`clear_color_image(image)`（无颜色参数）。
        fn clear_color_image(&mut self, image: u32);
        /// 闭包 0x14169a650：`copy_buffer(src, dst, regions)`，`regions` 恰一项（O：栈参数 =1）。
        fn copy_buffer(&mut self, src: &StagingAlloc, dst: u32, regions: &[CopyRegion]);
    }

    /// 动态常量（逐帧环形上传缓冲）中的一次分配。
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct StagingAlloc {
        pub buffer: u64,
        pub offset: u32,
        pub bytes: Vec<u8>,
    }

    /// `copy_buffer` 的区间：闭包里 `[+0x2c]` 读到的 u32 作源偏移，目的偏移 0（O）。
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct CopyRegion {
        pub src_offset: u64,
        pub dst_offset: u64,
    }

    /// `Send`：rhapsody 的回调可能在并行录制线程上执行；这些闭包只捕获资源号/数值。
    pub type RenderFn = Box<dyn FnOnce(&mut dyn RenderPassApi) + Send>;

    /// 渲染图上下文（`RenderContext`，pass 登记表在 +0x90 的 Vec 里，每项 0x40 B）。
    pub trait RenderGraph {
        /// `PassBuilder::new(name)` + `add_resource_access(id, access, 0)` + 登记回调。
        fn add_pass(&mut self, name: &'static str, resource: u32, access: u8, render: RenderFn);
        /// `DynamicConstants::allocate(size)`；失败（无该资源）原版 `Option::unwrap` panic。
        fn allocate_dynamic(&mut self, size: usize) -> StagingAlloc;
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
) {
    let bytes = data.len() * 4;
    assert!(
        bytes as u64 <= buffer_size,
        "assertion failed: data.len() <= buffer.desc().size"
    );
    let mut staging = graph.allocate_dynamic(bytes);
    assert_eq!(staging.bytes.len(), bytes);
    for (dst, v) in staging.bytes.chunks_exact_mut(4).zip(data) {
        dst.copy_from_slice(&v.to_ne_bytes());
    }
    let src_offset = staging.offset as u64;
    graph.add_pass(
        "Buffer memcpy",
        buffer,
        ACCESS_TRANSFER_WRITE,
        Box::new(move |api| {
            api.copy_buffer(
                &staging,
                buffer,
                &[CopyRegion {
                    src_offset,
                    dst_offset: 0,
                }],
            )
        }),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Rec {
        passes: Vec<(&'static str, u32, u8, RenderFn)>,
    }
    impl RenderGraph for Rec {
        fn add_pass(&mut self, name: &'static str, r: u32, a: u8, f: RenderFn) {
            self.passes.push((name, r, a, f));
        }
        fn allocate_dynamic(&mut self, size: usize) -> StagingAlloc {
            StagingAlloc {
                buffer: 3,
                offset: 256,
                bytes: vec![0; size],
            }
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
        fn copy_buffer(&mut self, s: &StagingAlloc, d: u32, r: &[CopyRegion]) {
            self.0.push(format!("copy {:?}->{d} {r:?}", s.bytes));
        }
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
        rg_buffer_copy_from_slice_u32(&mut g, 7, 16, &[1, 2, 3]);
        let (n, r, _, f) = g.passes.pop().unwrap();
        assert_eq!((n, r), ("Buffer memcpy", 7));
        let mut api = Api::default();
        f(&mut api);
        assert!(api.0[0].contains("->7") && api.0[0].contains("src_offset: 256"));
        assert!(api.0[0].starts_with("copy [1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]"));
    }

    #[test]
    #[should_panic(expected = "assertion failed: data.len() <= buffer.desc().size")]
    fn copy_rejects_oversized_data() {
        rg_buffer_copy_from_slice_u32(&mut Rec::default(), 7, 8, &[1, 2, 3]);
    }
}
