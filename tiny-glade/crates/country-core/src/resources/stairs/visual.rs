//! `visual.rs` — 楼梯可视化资源（B `resources/stairs/visual.rs`，4 业务 + 6 胶水）。
//!
//! 章节：`chapters/resources/stairs/visual.rs.md`（O）。
//! - `MagicSupports`（`MagicSupports::clear` 0x140aa8210，render 资源：两个 mesh-atlas
//!   Arc/写锁 + 计数；`clear` = Arc downgrade → `SparseInstanceBufferWriter::clear
//!   <BrickTransformSsbo>` + 计数归零，全属 render/rt 胶水，登记）；
//! - `StairsVisualState::remove_edge`（0x140aa8620）：删除指定边的视觉状态
//!   （invalidate 系统经 `contains_edge_or_stub` 判定后调用）；
//! - `StairsVisualState::reset_edge`（0x140aa8a60）：边视觉重置（v1 force_rebuild 名错）。
//!
//! **登记待接线（胶水）**：`remove_edge`/`reset_edge` 的 RtWorld RtMesh 删除、
//! mesh atlas 句柄释放与 DirtyCow 交互属 render/rt 域（库内未建 `resources/rt`），
//! 本文件只实现边视觉存储的增删核心 + 脏字节（+0x30）。
use crate::render::sparse_instance_buffer::{
    MeshAtlasAllocation, SparseInstanceBuffer, SparseSsbo,
};
use crate::resources::stairs::stairs_state::StairGraphEdge;
use bevy_ecs::{component::Component, resource::Resource};
use hashbrown::HashMap;

/// `MagicSupports`（B `visual.rs`；render 资源）。
///
/// 机器型（fresh 0x140aa8210 逐指令）：`{ +0x10 计数 u64, +0x18 计数 u32,
/// +0x20/+0x28 第一 mesh-atlas 写锁槽, +0x30/+0x38 第二槽, +0x40 标志 u8 }`。
/// 每槽 = `{ Weak<SparseInstanceBuffer::waste> @+0x00, Box<Allocation> @+0x08 }`
/// （`MeshAtlasAllocation`）：槽空 → `Arc::downgrade(*(buffer+0x60))` + 新 0x20 B
/// `Allocation`。
///
/// `clear` 0x140aa8210（参数：`&mut self, first: &mut SparseInstanceBuffer<BrickTransformSsbo>,
/// second: &mut SparseInstanceBuffer<BrickTransformSsbo>`）：
/// 1. 第一槽懒初始化 → LOCK 分配 `+0x18` → 以缓冲 `+0x20..+0x58`（byte_map/active/data/
///    dirty）与分配指针拼 writer → `SparseInstanceBufferWriter::clear<BrickTransformSsbo>`；
/// 2. 第二槽同样；
/// 3. `+0x10`/`+0x18`/`+0x40` 归零；两把 RawMutex 逆序解锁。
///
/// 原版 `T` 固定为 `geometry::instanced_wall::BrickTransformSsbo`（本 crate 未落库该元素），
/// 故 `clear` 以泛型 `T: SparseSsbo` 承接同一控制流；注册时由 roof/geometry 域给出具体 `T`。
/// **地址判定（fresh 更正 LOGIC §5）**：0x140869a30 的 Ghidra 符号与函数体均为
/// `structural_graph::StairsStructuralGraph::clear`（8 张 hashbrown 表 + 1 Vec），
/// 不是 MagicSupports::clear；本函数唯一地址是 0x140aa8210。
/// 旧实现 `HashSet<StairGraphEdge>` 与机器不同构（机器无边集合），已改 render 资源占位；
/// insert/remove/contains 等边集合方法系旧实现编造，删除（库内无消费面）。
#[derive(Resource, Debug, Default)]
pub struct MagicSupports {
    /// +0x10：条目计数（机器 clear 归零）。
    count: u64,
    /// +0x18：计数（机器 clear 归零）。
    count32: u32,
    /// +0x20/+0x28：第一 mesh-atlas 写面分配槽。
    first: Option<MeshAtlasAllocation>,
    /// +0x30/+0x38：第二 mesh-atlas 写面分配槽。
    second: Option<MeshAtlasAllocation>,
    /// +0x40：标志（机器 clear 归零）。
    flag: u8,
}

impl MagicSupports {
    /// `MagicSupports::clear`（0x140aa8210）：两槽懒初始化 → 两把分配锁 →
    /// `SparseInstanceBufferWriter::clear<T>` ×2 → 计数归零。
    pub fn clear<T: SparseSsbo>(
        &mut self,
        first: &mut SparseInstanceBuffer<T>,
        second: &mut SparseInstanceBuffer<T>,
    ) {
        {
            let slot = self
                .first
                .get_or_insert_with(|| MeshAtlasAllocation::new(first));
            let mut writer = first.writer_for(slot.allocation());
            writer.clear();
        }
        {
            let slot = self
                .second
                .get_or_insert_with(|| MeshAtlasAllocation::new(second));
            let mut writer = second.writer_for(slot.allocation());
            writer.clear();
        }
        self.count = 0;
        self.count32 = 0;
        self.flag = 0;
    }
}

/// 边视觉条目（RtWorld/mesh 句柄域；字段语义 U，待 render 域接线）。
///
/// 除占位外，保留两个 mesh-atlas 写面分配槽：`reset_edge` 先懒初始化它们
/// （`Arc::downgrade(host+0x60)` + 新 Allocation），再各 `clear<BrickTransformSsbo>`
/// 一次。其余 0x110+ 字节缓存/实体表字段仍 U。
#[derive(Debug, Default)]
pub struct StairsEdgeVisual {
    /// 原value+0x100：强制重建请求；与全局dirty和实例分配分别拥有状态。
    force_rebuild_requested: bool,
    /// 第一写面分配槽（fresh 0x140aa8a60 `pauVar33[-3]` 槽）。
    first: Option<MeshAtlasAllocation>,
    /// 第二写面分配槽（`pauVar33[-2]` 槽）。
    second: Option<MeshAtlasAllocation>,
    /// 占位——其余实际字段（RtMesh 句柄等，U）。
    pub(crate) _opaque: (),
}

impl StairsEdgeVisual {
    /// 原interpolate读取value+0x100；true时不能复用旧sample/curve。
    pub fn force_rebuild_requested(&self) -> bool {
        self.force_rebuild_requested
    }
}

/// `StairsVisualState`（边 → 视觉状态 + 脏字节）。
#[derive(Resource, Debug, Default)]
pub struct StairsVisualState {
    /// +0x30：脏字节（O：remove_edge 处理末置 1）。
    dirty: bool,
    /// 边视觉状态表。
    edges: HashMap<StairGraphEdge, StairsEdgeVisual>,
}

impl StairsVisualState {
    /// `StairsVisualState::remove_edge`（0x140aa8620，O 存储核心）：删除指定边的视觉状态
    /// （RtWorld/mesh 撤除胶水登记），并置脏字节。
    pub fn remove_edge(&mut self, edge: StairGraphEdge) {
        self.edges.remove(&edge);
        self.dirty = true;
    }

    /// `StairsVisualState::reset_edge`（0x140aa8a60，PDB完整2,322 B）：边视觉重建。
    ///
    /// fresh 反编译主体 = **render/rt 写面**：
    /// 1. `hash_one<StairGraphEdge>` 探 edges 表（bucket stride **0x110**）；
    ///    未命中 → `RawTable::reserve_rehash<tuple<StairGraphEdge, StairsEdgeVisualState>>`；
    /// 2. 新条目的第一写面槽懒初始化（`Arc::downgrade(host+0x60)` + 新 `Allocation`）→
    ///    LOCK 分配 `+0x18` → 以 host `+0x20..+0x58` 拼 writer →
    ///    `SparseInstanceBufferWriter::clear<BrickTransformSsbo>`；
    /// 3. `MagicSupports::clear()`（同样清两路写面）；
    /// 4. 第二写面槽懒初始化 → clear<BrickTransformSsbo>；
    /// 5. 实体 despawn 循环（`Vec<Entity>`，高 dword != 0）+ 清内部表/向量；
    ///    以值返回重建后的 `StairsEdgeVisualState`（0x110+ 字节，字段 U）。
    ///
    /// 库内保留**存储核心 upsert**（`edges.insert(edge, default)`）。渲染写面见
    /// [`Self::reset_edge_with`]（需要两个 `SparseInstanceBuffer<T>` 宿主）。
    /// `dirty` 置位在 remove_edge 有机器码证据（+0x30），本函数 fresh 体未见该写入——
    /// 保留作消费侧脏标记（U）。
    pub fn reset_edge(&mut self, edge: StairGraphEdge) {
        self.edges.insert(edge, StairsEdgeVisual::default());
        self.dirty = true;
    }

    /// `reset_edge` 的**完整渲染写面**（0x140aa8a60 步骤 2–4）。原版 `T` =
    /// `geometry::instanced_wall::BrickTransformSsbo`（该元素本 crate 未落库，故泛型承接）：
    /// 第一槽懒初始化 → `clear<T>`；`MagicSupports::clear`（两路）→ 第二槽懒初始化 →
    /// `clear<T>`。`MagicSupports` 与两个宿主由调用方从 render 资源给出。
    ///
    /// 未建模：步骤 5 的实体 despawn 循环与 0x110 B 缓存字段（render host，登记）。
    pub fn reset_edge_with<T: SparseSsbo>(
        &mut self,
        edge: StairGraphEdge,
        first: &mut SparseInstanceBuffer<T>,
        second: &mut SparseInstanceBuffer<T>,
        magic: &mut MagicSupports,
    ) {
        let state = self.edges.entry(edge).or_default();
        {
            let slot = state
                .first
                .get_or_insert_with(|| MeshAtlasAllocation::new(first));
            let mut writer = first.writer_for(slot.allocation());
            writer.clear();
        }
        magic.clear(first, second);
        {
            let slot = state
                .second
                .get_or_insert_with(|| MeshAtlasAllocation::new(second));
            let mut writer = second.writer_for(slot.allocation());
            writer.clear();
        }
        // 原140aa8e26/140aa9067清value+0x100；未知的相邻byte不另造业务字段。
        state.force_rebuild_requested = false;
        // 步骤 5（实体 despawn + 缓存重建）属 render host；返回值的 dirty 置位为 U。
        self.dirty = true;
    }

    /// 原force_rebuild内联Line134：只标记已存在边，缺失不创建，也不清理实例。
    pub fn force_rebuild(&mut self, edge: StairGraphEdge) {
        if let Some(state) = self.edges.get_mut(&edge) {
            state.force_rebuild_requested = true;
        }
    }

    /// 兼容现宿主形参；原force只写请求byte，不接触这三个渲染owner。
    pub fn force_rebuild_with<T: SparseSsbo>(
        &mut self,
        edge: StairGraphEdge,
        first: &mut SparseInstanceBuffer<T>,
        second: &mut SparseInstanceBuffer<T>,
        magic: &mut MagicSupports,
    ) {
        let _ = (first, second, magic);
        self.force_rebuild(edge);
    }

    pub fn get(&self, edge: &StairGraphEdge) -> Option<&StairsEdgeVisual> {
        self.edges.get(edge)
    }
    pub fn contains(&self, edge: &StairGraphEdge) -> bool {
        self.edges.contains_key(edge)
    }
    pub fn edges(&self) -> &HashMap<StairGraphEdge, StairsEdgeVisual> {
        &self.edges
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }
}

/// `StairsVisualComponent`（B `impl$6::register_required_components` 注册胶水的组件面）。
#[derive(Component, Clone, Debug, Default)]
pub struct StairsVisualComponent {
    pub(crate) _marker: (),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::sparse_instance_buffer::SLOTS_PER_CHUNK;
    use crate::render::ssbo::{
        StorageElement,
        rhapsody_boundary::{BackendError, Buffer, BufferDesc},
    };
    use core::ops::Range;

    // `SparseSsbo` 的 CPU 影子按 `T::default()` 填充（2026-10-17 impl-brick fresh：
    // `new_storage_buffer<BrickTransformSsbo>` 0x1412e5800 的元素初值 = Default 模式）。
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    struct Elem(u32);
    impl StorageElement for Elem {
        const NAME: &'static str = "MagicSupportsTestElem";
        const BUFFER_LEN: usize = 64 * SLOTS_PER_CHUNK;
    }
    unsafe impl SparseSsbo for Elem {
        const CHUNK_COUNT: usize = 64;
    }

    #[derive(Default)]
    struct Dev;
    impl crate::render::ssbo::rhapsody_boundary::GraphicsDevice for Dev {
        fn create_buffer(
            &self,
            desc: &BufferDesc,
            _name: &str,
            _data: Option<&[u8]>,
        ) -> Result<Buffer, BackendError> {
            Ok(Buffer {
                byte_size: desc.size as u64,
                ..Buffer::default()
            })
        }
        fn update_buffer_bytes(
            &self,
            _buffer: &Buffer,
            _data: &[u8],
            _ranges: &[Range<usize>],
            _offset: usize,
        ) -> Result<(), BackendError> {
            Ok(())
        }
    }

    fn buffers() -> (SparseInstanceBuffer<Elem>, SparseInstanceBuffer<Elem>) {
        let dev = Dev;
        (
            SparseInstanceBuffer::new_storage_buffer(&dev, None),
            SparseInstanceBuffer::new_storage_buffer(&dev, None),
        )
    }

    #[test]
    fn magic_supports_clear_lazily_creates_and_clears_both_writers() {
        let (mut first, mut second) = buffers();
        let mut supports = MagicSupports::default();
        supports.count = 5;
        supports.count32 = 6;
        supports.flag = 7;
        // 第一/第二槽已存在：各自先写一个元素，clear 后应回到空闲栈。
        supports.first = Some(MeshAtlasAllocation::new(&first));
        supports.second = Some(MeshAtlasAllocation::new(&second));
        {
            let mut w = first.writer_for(supports.first.as_ref().unwrap().allocation());
            w.push(&Elem(1)).unwrap();
            w.push(&Elem(2)).unwrap();
        }
        {
            let mut w = second.writer_for(supports.second.as_ref().unwrap().allocation());
            w.push(&Elem(3)).unwrap();
        }
        supports.clear(&mut first, &mut second);
        assert_eq!(supports.count, 0);
        assert_eq!(supports.count32, 0);
        assert_eq!(supports.flag, 0);
        assert!(first.byte_map[..64].iter().all(|&x| x == 0));
        assert!(second.byte_map[..64].iter().all(|&x| x == 0));
        assert_eq!(first.free_chunks.read().len(), 64);
        assert_eq!(second.free_chunks.read().len(), 64);
        // 再次 clear：槽已存在，不做第二次懒初始化（与机器 +0x20 != 0 分支一致）。
        let first_ptr = supports.first.as_ref().unwrap().allocation() as *const _;
        supports.clear(&mut first, &mut second);
        assert_eq!(
            supports.first.as_ref().unwrap().allocation() as *const _,
            first_ptr
        );
    }

    #[test]
    fn magic_supports_is_a_render_resource_clear_zeroes_the_counters() {
        let (mut first, mut second) = buffers();
        let mut supports = MagicSupports::default();
        supports.clear(&mut first, &mut second);
        assert_eq!(supports.count, 0);
        assert_eq!(supports.count32, 0);
        // 懒初始化产生两个空分配槽。
        assert!(supports.first.is_some());
        assert!(supports.second.is_some());
    }

    #[test]
    fn visual_state_remove_and_reset() {
        let mut state = StairsVisualState::default();
        let edge = StairGraphEdge::new(1, 2);
        state.reset_edge(edge);
        assert!(state.contains(&edge));
        assert!(state.is_dirty());
        state.clear_dirty();
        state.remove_edge(edge);
        assert!(!state.contains(&edge));
        assert!(state.is_dirty());
    }

    #[test]
    fn reset_edge_with_clears_both_detached_allocations() {
        let (mut first, mut second) = buffers();
        let mut magic = MagicSupports::default();
        let mut state = StairsVisualState::default();
        let edge = StairGraphEdge::new(5, 9);
        state.reset_edge_with(edge, &mut first, &mut second, &mut magic);
        assert!(state.contains(&edge));
        // 由每边槽各写入一个元素，随后 reset 必须用同一分配并 clear。
        {
            let item = state.edges.get_mut(&edge).unwrap();
            {
                let mut w = first.writer_for(item.first.as_ref().unwrap().allocation());
                w.push(&Elem(11)).unwrap();
            }
            {
                let mut w = second.writer_for(item.second.as_ref().unwrap().allocation());
                w.push(&Elem(12)).unwrap();
            }
        }
        assert_eq!(first.free_chunks.read().len(), 63);
        state.reset_edge_with(edge, &mut first, &mut second, &mut magic);
        assert!(first.byte_map[..64].iter().all(|&x| x == 0));
        assert!(second.byte_map[..64].iter().all(|&x| x == 0));
        assert_eq!(first.free_chunks.read().len(), 64);
        assert_eq!(second.free_chunks.read().len(), 64);
    }

    #[derive(Debug, PartialEq)]
    struct BufferSnapshot {
        data: Vec<Elem>,
        byte_map: Vec<u8>,
        active: Vec<usize>,
        dirty: Vec<usize>,
        free: Vec<u32>,
        waste_len: usize,
    }

    fn snapshot(buffer: &SparseInstanceBuffer<Elem>) -> BufferSnapshot {
        use core::sync::atomic::Ordering;
        BufferSnapshot {
            data: buffer.data.to_vec(),
            byte_map: buffer.byte_map.to_vec(),
            active: buffer.active_words().collect(),
            dirty: buffer
                .dirty_bitset
                .iter()
                .map(|v| v.load(Ordering::Relaxed))
                .collect(),
            free: buffer.free_chunks.read().clone(),
            waste_len: buffer.waste.lock().len(),
        }
    }

    #[test]
    fn force_rebuild_preserves_actual_allocations_and_written_shadow_for_both_dirty_states() {
        for initial_dirty in [false, true] {
            let (mut first, mut second) = buffers();
            let mut magic = MagicSupports::default();
            let mut state = StairsVisualState::default();
            let edge = StairGraphEdge::new(7, 11);
            state.reset_edge_with(edge, &mut first, &mut second, &mut magic);
            let item = state.get(&edge).unwrap();
            assert!(!item.force_rebuild_requested());
            let first_ptr = item.first.as_ref().unwrap().allocation() as *const _;
            let second_ptr = item.second.as_ref().unwrap().allocation() as *const _;
            first
                .writer_for(item.first.as_ref().unwrap().allocation())
                .push(&Elem(71))
                .unwrap();
            second
                .writer_for(item.second.as_ref().unwrap().allocation())
                .push(&Elem(72))
                .unwrap();
            let first_before = snapshot(&first);
            let second_before = snapshot(&second);
            let magic_first = magic.first.as_ref().unwrap().allocation() as *const _;
            let magic_second = magic.second.as_ref().unwrap().allocation() as *const _;
            magic.count = 17;
            magic.count32 = 23;
            magic.flag = 9;
            state.dirty = initial_dirty;

            state.force_rebuild(edge);
            state.force_rebuild_with(edge, &mut first, &mut second, &mut magic);
            let item = state.get(&edge).unwrap();
            assert!(item.force_rebuild_requested());
            assert_eq!(
                item.first.as_ref().unwrap().allocation() as *const _,
                first_ptr
            );
            assert_eq!(
                item.second.as_ref().unwrap().allocation() as *const _,
                second_ptr
            );
            assert_eq!(snapshot(&first), first_before);
            assert_eq!(snapshot(&second), second_before);
            assert_eq!(
                magic.first.as_ref().unwrap().allocation() as *const _,
                magic_first
            );
            assert_eq!(
                magic.second.as_ref().unwrap().allocation() as *const _,
                magic_second
            );
            assert_eq!((magic.count, magic.count32, magic.flag), (17, 23, 9));
            assert_eq!(state.is_dirty(), initial_dirty);

            // 真reset清实例，并在同一owner清掉强制重建请求。
            state.reset_edge_with(edge, &mut first, &mut second, &mut magic);
            let item = state.get(&edge).unwrap();
            assert!(!item.force_rebuild_requested());
            assert_eq!(
                item.first.as_ref().unwrap().allocation() as *const _,
                first_ptr
            );
            assert!(first.byte_map[..64].iter().all(|&value| value == 0));
            assert!(second.byte_map[..64].iter().all(|&value| value == 0));
            assert_eq!(first.free_chunks.read().len(), 64);
            assert_eq!(second.free_chunks.read().len(), 64);
            state.force_rebuild(edge);
            state.remove_edge(edge);
            assert!(!state.contains(&edge));
        }
    }

    #[test]
    fn missing_force_request_is_noop_without_lazy_allocations_or_global_dirty_changes() {
        let (mut first, mut second) = buffers();
        let first_before = snapshot(&first);
        let second_before = snapshot(&second);
        let mut magic = MagicSupports::default();
        let mut state = StairsVisualState::default();
        let edge = StairGraphEdge::new(19, 23);
        for initial_dirty in [false, true] {
            state.dirty = initial_dirty;
            state.force_rebuild(edge);
            state.force_rebuild_with(edge, &mut first, &mut second, &mut magic);
            assert!(!state.contains(&edge));
            assert_eq!(state.is_dirty(), initial_dirty);
            assert!(magic.first.is_none());
            assert!(magic.second.is_none());
            assert_eq!(snapshot(&first), first_before);
            assert_eq!(snapshot(&second), second_before);
        }
    }

    #[test]
    fn storage_reset_replaces_the_requested_flag_with_default_zero() {
        let mut state = StairsVisualState::default();
        let edge = StairGraphEdge::new(29, 7);
        state.reset_edge(edge);
        state.force_rebuild(edge);
        assert!(state.get(&edge).unwrap().force_rebuild_requested());
        state.reset_edge(edge);
        assert!(!state.get(&edge).unwrap().force_rebuild_requested());
    }
}
