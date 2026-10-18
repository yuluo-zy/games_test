//! `reset_indirect_args.rs` — 每帧重置四个 indirect-args 缓冲（`reset_indirect_args` 0x14090a870，3044 B；
//! 闭包 `call_once` 0x14169a830，1842 B）。**不是 `ComputePass`**，是一个纯传输 pass。
//!
//! 机器码（O）：
//! 1. world 资源 `Mutex<DynamicConstants>`（TypeId `0x47c94cb78283a72e / 0x8c173ded97323e9d`）、`AssetSsboLibrary`
//!    （`0x8226cc786e679355 / 0x65cfda6a55a841ac`），缺失 → `Option::unwrap`（两个 `unwrap_failed` 站点）。
//! 2. 依次 `get_rg_buffer<T>`：`DispatchIndirectBuffer` → `DrawIndexedIndirectBuffer` → `DrawIndirectBuffer` → `AtomicCounterBuffer`
//!    （顺序即机器码调用顺序；各取返回句柄的资源号）。
//! 3. 锁 `Mutex<DynamicConstants>`（`parking_lot::RawMutex`），按顺序 `DynamicConstants::allocate(frame, size, ..)` 四块暂存区并填充：
//!    * 0x80 B：16 个 u64，交替写 `{u32 0, u32 1}`、`{u32 1, u32 0}`（0x142aaa1e0/e8）= 8 × `[0,1,1,0]`（`VkDispatchIndirectCommand` + 填充）；
//!    * 0x1e0 B：**`memcpy` 自静态表 0x142aaaa78**（0x1e0 B；`DrawIndexedIndirectCommand` × 24，每条 20 B，只有 `index_count`
//!      非零，其余四字段全 0；长度校验 `lStack_150 == 0x1e0`，0x14090a870 反编译，EXE 字节核对）。**纠错**：此前（B_c3）判为
//!      "无任何写入"是错的——反编译里那一处 `memcpy` 被漏看了；`render-gap/DIGEST.md §3.2`；
//!    * 0x10 B：`{0,1,0,0}`（`VkDrawIndirectCommand`：顶点数 0、实例数 1）；
//!    * 8 B：`0x0009_0000_0000_0000`（u32 对 `[0, 0x90000]`）。
//!    每次 `allocate` 返回切片长度若不等于请求长度 → `copy_from_slice::len_mismatch_fail`（长度不符 panic）。
//!    解锁。
//! 4. `PassBuilder::new("Reset indirect args")`（名长 0x13）：对四个资源依次 `add_resource_access(id, 0x34, 0)`
//!    （顺序同第 2 步），然后 `render`：闭包按同一顺序做 4 次 `copy_buffer(暂存区 → 缓冲, 1 个区间)`，
//!    源偏移 = 该暂存区偏移，目的偏移 0（`RenderPassApi::copy_buffer`，区间数栈参数 = 1）。
//!
//! 本文件的接缝：`RenderGraph::{allocate_dynamic}` 与 `RenderPassApi::copy_buffer` 复用 `render::rg_helpers::rg_boundary`；
//! 多资源访问的 pass 登记由 [`IndirectResetGraph`] 补一个方法（`rg_boundary` 的 `add_pass` 只收单个资源）。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::doc_lazy_continuation, clippy::type_complexity)]
use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
use crate::render::rg_helpers::rg_boundary::{RenderGraph, StagingAlloc, StagingCopy};
use crate::rhapsody_types::RenderGraph as ImportGraph;
use crate::systems::render_loop::indirect_params::{
    AtomicCounterBuffer, DispatchIndirectBuffer, DrawIndexedIndirectBuffer, DrawIndirectBuffer,
};

pub const PASS_NAME: &str = "Reset indirect args";
/// 四块暂存区大小（O：`allocate` 的 size 实参，调用顺序）。
pub const STAGING_SIZES: [usize; 4] = [0x80, 0x1e0, 0x10, 8];
/// `{u32 0, u32 1}`（0x142aaa1e0）与 `{u32 1, u32 0}`（0x142aaa1e8）。
pub const DISPATCH_ROW: [u32; 4] = [0, 1, 1, 0];
pub const DRAW_COMMAND: [u32; 4] = [0, 1, 0, 0];
/// 原子计数缓冲：8 B = 一个 u64 `0x0009_0000_0000_0000`。
pub const ATOMIC_INIT: u64 = 0x0009_0000_0000_0000;
/// 0x1e0 B 暂存区的静态表 0x142aaaa78：24 条 `DrawIndexedIndirectCommand`（20 B）的 `index_count`
/// （O，EXE 字节；`instance_count`/`first_index`/`vertex_offset`/`first_instance` 全 0）。
/// 数值是各预置网格的索引数（语义 I，未逐项对应到网格）。
pub const DRAW_INDEXED_INDEX_COUNTS: [u32; 24] = [
    27, 300, 96, 57, 36, 6, 60, 6, 48, 174, 0, 0, 2817, 4716, 66, 180, 15, 900, 18, 900, 18, 18,
    18, 18,
];

/// 渲染图接缝：在 [`RenderGraph`] 之上补“多资源访问登记”与“库缓冲导入用的渲染图”。
pub trait IndirectResetGraph: RenderGraph {
    /// `get_rg_buffer<T>` 的第二参数（`recorder` 内的渲染图）。
    fn import_graph(&mut self) -> &mut ImportGraph;
}

/// 0x80 B 的 dispatch 缓冲初值：8 × `[0,1,1,0]`。
pub fn dispatch_init() -> Vec<u8> {
    (0..8)
        .flat_map(|_| DISPATCH_ROW)
        .flat_map(u32::to_le_bytes)
        .collect()
}

/// 0x1e0 B 的 draw-indexed 缓冲初值：`DRAW_INDEXED_INDEX_COUNTS` 每项展开成 20 B 命令
/// `{index_count, 0, 0, 0, 0}`（O：0x142aaaa78 的 memcpy 源）。
pub fn draw_indexed_init() -> Vec<u8> {
    DRAW_INDEXED_INDEX_COUNTS
        .into_iter()
        .flat_map(|index_count| [index_count, 0, 0, 0, 0])
        .flat_map(u32::to_le_bytes)
        .collect()
}

/// 0x10 B 的 draw 缓冲初值。
pub fn draw_init() -> Vec<u8> {
    DRAW_COMMAND
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect()
}

/// 8 B 的原子计数初值。
pub fn atomic_init() -> Vec<u8> {
    ATOMIC_INIT.to_le_bytes().to_vec()
}

/// 把 `init` 拷进 `staging.bytes`（O：`copy_from_slice`，长度不符 → panic）。
fn fill(staging: &mut StagingAlloc, init: &[u8]) -> rhapsody::Result<()> {
    if staging.bytes.len() != init.len() {
        return Err(rhapsody::Error::invalid_usage(
            "indirect staging allocation has the wrong length",
        ));
    }
    staging.bytes.copy_from_slice(init);
    Ok(())
}

/// W：每帧把四个 indirect-args 缓冲重置为初值。
pub fn reset_indirect_args<G: IndirectResetGraph>(
    graph: &mut G,
    ssbos: &AssetSsboLibrary,
) -> rhapsody::Result<()> {
    let dispatch = ssbos
        .get_rg_buffer::<DispatchIndirectBuffer>(graph.import_graph())
        .resource as u32;
    let draw_indexed = ssbos
        .get_rg_buffer::<DrawIndexedIndirectBuffer>(graph.import_graph())
        .resource as u32;
    let draw = ssbos
        .get_rg_buffer::<DrawIndirectBuffer>(graph.import_graph())
        .resource as u32;
    let atomic = ssbos
        .get_rg_buffer::<AtomicCounterBuffer>(graph.import_graph())
        .resource as u32;

    let mut s0 = graph.allocate_dynamic(STAGING_SIZES[0])?;
    fill(&mut s0, &dispatch_init())?;
    let mut s1 = graph.allocate_dynamic(STAGING_SIZES[1])?;
    fill(&mut s1, &draw_indexed_init())?;
    let mut s2 = graph.allocate_dynamic(STAGING_SIZES[2])?;
    fill(&mut s2, &draw_init())?;
    let mut s3 = graph.allocate_dynamic(STAGING_SIZES[3])?;
    fill(&mut s3, &atomic_init())?;

    graph.copy_from_staging(
        PASS_NAME,
        vec![
            StagingCopy {
                source: s0,
                target: dispatch,
                dst_offset: 0,
            },
            StagingCopy {
                source: s1,
                target: draw_indexed,
                dst_offset: 0,
            },
            StagingCopy {
                source: s2,
                target: draw,
                dst_offset: 0,
            },
            StagingCopy {
                source: s3,
                target: atomic,
                dst_offset: 0,
            },
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
    use crate::render::rg_helpers::rg_boundary::RenderFn;
    use crate::render::test_device::TestDevice;

    #[derive(Default)]
    struct Graph {
        import: ImportGraph,
        cursor: u32,
        passes: Vec<(&'static str, Vec<StagingCopy>)>,
        wrong_len: Option<usize>,
        failure: Option<(usize, rhapsody::Error)>,
        allocations: usize,
    }
    impl RenderGraph for Graph {
        fn add_pass(&mut self, _: &'static str, _: u32, _: u8, _: RenderFn) {
            unreachable!("single-resource add_pass is not used by reset_indirect_args")
        }
        fn copy_from_staging(
            &mut self,
            name: &'static str,
            copies: Vec<StagingCopy>,
        ) -> rhapsody::Result<()> {
            self.passes.push((name, copies));
            Ok(())
        }
        fn allocate_dynamic(&mut self, size: usize) -> rhapsody::Result<StagingAlloc> {
            self.allocations += 1;
            if self
                .failure
                .as_ref()
                .is_some_and(|(at, _)| *at == self.allocations)
            {
                return Err(self.failure.take().unwrap().1);
            }
            let offset = self.cursor;
            self.cursor += size as u32;
            let n = self.wrong_len.unwrap_or(size);
            Ok(StagingAlloc {
                buffer: 7,
                offset,
                bytes: vec![0xAB; n],
                lease: None,
            })
        }
    }
    impl IndirectResetGraph for Graph {
        fn import_graph(&mut self) -> &mut ImportGraph {
            &mut self.import
        }
    }

    fn lib() -> AssetSsboLibrary {
        struct DistinctBuffers(core::cell::Cell<u64>);
        use crate::render::ssbo::rhapsody_boundary::{
            BackendError, Buffer, BufferDesc, GraphicsDevice,
        };
        impl GraphicsDevice for DistinctBuffers {
            fn create_buffer(
                &self,
                desc: &BufferDesc,
                name: &str,
                data: Option<&[u8]>,
            ) -> Result<Buffer, BackendError> {
                let mut buffer = GraphicsDevice::create_buffer(&TestDevice(1), desc, name, data)?;
                let id = self.0.get() + 1;
                self.0.set(id);
                buffer.id = id;
                Ok(buffer)
            }
            fn update_buffer_bytes(
                &self,
                buffer: &Buffer,
                data: &[u8],
                ranges: &[core::ops::Range<usize>],
                offset: usize,
            ) -> Result<(), BackendError> {
                GraphicsDevice::update_buffer_bytes(&TestDevice(1), buffer, data, ranges, offset)
            }
        }
        let l = AssetSsboLibrary::new();
        let dev = DistinctBuffers(core::cell::Cell::new(0));
        l.add_new::<DispatchIndirectBuffer>(&dev);
        l.add_new::<DrawIndexedIndirectBuffer>(&dev);
        l.add_new::<DrawIndirectBuffer>(&dev);
        l.add_new::<AtomicCounterBuffer>(&dev);
        l
    }

    #[test]
    fn initial_values_are_the_exe_constants() {
        let d = dispatch_init();
        assert_eq!(d.len(), 0x80);
        for row in d.chunks(16) {
            let w: Vec<u32> = row
                .chunks(4)
                .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
                .collect();
            assert_eq!(w, [0, 1, 1, 0]);
        }
        assert_eq!(
            draw_init(),
            [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            u64::from_le_bytes(atomic_init().try_into().unwrap()),
            0x0009_0000_0000_0000
        );
        assert_eq!(STAGING_SIZES, [0x80, 0x1e0, 0x10, 8]);
    }

    #[test]
    fn draw_indexed_table_matches_the_exe_static_blob() {
        let b = draw_indexed_init();
        assert_eq!(b.len(), 0x1e0);
        assert_eq!(b.len(), 24 * 20);
        for (i, cmd) in b.chunks(20).enumerate() {
            let w: Vec<u32> = cmd
                .chunks(4)
                .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
                .collect();
            assert_eq!(
                w[0], DRAW_INDEXED_INDEX_COUNTS[i],
                "index_count of command {i}"
            );
            assert_eq!(
                &w[1..],
                &[0, 0, 0, 0],
                "only index_count is non-zero (command {i})"
            );
        }
        // 0x142aaaa78 起 0x1e0 B 的 EXE 原值（十六进制 `0x1b,0x12c,0x60,0x39,…`）。
        assert_eq!(&DRAW_INDEXED_INDEX_COUNTS[..4], &[0x1b, 0x12c, 0x60, 0x39]);
        // 第 13 项的 EXE 字节是 `01 0b 00 00`（小端 0xb01 = 2817）；DIGEST 曾误抄成 0x10b。
        assert_eq!(DRAW_INDEXED_INDEX_COUNTS[12], 0xb01);
        assert_eq!(DRAW_INDEXED_INDEX_COUNTS[13], 0x126c);
        assert_eq!(DRAW_INDEXED_INDEX_COUNTS[17], 0x384);
    }

    #[test]
    fn partial_staging_allocation_failure_preserves_the_error_without_declaring_work() {
        for error in [
            rhapsody::Error::Timeout,
            rhapsody::Error::OutOfHostMemory,
            rhapsody::Error::OutOfDeviceMemory,
            rhapsody::Error::DeviceLost,
        ] {
            let expected = core::mem::discriminant(&error);
            let mut graph = Graph {
                failure: Some((2, error)),
                ..Default::default()
            };
            let error = reset_indirect_args(&mut graph, &lib()).unwrap_err();
            assert_eq!(core::mem::discriminant(&error), expected);
            assert_eq!(graph.allocations, 2);
            assert!(graph.passes.is_empty());
        }
    }

    #[test]
    fn registers_one_pass_with_four_transfer_writes_in_call_order() {
        let (mut g, l) = (Graph::default(), lib());
        reset_indirect_args(&mut g, &l).unwrap();
        assert_eq!(g.passes.len(), 1);
        let (name, copies) = &g.passes[0];
        assert_eq!(*name, PASS_NAME);
        assert_eq!(copies.len(), 4);
        assert!(copies.iter().all(|copy| copy.dst_offset == 0));
        // 库导入顺序 = get_rg_buffer 调用顺序：Dispatch, DrawIndexed, Draw, Atomic
        assert_eq!(g.import.imports.len(), 4);
    }

    #[test]
    fn closure_copies_each_staging_into_its_buffer_in_order() {
        let (mut g, l) = (Graph::default(), lib());
        reset_indirect_args(&mut g, &l).unwrap();
        let (_, copies) = g.passes.pop().unwrap();
        assert_eq!(copies.len(), 4);
        assert_eq!(
            copies
                .iter()
                .map(|copy| copy.source.bytes.len())
                .collect::<Vec<_>>(),
            STAGING_SIZES
        );
        assert_eq!(
            copies
                .iter()
                .map(|copy| copy.source.offset)
                .collect::<Vec<_>>(),
            [0, 0x80, 0x80 + 0x1e0, 0x80 + 0x1e0 + 0x10]
        );
        assert_eq!(copies[0].source.bytes, dispatch_init());
        assert_eq!(copies[1].source.bytes, draw_indexed_init());
        assert_eq!(copies[2].source.bytes, draw_init());
        assert_eq!(copies[3].source.bytes, atomic_init());
        assert_eq!(
            copies.iter().map(|copy| copy.target).collect::<Vec<_>>(),
            [1, 2, 3, 4]
        );
    }

    #[test]
    fn staging_length_mismatch_is_rejected_without_a_copy_declaration() {
        let (mut g, l) = (
            Graph {
                wrong_len: Some(8),
                ..Default::default()
            },
            lib(),
        );
        let error = reset_indirect_args(&mut g, &l).unwrap_err();
        assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
        assert!(g.passes.is_empty());
    }

    #[test]
    #[should_panic(expected = "is not available in the library")]
    fn missing_ssbo_registration_panics() {
        reset_indirect_args(&mut Graph::default(), &AssetSsboLibrary::new()).unwrap();
    }
}
