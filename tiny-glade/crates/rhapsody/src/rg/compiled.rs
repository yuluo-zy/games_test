// 候选实现迁移来源：reconstruction/render/work-packages/compiled-state/lib.rs.
// 这是行为重建，不是原作者源码。
use std::ops::Range;

pub const NO_ACCESS: u32 = u32::MAX;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Batch {
    pub pass_range: Range<usize>,
    pub export_barriers: bool,
}

#[derive(Debug, Default)]
pub struct BatchCompiler {
    pub batches: Vec<Batch>,
    pub current_range: Range<usize>,
    pub current_export_barriers: bool,
}

impl BatchCompiler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn flush_batch(&mut self, boundary: usize) {
        if self.current_range.start == boundary {
            return;
        }
        self.current_range.end = boundary;
        let pass_range = std::mem::take(&mut self.current_range);
        let export_barriers = std::mem::take(&mut self.current_export_barriers);
        self.batches.push(Batch {
            pass_range,
            export_barriers,
        });
        self.current_range.start = boundary;
    }

    pub fn finish(mut self, pass_count: usize) -> Vec<Batch> {
        self.flush_batch(pass_count);
        if let Some(last) = self.batches.last_mut() {
            last.export_barriers = true;
        }
        self.batches
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceLifetime {
    pub first_access_index: u32,
    pub last_access_index: u32,
}

pub fn pending_initial_transition_resources(resources: &[ResourceLifetime]) -> Vec<u32> {
    resources
        .iter()
        .enumerate()
        .filter_map(|(resource_index, lifetime)| {
            (lifetime.first_access_index != NO_ACCESS).then_some(resource_index as u32)
        })
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceAccess {
    pub resource_index: u32,
    pub previous_access_index: u32,
    pub synchronization_group: u32,
    pub access_code: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuntimeResourceState {
    pub has_initial_state: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BarrierTransition {
    pub resource_index: u32,
    pub previous_access_code: u8,
    pub next_access_code: u8,
}

pub fn pre_pass_barrier_transitions(
    pass_access_range: Range<usize>,
    accesses: &[ResourceAccess],
    resources: &[RuntimeResourceState],
    mut can_be_concurrent: impl FnMut(&ResourceAccess, &ResourceAccess) -> bool,
) -> Vec<BarrierTransition> {
    let pass_accesses = &accesses[pass_access_range];
    let mut barriers = Vec::new();
    for current in pass_accesses {
        let resource = resources[current.resource_index as usize];
        let previous_access_code = if current.previous_access_index == NO_ACCESS {
            if !resource.has_initial_state {
                continue;
            }
            0
        } else {
            let previous = &accesses[current.previous_access_index as usize];
            if can_be_concurrent(current, previous) {
                continue;
            }
            previous.access_code
        };
        barriers.push(BarrierTransition {
            resource_index: current.resource_index,
            previous_access_code,
            next_access_code: current.access_code,
        });
    }
    barriers
}

#[derive(Debug)]
pub struct CompiledGraphSnapshot<Graph, Registry, Pass> {
    pub graph: Graph,
    pub recorded_passes: Vec<Pass>,
    pub registry: Registry,
    pub pending_initial_transitions: Vec<u32>,
}

impl<Graph, Registry, Pass> CompiledGraphSnapshot<Graph, Registry, Pass> {
    pub fn new(
        graph: Graph,
        recorded_passes: Vec<Pass>,
        registry: Registry,
        resource_lifetimes: &[ResourceLifetime],
    ) -> Self {
        Self {
            graph,
            recorded_passes,
            registry,
            pending_initial_transitions: pending_initial_transition_resources(resource_lifetimes),
        }
    }

    pub fn take_initial_transitions(&mut self) -> Vec<u32> {
        std::mem::take(&mut self.pending_initial_transitions)
    }
}

/// `CompiledRenderGraph<PassResources>`（`ResourceExportHandle<Texture>` 一类 compile 结果里的 0x218 字节主体）
/// 的已读布局。证据：`CompiledRenderGraph::new` 0x1402c4280、`execute_compiled` 0x1400fa190、
/// `record_batch` 0x1402c1d40，及 rne-read 对 0x140bd6650（`SynchronizationScope1::new`）的手解。
///
/// 同布局的 `repr(C)` 类型见 [`CompiledRenderGraphLayout`]；现实现的 `graph::CompiledGraph` 是按语义建的候选表示，
/// 与这些偏移没有一一对应，强行 `repr(C)` 对齐只会制造假的 ABI。
pub mod original_layout {
    pub const SIZE: usize = 0x218;
    /// `+0x00..0x90`：图记录段，`execute_compiled` 原样拷给 `ExecutedRenderGraph`。
    pub const GRAPH_RECORD: usize = 0x00;
    pub const GRAPH_RECORD_BYTES: usize = 0x90;
    /// `+0x90..0xa8`：`Vec<Pass>`（cap-first：`cap@0x90, ptr@0x98, len@0xa0`；元素 0x40 字节）。
    /// `execute_compiled` 以 `[+0xa0]` 作 pass 数量（0x1400fa190 的 `flush_batch` 循环上界）；
    /// 早先文档写的"5 个 qword 的 Pass 头"与 `+0xa8` 起的 Registry 重叠（0x90+0x28 > 0xa8），按 3 个 qword 读。
    pub const PASS_VEC: usize = 0x90;
    pub const PASS_VEC_BYTES: usize = 0x18;
    pub const PASS_VEC_LEN: usize = 0xa0;
    /// `+0xa8..0x1b0`：`ResourceRegistry`（0x108 字节）。
    pub const RESOURCE_REGISTRY: usize = 0xa8;
    pub const RESOURCE_REGISTRY_BYTES: usize = 0x108;
    /// `+0x1b0..0x200`：`SynchronizationScope`（0x50 字节）。
    pub const SYNCHRONIZATION_SCOPE: usize = 0x1b0;
    pub const SYNCHRONIZATION_SCOPE_BYTES: usize = 0x50;
    /// `+0x200..0x218`：`Vec<u32>` 待发初始过渡资源下标，cap-first `{cap@0x200, ptr@0x208, len@0x210}`。
    pub const PENDING_INITIAL_TRANSITIONS: usize = 0x200;
    /// `GraphExecutionRecorder::execute_compiled` 里对该 Vec 的消费发生在 `record_batch` 的第一批前段。
    pub const PENDING_INITIAL_TRANSITIONS_BYTES: usize = 0x18;

    #[cfg(test)]
    #[test]
    fn layout_segments_tile_the_0x218_bytes() {
        let segments = [
            (GRAPH_RECORD, GRAPH_RECORD_BYTES),
            (PASS_VEC, PASS_VEC_BYTES),
            (RESOURCE_REGISTRY, RESOURCE_REGISTRY_BYTES),
            (SYNCHRONIZATION_SCOPE, SYNCHRONIZATION_SCOPE_BYTES),
            (PENDING_INITIAL_TRANSITIONS, PENDING_INITIAL_TRANSITIONS_BYTES),
        ];
        let mut end = 0;
        for (start, len) in segments {
            assert_eq!(start, end, "segment at {start:#x} must start where the previous ended");
            end = start + len;
        }
        assert_eq!(end, SIZE);
        assert_eq!(PASS_VEC_LEN, PASS_VEC + 0x10);
    }
}

/// `Vec<T>` 在原版 EXE 里的 24 字节排布（cap-first：`{cap, ptr, len}`），只作布局占位。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RawVecLayout {
    pub cap: usize,
    pub ptr: usize,
    pub len: usize,
}

/// `CompiledRenderGraph<PassResources>` 的 0x218 字节 `repr(C)` 排布，段划分与 [`original_layout`] 常量一致
/// （证据：`CompiledRenderGraph::new` 0x1402c4280 的逐段拷贝：`+0x00` 0x90、`+0x90` 0x18、`+0xa8` 0x108、
/// `+0x1b0` 0x50、`+0x200` 0x18）。各段内部字段仍只有"字节块"级证据，所以段体用 `[u8; N]`；
/// 现实现的 `graph::CompiledGraph` 与它没有一一对应，不据此改写。
#[repr(C)]
#[derive(Clone, Debug)]
pub struct CompiledRenderGraphLayout {
    pub graph_record: [u8; original_layout::GRAPH_RECORD_BYTES],
    pub passes: RawVecLayout,
    pub resource_registry: [u8; original_layout::RESOURCE_REGISTRY_BYTES],
    pub synchronization_scope: [u8; original_layout::SYNCHRONIZATION_SCOPE_BYTES],
    pub pending_initial_transitions: RawVecLayout,
}

/// `ResourceExportHandle<T>`：原版 `compile<ResourceExportHandle<Texture>, ..>` 成功值里紧跟在
/// `CompiledRenderGraph` 之后的 4 字节。
///
/// ✅ 布局：`0x1401647b0` / `0x140161cd0` 成功路径都是 `memcpy(out, graph, 0x218); *(u32*)(out + 0x218) = h`，
///    不是 `CompiledRenderGraph` 的包装（旧文档"=0x218B"的说法不对：0x218 是 `Ok` 主体，句柄另占 4 字节，整体 0x220）。
/// ⚠️ 语义：`h` 取自导出前（`Texture::export` 0x140bd6380，访问码 0x22）那份纹理句柄记录的首个 u32
///    （0x1401647b0 行 `uVar23 = *(u32*)slot` 先于 `FUN_140bd6380(slot, ..., 0x22)`），即该纹理在图里的逻辑资源下标；
///    Rust 侧类型名/字段名未读出。
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ResourceExportHandle<T> {
    pub resource_index: u32,
    _marker: std::marker::PhantomData<fn() -> T>,
}
impl<T> ResourceExportHandle<T> {
    pub const fn new(resource_index: u32) -> Self {
        Self { resource_index, _marker: std::marker::PhantomData }
    }
}

/// 导出句柄的资源种类标记（`ResourceExportHandle<Texture>` 里的 `Texture`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TextureKind {}

/// `compile<Export, UD, F>` 返回值（经 sret 指针）的 `Ok` 排布：`{ CompiledRenderGraph(0x218), Export }`。
/// `Export = ()` 的展示图实例（0x140166510）只有 0x218（`memcpy(out, graph, 0x218)`，无尾部写入），
/// `Export = ResourceExportHandle<Texture>` 的加载图/世界图实例是 0x220。
/// `Err` 用 `Result` 的小值优化：只写第一个 qword = [`COMPILE_ERR_NICHE`]（三个实例的失败路径都是
/// `*out = 0x8000000000000000`），错误载荷的字段仍未读出。
#[repr(C)]
#[derive(Clone, Debug)]
pub struct CompileOkLayout<Export> {
    pub graph: CompiledRenderGraphLayout,
    pub export: Export,
}

/// 失败路径写入返回值首 qword 的哨兵（`i64::MIN`）。
pub const COMPILE_ERR_NICHE: u64 = 0x8000_0000_0000_0000;

/// 按原版小值优化判断返回值首 qword 是不是 `Err`。
pub const fn compile_result_is_err(first_qword: u64) -> bool {
    first_qword == COMPILE_ERR_NICHE
}

#[cfg(test)]
mod compile_layout_tests {
    use super::*;
    use std::mem::{offset_of, size_of};

    #[test]
    fn compiled_graph_layout_matches_original_offsets() {
        assert_eq!(size_of::<CompiledRenderGraphLayout>(), original_layout::SIZE);
        assert_eq!(offset_of!(CompiledRenderGraphLayout, graph_record), original_layout::GRAPH_RECORD);
        assert_eq!(offset_of!(CompiledRenderGraphLayout, passes), original_layout::PASS_VEC);
        assert_eq!(
            offset_of!(CompiledRenderGraphLayout, passes) + offset_of!(RawVecLayout, len),
            original_layout::PASS_VEC_LEN
        );
        assert_eq!(offset_of!(CompiledRenderGraphLayout, resource_registry), original_layout::RESOURCE_REGISTRY);
        assert_eq!(offset_of!(CompiledRenderGraphLayout, synchronization_scope), original_layout::SYNCHRONIZATION_SCOPE);
        assert_eq!(
            offset_of!(CompiledRenderGraphLayout, pending_initial_transitions),
            original_layout::PENDING_INITIAL_TRANSITIONS
        );
    }

    #[test]
    fn compile_ok_layout_has_the_export_handle_at_0x218() {
        assert_eq!(size_of::<ResourceExportHandle<TextureKind>>(), 4);
        assert_eq!(offset_of!(CompileOkLayout<ResourceExportHandle<TextureKind>>, export), 0x218);
        assert_eq!(size_of::<CompileOkLayout<ResourceExportHandle<TextureKind>>>(), 0x220);
        assert_eq!(size_of::<CompileOkLayout<()>>(), 0x218);
    }

    #[test]
    fn err_is_the_i64_min_niche() {
        assert!(compile_result_is_err(i64::MIN as u64));
        assert!(!compile_result_is_err(0));
        assert_eq!(ResourceExportHandle::<TextureKind>::new(7).resource_index, 7);
    }
}
