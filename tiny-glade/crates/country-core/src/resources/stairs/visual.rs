//! `visual.rs` — 楼梯可视化资源（B `resources/stairs/visual.rs`，4 业务 + 6 胶水）。
//!
//! 章节：`chapters/resources/stairs/visual.rs.md`（O）。
//! - `MagicSupports`（`MagicSupports::clear` 0x140aa8210，render 资源：两个 mesh-atlas
//!   Arc/写锁、漂浮砖参数Vec与计数；`clear` = Arc downgrade → `SparseInstanceBufferWriter::clear
//!   <BrickTransformSsbo>` + 计数归零，全属 render/rt 胶水，登记）；
//! - `StairsVisualState::remove_edge`（0x140aa8620）：删除指定边的视觉状态
//!   （invalidate 系统经 `contains_edge_or_stub` 判定后调用）；
//! - `StairsVisualState::reset_edge`（0x140aa8a60）：边视觉重置（v1 force_rebuild 名错）。
//!
//! `remove_edge_with_commands` 已恢复真实实体与四路稀疏分配的释放；RT条目由
//! `invalidate_dirty_stair_artifacts` 使用独立edge identity移除，不归此helper重复释放。
//! `reset_edge` 的旧兼容入口仍未恢复完整实体/缓存重建；原invalidate不调用它。
use crate::render::sparse_instance_buffer::{
    MeshAtlasAllocation, SparseInstanceBuffer, SparseSsbo,
};
use crate::resources::stairs::stairs_state::StairGraphEdge;
use bevy_ecs::{component::Component, resource::Resource};
use hashbrown::HashMap;

/// 原楼层mesh的共享物理缓冲：原Arc<Buffer>在最后一个chunk卸载后释放同一设备分配。
/// 当前40B Buffer为Copy句柄回执，必须保留真实设备lease，不能仅Arc包回执而泄漏物理资源。
pub struct StairsFloorBuffer {
    device: std::sync::Arc<dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice + Send + Sync>,
    buffer: crate::render::ssbo::rhapsody_boundary::Buffer,
}
impl StairsFloorBuffer {
    pub fn new(device: std::sync::Arc<dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice + Send + Sync>, buffer: crate::render::ssbo::rhapsody_boundary::Buffer) -> Self { Self {device,buffer} }
    pub fn handle(&self) -> &crate::render::ssbo::rhapsody_boundary::Buffer { &self.buffer }
}
impl std::fmt::Debug for StairsFloorBuffer {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.debug_struct("StairsFloorBuffer").field("buffer",&self.buffer).finish_non_exhaustive() }
}
impl Drop for StairsFloorBuffer { fn drop(&mut self) { self.device.release_buffer(&self.buffer); } }

/// 原1407dd873..8db：Arc@0、32B TrimeshChunk@8、来源u32@40，组件48B。
/// 所有chunk共享一个lease，reset按原Vec<Entity>逐个despawn即可正确回收。
#[repr(C)]
#[derive(Component, Clone, Debug)]
pub struct StairsFloorMeshChunk {
    pub buffer: std::sync::Arc<StairsFloorBuffer>,
    pub chunk: crate::render::nani::trimesh::TrimeshChunk,
    pub source: u32,
}
impl StairsFloorMeshChunk {
    /// 原14207fcc0复制+8起36B：最后一个字是source，不能只上传TrimeshChunk的32B。
    pub fn gpu_words(&self) -> [u32; 9] {
        [self.chunk.address as u32, (self.chunk.address >> 32) as u32,
            u32::from(self.chunk.index_count) | (u32::from(self.chunk.vertex_count) << 16),
            self.chunk.attributes_offset, self.chunk.sphere_center.x.to_bits(), self.chunk.sphere_center.y.to_bits(),
            self.chunk.sphere_center.z.to_bits(), self.chunk.sphere_radius.to_bits(), self.source]
    }
}

/// 原64B漂浮砖参数：14078c940的Vec元素与14077bd63..be20真实读取相互校验。
/// Transform占48B；余下四个标量分别驱动出现动画与竖直浮动，不能当实例矩阵。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StairsFloatingBrickParams {
    pub transform: crate::utils::components::transform::Transform,
    /// 原+30，按delta/.3累加并钳制0..1。
    pub appearance_progress: f32,
    /// 原+34，加在time*(frequency+2)之后。
    pub phase: f32,
    /// 原+38，原producer取Rng.f32。
    pub frequency: f32,
    /// 原+3c，在原两常量之间插值后乘sin，控制竖直浮动幅度。
    pub amplitude: f32,
}

/// 原边缓存24B；1407d2070..208e将后四个f32原样传WallHoles::add(kind4)。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StairsWallCutout {
    pub wall: crate::resources::walls::WallId,
    pub center: glam::Vec2,
    pub dimensions: glam::Vec2,
}

/// `MagicSupports`（B `visual.rs`；render 资源）。
///
/// 机器型（fresh 0x140aa8210及真实producer/consumer）：`{ +0 Vec<StairsFloatingBrickParams>, +0x18 计数 u32,
/// +0x20/+0x28 第一 mesh-atlas 写锁槽, +0x30/+0x38 第二槽, +0x40 标志 u8 }`。
/// 每槽 = `{ Weak<SparseInstanceBuffer::waste> @+0x00, Box<Allocation> @+0x08 }`
/// （`MeshAtlasAllocation`）：槽空 → `Arc::downgrade(*(buffer+0x60))` + 新 0x20 B
/// `Allocation`。
///
/// `clear` 0x140aa8210（参数：`&mut self, first: &mut SparseInstanceBuffer<BrickTransformSsbo>,
/// second: &mut SparseInstanceBuffer<SparkleSsbo>`）：
/// 1. 第一槽懒初始化 → LOCK 分配 `+0x18` → 以缓冲 `+0x20..+0x58`（byte_map/active/data/
///    dirty）与分配指针拼 writer → `SparseInstanceBufferWriter::clear<BrickTransformSsbo>`；
/// 2. 第二槽同样；
/// 3. 清漂浮砖Vec的len（+0x10），`+0x18`/`+0x40`归零；两把RawMutex逆序解锁。
///
/// 原main14000966e..675与reset140aa8c20..c31证明两宿主分别是Brick/Sparkle；
/// ICF把相同clear指令折叠为Brick符号，不能据符号别名把两缓冲误约束成同一个T。
/// **地址判定（fresh 更正 LOGIC §5）**：0x140869a30 的 Ghidra 符号与函数体均为
/// `structural_graph::StairsStructuralGraph::clear`（8 张 hashbrown 表 + 1 Vec），
/// 不是 MagicSupports::clear；本函数唯一地址是 0x140aa8210。
/// 旧实现 `HashSet<StairGraphEdge>` 与机器不同构（机器无边集合），已改 render 资源占位；
/// insert/remove/contains 等边集合方法系旧实现编造，删除（库内无消费面）。
#[derive(Resource, Debug, Default)]
pub struct MagicSupports {
    /// 原前三qword是真Vec，+10仅是len；clear必须保留容量并释放元素。
    pub floating_bricks: Vec<StairsFloatingBrickParams>,
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
    /// 原两路分配的只读owner；writer继续由各自真实SparseInstanceBuffer借出。
    pub fn allocations(&self) -> (Option<&MeshAtlasAllocation>, Option<&MeshAtlasAllocation>) {
        (self.first.as_ref(), self.second.as_ref())
    }

    /// `MagicSupports::clear`（0x140aa8210）：两槽懒初始化 → 两把分配锁 →
    /// 两个不同稀疏元素类型各自clear，保留原独立allocation与Vec容量、清len和计数。
    pub fn clear<T: SparseSsbo, U: SparseSsbo>(
        &mut self,
        first: &mut SparseInstanceBuffer<T>,
        second: &mut SparseInstanceBuffer<U>,
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
        self.floating_bricks.clear();
        self.count32 = 0;
        self.flag = 0;
    }
}

/// 边视觉条目的已证实owner：实体、两个独立分配槽与内嵌MagicSupports。
///
/// 除占位外，保留两个 mesh-atlas 写面分配槽：`reset_edge` 先懒初始化它们
/// （`Arc::downgrade(host+0x60)` + 新 Allocation），再各 `clear<BrickTransformSsbo>`
/// 一次。原0x108 B value的其它纯数据缓存尚待装配producer恢复，不能据此声称完整装配。
#[derive(Debug, Default)]
pub struct StairsEdgeVisual {
    /// 原value+0x100：强制重建请求；与全局dirty和实例分配分别拥有状态。
    force_rebuild_requested: bool,
    /// 第一写面分配槽（fresh 0x140aa8a60 `pauVar33[-3]` 槽）。
    first: Option<MeshAtlasAllocation>,
    /// 第二写面分配槽（`pauVar33[-2]` 槽）。
    second: Option<MeshAtlasAllocation>,
    /// 原value首24B的Vec<Entity>；remove_edge释放其它owner后逐项despawn。
    pub entities: Vec<bevy_ecs::entity::Entity>,
    /// 原value+18；碰撞/RT helper同时发布的三角顶面，供后续装配阶段复用。
    pub top_surface: Vec<glam::Vec4>,
    /// 原value+30；由主装配产生，墙洞阶段只读并保留原有序重复项。
    pub wall_cutouts: Vec<StairsWallCutout>,
    /// 原 +0x90 的44字节记录；语义尚未核实，保留原有序拥有关系。
    pub unknown_records_44: Vec<[u8; 44]>,
    /// 原内嵌MagicSupports；两路allocation随该edge生命周期回归真实waste队列。
    pub magic_supports: MagicSupports,
    /// 原1412946a2..6ae把LazyTile线性索引拆为x/y再各截u8；
    /// 原集合键是两字节坐标，不能直接用32位LazyTile身份代替。
    pub affected_tiles: hashbrown::HashSet<(u8, u8), std::hash::RandomState>,
}

impl StairsEdgeVisual {
    /// 原两个分离分配槽；不构造默认GPU宿主或第二实例owner。
    pub fn allocations_mut(&mut self) -> (&mut Option<MeshAtlasAllocation>, &mut Option<MeshAtlasAllocation>) {
        (&mut self.first, &mut self.second)
    }

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
    /// 无实体条目的旧存储兼容入口；真实失效系统必须使用带Commands的完整释放入口。
    /// 缺边不置dirty；有实体时拒绝静默漏掉despawn。
    pub fn remove_edge(&mut self, edge: StairGraphEdge) {
        let entities = self.take_edge_for_removal(edge);
        assert!(entities.is_empty(), "stair edge with entities requires remove_edge_with_commands");
    }

    /// 原140aa8620：先移除value并drop两allocation/MagicSupports，再置dirty并发despawn。
    /// 缺边不置全局dirty；原RT条目由invalidate按独立edge identity移除。
    pub fn remove_edge_with_commands(&mut self, edge: StairGraphEdge, commands: &mut bevy_ecs::system::Commands) {
        for entity in self.take_edge_for_removal(edge) {
            commands.entity(entity).despawn();
        }
    }

    fn take_edge_for_removal(&mut self, edge: StairGraphEdge) -> Vec<bevy_ecs::entity::Entity> {
        let Some(mut state) = self.edges.remove(&edge) else { return Vec::new(); };
        let entities = std::mem::take(&mut state.entities);
        drop(state);
        self.dirty = true;
        entities
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

    /// 原 reset 的三宿主写面。先清 Brick、内嵌 magic、Wooden，随后清缓存并发出实体卸载。
    /// 返回的两个 writer 持有原分配锁；缓存引用与 writer 分别借用独立字段。
    pub fn reset_edge_native<'a>(
        &'a mut self, edge: StairGraphEdge, commands: &mut bevy_ecs::system::Commands,
        brick: &'a mut SparseInstanceBuffer<crate::geometry::instanced_wall::BrickTransformSsbo>,
        sparkle: &mut SparseInstanceBuffer<crate::resources::sparkle::SparkleSsbo>,
        wooden: &'a mut SparseInstanceBuffer<crate::resources::roofs::roof_buffers::WoodenPlankSsbo>,
    ) -> (crate::render::sparse_instance_buffer::SparseInstanceBufferWriter<'a, crate::geometry::instanced_wall::BrickTransformSsbo>,
          crate::render::sparse_instance_buffer::SparseInstanceBufferWriter<'a, crate::resources::roofs::roof_buffers::WoodenPlankSsbo>,
          &'a mut Vec<bevy_ecs::entity::Entity>, &'a mut hashbrown::HashSet<(u8,u8), std::hash::RandomState>,
          &'a mut Vec<glam::Vec4>, &'a mut Vec<StairsWallCutout>) {
        let state = self.edges.entry(edge).or_default();
        let first = state.first.get_or_insert_with(|| MeshAtlasAllocation::new(brick));
        let first_lock = brick.clear_and_lock_allocation(first.allocation());
        state.magic_supports.clear(brick, sparkle);
        let second = state.second.get_or_insert_with(|| MeshAtlasAllocation::new(wooden));
        let second_lock = wooden.clear_and_lock_allocation(second.allocation());
        for entity in state.entities.drain(..) { commands.entity(entity).despawn(); }
        state.top_surface.clear();
        state.wall_cutouts.clear();
        state.unknown_records_44.clear();
        state.affected_tiles.clear();
        state.force_rebuild_requested = false;
        (brick.writer_for_locked(first_lock), wooden.writer_for_locked(second_lock),
            &mut state.entities, &mut state.affected_tiles, &mut state.top_surface, &mut state.wall_cutouts)
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
    pub fn get_mut(&mut self, edge: &StairGraphEdge) -> Option<&mut StairsEdgeVisual> {
        self.edges.get_mut(edge)
    }
    /// 原DirtyTiles::iter_tiles逐tile遍历已有视觉条目，不创建缺失边。
    pub fn force_rebuild_for_tile(&mut self, tile: crate::LazyTile) {
        let key = (tile.x() as u8, tile.y() as u8);
        for visual in self.edges.values_mut() {
            if visual.affected_tiles.contains(&key) { visual.force_rebuild_requested = true; }
        }
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

    fn floating_brick() -> StairsFloatingBrickParams {
        StairsFloatingBrickParams {
            transform: crate::utils::components::transform::Transform::identity(),
            appearance_progress: 1.0, phase: 0.25, frequency: 0.5, amplitude: 0.75,
        }
    }

    #[test]
    fn floating_brick_cache_preserves_original_transform_and_animation_layout() {
        use core::mem::{offset_of, size_of};
        assert_eq!(size_of::<StairsFloatingBrickParams>(), 64);
        assert_eq!(offset_of!(StairsFloatingBrickParams, appearance_progress), 0x30);
        assert_eq!(offset_of!(StairsFloatingBrickParams, phase), 0x34);
        assert_eq!(offset_of!(StairsFloatingBrickParams, frequency), 0x38);
        assert_eq!(offset_of!(StairsFloatingBrickParams, amplitude), 0x3c);
    }

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
        supports.floating_bricks = vec![floating_brick(); 5];
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
        assert!(supports.floating_bricks.is_empty());
        assert!(supports.floating_bricks.capacity() >= 5, "原clear保留Vec容量");
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
        assert!(supports.floating_bricks.is_empty());
        assert_eq!(supports.count32, 0);
        // 懒初始化产生两个空分配槽。
        assert!(supports.first.is_some());
        assert!(supports.second.is_some());
    }

    #[test]
    fn native_reset_preserves_allocations_clears_writers_and_despawns() {
        use crate::{geometry::instanced_wall::BrickTransformSsbo,
            resources::{sparkle::SparkleSsbo, roofs::roof_buffers::WoodenPlankSsbo}};
        let mut brick = SparseInstanceBuffer::<BrickTransformSsbo>::new_storage_buffer(&Dev, None);
        let mut sparkle = SparseInstanceBuffer::<SparkleSsbo>::new_storage_buffer(&Dev, None);
        let mut wooden = SparseInstanceBuffer::<WoodenPlankSsbo>::new_storage_buffer(&Dev, None);
        let mut world = bevy_ecs::world::World::new();
        let entity = world.spawn_empty().id();
        let mut queue = bevy_ecs::world::CommandQueue::default();
        let mut state = StairsVisualState::default();
        let edge = StairGraphEdge::new(1, 2);
        {
            let mut commands = bevy_ecs::system::Commands::new(&mut queue, &world);
            let (mut b, mut w, entities, tiles, top, holes) = state.reset_edge_native(edge, &mut commands, &mut brick, &mut sparkle, &mut wooden);
            b.push(&BrickTransformSsbo::default()).unwrap();
            w.push(&WoodenPlankSsbo::default()).unwrap();
            entities.push(entity); tiles.insert((3,4)); top.push(glam::Vec4::ONE);
            holes.push(StairsWallCutout { wall: crate::resources::walls::WallId(7), center: glam::Vec2::ONE, dimensions: glam::Vec2::ONE });
        }
        state.get_mut(&edge).unwrap().unknown_records_44.push([7;44]);
        let old_first = state.get(&edge).unwrap().first.as_ref().unwrap().allocation() as *const _;
        {
            let mut commands = bevy_ecs::system::Commands::new(&mut queue, &world);
            let (mut b, mut w, entities, tiles, top, holes) = state.reset_edge_native(edge, &mut commands, &mut brick, &mut sparkle, &mut wooden);
            assert!(entities.is_empty() && tiles.is_empty() && top.is_empty() && holes.is_empty());
            b.push(&BrickTransformSsbo::default()).unwrap();
            w.push(&WoodenPlankSsbo::default()).unwrap();
        }
        queue.apply(&mut world);
        assert!(world.get_entity(entity).is_err());
        let edge_state = state.get(&edge).unwrap();
        assert!(edge_state.unknown_records_44.is_empty());
        assert_eq!(edge_state.first.as_ref().unwrap().allocation() as *const _, old_first);
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
            magic.floating_bricks = vec![floating_brick(); 17];
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
            assert_eq!((magic.floating_bricks.len(), magic.count32, magic.flag), (17, 23, 9));
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
