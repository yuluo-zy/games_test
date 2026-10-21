//! 玩家楼梯的唯一视觉状态与真实 GPU 缓冲生命周期。
//!
//! B：reset 140aa8a60/2322B、remove 140aa8620/1087B、Magic.clear
//! 140aa8210/1039B。原 value 为 0x108B，0x110B 包含外层边键。
//! 类型与副作用合同见 TG-ad9668ff273a982820 的独立审查；这里不代替楼梯几何生产者。

use crate::geometry::instanced_wall::BrickTransformSsbo;
use crate::render::nani::trimesh::TrimeshChunk;
use crate::render::sparse_instance_buffer::{
    MeshAtlasAllocation, SparseInstanceBuffer, SparseInstanceBufferWriter,
};
use crate::render::ssbo::rhapsody_boundary::{Buffer, GraphicsDevice};
use crate::resources::roofs::roof_buffers::WoodenPlankSsbo;
use crate::resources::sparkle::SparkleSsbo;
use crate::resources::stairs::stairs_state::StairGraphEdge;
use crate::resources::stairs_walls::StairsPillarProposal;
use crate::resources::walls::WallId;
use crate::utils::components::transform::Transform;
use bevy_ecs::{component::Component, entity::Entity, resource::Resource, system::Commands};
use glam::Vec4;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// 原 Vec64 的真实元素；Transform 的尾部补齐不作为额外业务字段。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StairsFloatingBrickParams {
    pub transform: Transform,
    pub progress: f32,
    pub phase: f32,
    pub speed: f32,
    pub amplitude: f32,
}

/// 每条边内联的浮动砖缓存及两种真实实例分配。
#[derive(Resource, Debug, Default)]
pub struct MagicSupports {
    pub floating_bricks: Vec<StairsFloatingBrickParams>,
    pub count32: u32,
    brick_allocation: Option<MeshAtlasAllocation>,
    sparkle_allocation: Option<MeshAtlasAllocation>,
    pub flag: u8,
}

impl MagicSupports {
    /// 先清 Brick，保持其分配锁，再清 Sparkle；两锁持有时清缓存，逆序解锁。
    pub fn clear(
        &mut self,
        bricks: &mut SparseInstanceBuffer<BrickTransformSsbo>,
        sparkles: &mut SparseInstanceBuffer<SparkleSsbo>,
    ) {
        let brick = self
            .brick_allocation
            .get_or_insert_with(|| MeshAtlasAllocation::new(bricks));
        let brick_lock = bricks.clear_and_lock_allocation(brick.allocation());
        let sparkle = self
            .sparkle_allocation
            .get_or_insert_with(|| MeshAtlasAllocation::new(sparkles));
        let sparkle_lock = sparkles.clear_and_lock_allocation(sparkle.allocation());
        self.floating_bricks.clear();
        self.count32 = 0;
        self.flag = 0;
        drop(sparkle_lock);
        drop(brick_lock);
    }
}

impl Drop for MagicSupports {
    fn drop(&mut self) {
        // 原析构先归还两个分配句柄，随后才析构 Vec64。
        drop(self.brick_allocation.take());
        drop(self.sparkle_allocation.take());
    }
}

/// 原单边状态。字段声明中的拥有者顺序保留原析构的外部副作用顺序。
#[derive(Debug, Default)]
pub struct StairsEdgeVisualState {
    pub entities: Vec<Entity>,
    pub dirty_cells: HashSet<[u8; 2]>,
    pub floor_vertices: Vec<Vec4>,
    /// 位置两分量、尺寸两分量；真实墙洞消费者另外传原 kind4。
    pub wall_holes: Vec<(WallId, [f32; 4])>,
    pub magic: MagicSupports,
    pub pillar_proposals: Vec<StairsPillarProposal>,
    brick_allocation: Option<MeshAtlasAllocation>,
    wooden_allocation: Option<MeshAtlasAllocation>,
    /// 原 value+f8；本轮仅恢复原存储和清零，不猜测其上游业务名称。
    pub word_f8: u64,
    force_rebuild_requested: bool,
    /// 原 value+101；不把未知 byte 解释为新的渲染门。
    pub flag_101: u8,
}

/// 兼容现有调用方的类型名，仍为同一个状态 owner。
pub type StairsEdgeVisual = StairsEdgeVisualState;

impl StairsEdgeVisualState {
    pub fn force_rebuild_requested(&self) -> bool {
        self.force_rebuild_requested
    }
}

impl Drop for StairsEdgeVisualState {
    fn drop(&mut self) {
        // 之后按声明顺序析构实体 Vec、cell set、两个 Vec、Magic 和支柱 Vec。
        drop(self.brick_allocation.take());
        drop(self.wooden_allocation.take());
    }
}

/// 原 sret 的六项顺序：两实例 writer，实体、脏格、顶点、墙洞缓存。
pub type StairsEdgeReset<'a> = (
    SparseInstanceBufferWriter<'a, BrickTransformSsbo>,
    SparseInstanceBufferWriter<'a, WoodenPlankSsbo>,
    &'a mut Vec<Entity>,
    &'a mut HashSet<[u8; 2]>,
    &'a mut Vec<Vec4>,
    &'a mut Vec<(WallId, [f32; 4])>,
);

#[derive(Resource, Debug, Default)]
pub struct StairsVisualState {
    dirty: bool,
    edges: HashMap<StairGraphEdge, StairsEdgeVisualState>,
}

fn queue_despawns(commands: &mut Commands<'_, '_>, entities: impl IntoIterator<Item = Entity>) {
    for entity in entities {
        // 原 drain 的终止编码；安全构造的 Bevy Entity 本身保证 generation 非零。
        if entity.to_bits() >> 32 == 0 {
            break;
        }
        commands.entity(entity).despawn();
    }
}

impl StairsVisualState {
    /// 缺失边没有副作用；存在边先析构其实例分配，再置脏并排队销毁实体。
    pub fn remove_edge(&mut self, edge: StairGraphEdge, commands: &mut Commands<'_, '_>) {
        if let Some(mut state) = self.edges.remove(&edge) {
            let entities = std::mem::take(&mut state.entities);
            drop(state);
            self.dirty = true;
            queue_despawns(commands, entities);
        }
    }

    /// 在同一条目重用分配和缓存容量；不写全局 dirty，不操作 RT/物理世界。
    pub fn reset_edge<'a>(
        &'a mut self,
        edge: StairGraphEdge,
        commands: &mut Commands<'_, '_>,
        bricks: &'a mut SparseInstanceBuffer<BrickTransformSsbo>,
        sparkles: &mut SparseInstanceBuffer<SparkleSsbo>,
        wooden_planks: &'a mut SparseInstanceBuffer<WoodenPlankSsbo>,
    ) -> StairsEdgeReset<'a> {
        let StairsEdgeVisualState {
            entities,
            dirty_cells,
            floor_vertices,
            wall_holes,
            magic,
            pillar_proposals,
            brick_allocation,
            wooden_allocation,
            word_f8,
            force_rebuild_requested,
            flag_101,
        } = self.edges.entry(edge).or_default();

        let brick = brick_allocation.get_or_insert_with(|| MeshAtlasAllocation::new(bricks));
        let brick_lock = bricks.clear_and_lock_allocation(brick.allocation());
        magic.clear(bricks, sparkles);
        let wooden =
            wooden_allocation.get_or_insert_with(|| MeshAtlasAllocation::new(wooden_planks));
        let wooden_lock = wooden_planks.clear_and_lock_allocation(wooden.allocation());

        queue_despawns(commands, entities.drain(..));
        // 原 8daf→8e2d：两个 Vec、cell set、支柱 Vec、两个 flag、qword。
        floor_vertices.clear();
        wall_holes.clear();
        dirty_cells.clear();
        pillar_proposals.clear();
        *force_rebuild_requested = false;
        *flag_101 = 0;
        *word_f8 = 0;

        (
            bricks.writer_for_locked(brick_lock),
            wooden_planks.writer_for_locked(wooden_lock),
            entities,
            dirty_cells,
            floor_vertices,
            wall_holes,
        )
    }

    /// 原 interpolate 的失效门：只标记已存在的条目，不建表、不清实例或缓存。
    pub fn force_rebuild(&mut self, edge: StairGraphEdge) {
        if let Some(state) = self.edges.get_mut(&edge) {
            state.force_rebuild_requested = true;
        }
    }

    pub fn get(&self, edge: &StairGraphEdge) -> Option<&StairsEdgeVisualState> {
        self.edges.get(edge)
    }
    pub fn get_mut(&mut self, edge: &StairGraphEdge) -> Option<&mut StairsEdgeVisualState> {
        self.edges.get_mut(edge)
    }
    pub fn contains(&self, edge: &StairGraphEdge) -> bool {
        self.edges.contains_key(edge)
    }
    pub fn edges(&self) -> &HashMap<StairGraphEdge, StairsEdgeVisualState> {
        &self.edges
    }
    pub fn edges_mut(&mut self) -> &mut HashMap<StairGraphEdge, StairsEdgeVisualState> {
        &mut self.edges
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }
}

/// 原 Arc<Buffer> 在现 Copy Buffer 边界的 RAII 适配；所有 chunk 共享同一份。
/// 不创建后端表；最后一个 owner 归还创建者，底层宿主负责在途资源延迟回收。
pub struct StairsMeshBuffer {
    buffer: Buffer,
    device: Arc<dyn GraphicsDevice + Send + Sync>,
}

impl StairsMeshBuffer {
    pub fn new(buffer: Buffer, device: Arc<dyn GraphicsDevice + Send + Sync>) -> Self {
        Self { buffer, device }
    }
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }
}

impl std::fmt::Debug for StairsMeshBuffer {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple("StairsMeshBuffer")
            .field(&self.buffer)
            .finish()
    }
}

impl Drop for StairsMeshBuffer {
    fn drop(&mut self) {
        self.device.release_buffer(&self.buffer);
    }
}

/// 原真实 Nani floor 组件：强持 Buffer、32B meshlet、Info.u28。
#[repr(C)]
#[derive(Component, Clone, Debug)]
pub struct StairsFloorMeshChunk {
    pub buffer: Arc<StairsMeshBuffer>,
    pub chunk: TrimeshChunk,
    pub info_28: u32,
}

impl StairsFloorMeshChunk {
    /// 原14207fcc0逐字复制组件+8起9word；保留 NaN 等浮点原始位。
    pub fn instance_words(&self) -> [u32; 9] {
        [
            self.chunk.address as u32,
            (self.chunk.address >> 32) as u32,
            u32::from(self.chunk.index_count) | (u32::from(self.chunk.vertex_count) << 16),
            self.chunk.attributes_offset,
            self.chunk.sphere_center.x.to_bits(),
            self.chunk.sphere_center.y.to_bits(),
            self.chunk.sphere_center.z.to_bits(),
            self.chunk.sphere_radius.to_bits(),
            self.info_28,
        ]
    }
}

/// 旧名称兼容同一组件身份，不能再构造空 marker 冒充楼梯网格。
pub type StairsVisualComponent = StairsFloorMeshChunk;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::nani::{instance_updates::build_stairs_floor, trimesh::Trimesh};
    use crate::render::ssbo::rhapsody_boundary::{BackendError, BufferDesc};
    use bevy_ecs::{world::CommandQueue, world::World};
    use glam::{Quat, Vec3};
    use std::ops::Range;
    use std::sync::{
        Mutex,
        atomic::{AtomicU64, Ordering},
    };

    /// 只记录后端所有权，不冒充真实 GPU；几何与实例存储仍用生产实现。
    #[derive(Default)]
    struct RecordDevice {
        next: AtomicU64,
        released: Mutex<Vec<Buffer>>,
        uploads: Mutex<Vec<Vec<u8>>>,
    }
    impl GraphicsDevice for RecordDevice {
        fn create_buffer(
            &self,
            desc: &BufferDesc,
            _: &str,
            data: Option<&[u8]>,
        ) -> Result<Buffer, BackendError> {
            if let Some(data) = data {
                self.uploads.lock().unwrap().push(data.to_vec());
            }
            Ok(Buffer {
                id: self.next.fetch_add(1, Ordering::Relaxed) + 1,
                generation: 7,
                byte_size: desc.size as u64,
                ..Buffer::default()
            })
        }
        fn release_buffer(&self, buffer: &Buffer) {
            self.released.lock().unwrap().push(*buffer);
        }
        fn update_buffer_bytes(
            &self,
            _: &Buffer,
            _: &[u8],
            _: &[Range<usize>],
            _: usize,
        ) -> Result<(), BackendError> {
            Ok(())
        }
        fn device_address(&self, buffer: &Buffer) -> u64 {
            0x1234_5678_0000_0000 + buffer.id * 0x10000
        }
    }

    fn floor_chunk(device: &Arc<RecordDevice>) -> StairsFloorMeshChunk {
        let positions = [
            Vec3::new(-1.0, 0.25, 2.0),
            Vec3::new(3.0, 0.5, 2.0),
            Vec3::new(0.0, 1.0, -4.0),
        ];
        let normals = [Vec3::Y; 3];
        let mesh = Trimesh::builder(&[0, 1, 2])
            .positions(&positions)
            .attrib(&normals)
            .build(device.as_ref(), "stairs-visual-owned-fixture")
            .unwrap();
        assert_eq!(mesh.chunks.len(), 1);
        assert_eq!(mesh.chunks[0].index_count(), 3);
        let buffer = Arc::new(StairsMeshBuffer::new(mesh.buffer, device.clone()));
        StairsFloorMeshChunk {
            buffer,
            chunk: mesh.chunks[0],
            info_28: 0x89ab_cdef,
        }
    }

    fn allocation_ids(row: &StairsEdgeVisualState) -> [usize; 4] {
        [
            row.brick_allocation.as_ref().unwrap().allocation() as *const _ as usize,
            row.wooden_allocation.as_ref().unwrap().allocation() as *const _ as usize,
            row.magic.brick_allocation.as_ref().unwrap().allocation() as *const _ as usize,
            row.magic.sparkle_allocation.as_ref().unwrap().allocation() as *const _ as usize,
        ]
    }

    #[test]
    fn real_typed_reset_and_remove_reuse_allocations_and_queue_entity_and_buffer_retirement() {
        let device = Arc::new(RecordDevice::default());
        // 同一组真实类型/生产容量串行复用，避免每个断言重复分配 Brick 的大影子。
        let mut bricks =
            SparseInstanceBuffer::<BrickTransformSsbo>::try_new_owned(device.clone()).unwrap();
        let mut sparkles =
            SparseInstanceBuffer::<SparkleSsbo>::try_new_owned(device.clone()).unwrap();
        let mut wood =
            SparseInstanceBuffer::<WoodenPlankSsbo>::try_new_owned(device.clone()).unwrap();
        let mut visual = StairsVisualState::default();
        let edge = StairGraphEdge::new(31, 7);
        let mut world = World::new();
        let mut queue = CommandQueue::default();
        {
            let mut commands = Commands::new(&mut queue, &world);
            drop(visual.reset_edge(edge, &mut commands, &mut bricks, &mut sparkles, &mut wood));
        }
        assert!(!visual.is_dirty());
        let ids = allocation_ids(visual.get(&edge).unwrap());
        assert_ne!(ids[0], ids[2], "同 Brick buffer 的状态与 Magic 是不同分配");
        let chunk = floor_chunk(&device);
        let mesh_buffer = *chunk.buffer.buffer();
        let a = world.spawn(chunk.clone()).id();
        let b = world.spawn(chunk).id();
        let (
            brick_chunk,
            magic_brick_chunk,
            sparkle_chunk,
            wooden_chunk,
            vector_capacity,
            cell_capacity,
        );
        {
            let row = visual.get_mut(&edge).unwrap();
            let state_brick = row.brick_allocation.as_ref().unwrap().allocation();
            let magic_brick = row.magic.brick_allocation.as_ref().unwrap().allocation();
            let magic_sparkle = row.magic.sparkle_allocation.as_ref().unwrap().allocation();
            let state_wooden = row.wooden_allocation.as_ref().unwrap().allocation();
            let mut brick = BrickTransformSsbo::default();
            brick.seed = 42;
            bricks.writer_for(state_brick).push(&brick).unwrap();
            brick.seed = 43;
            bricks.writer_for(magic_brick).push(&brick).unwrap();
            sparkles
                .writer_for(magic_sparkle)
                .push(&SparkleSsbo {
                    speed_mult: 7.0,
                    ..SparkleSsbo::default()
                })
                .unwrap();
            wood.writer_for(state_wooden)
                .push(&WoodenPlankSsbo {
                    piece_seed: 13,
                    ..WoodenPlankSsbo::default()
                })
                .unwrap();
            brick_chunk = state_brick.chunk_ids.lock()[0] as usize;
            magic_brick_chunk = magic_brick.chunk_ids.lock()[0] as usize;
            sparkle_chunk = magic_sparkle.chunk_ids.lock()[0] as usize;
            wooden_chunk = state_wooden.chunk_ids.lock()[0] as usize;
            row.entities.extend([a, b]);
            row.floor_vertices.push(Vec4::new(1.0, 2.0, 3.0, 1.0));
            row.wall_holes.push((WallId(91), [2.0, 4.0, 1.0, 10.0]));
            row.dirty_cells.insert([7, 11]);
            row.pillar_proposals
                .push(StairsPillarProposal::from_observed([0x5a; 44]));
            row.magic.floating_bricks.push(StairsFloatingBrickParams {
                transform: Transform {
                    rotation: Quat::from_rotation_y(0.4),
                    translation: Vec3::new(2.0, 3.0, 4.0),
                    scale: Vec3::new(0.7, 0.8, 0.9),
                },
                progress: 0.25,
                phase: 0.5,
                speed: 0.75,
                amplitude: 1.0,
            });
            row.magic.count32 = 37;
            row.magic.flag = 1;
            row.word_f8 = 19;
            row.flag_101 = 1;
            vector_capacity = [
                row.entities.capacity(),
                row.floor_vertices.capacity(),
                row.wall_holes.capacity(),
                row.pillar_proposals.capacity(),
                row.magic.floating_bricks.capacity(),
            ];
            cell_capacity = row.dirty_cells.capacity();
        }
        visual.force_rebuild(edge);
        assert!(visual.get(&edge).unwrap().force_rebuild_requested());
        assert_eq!(allocation_ids(visual.get(&edge).unwrap()), ids);
        assert_eq!(visual.get(&edge).unwrap().entities, [a, b]);
        assert_eq!(visual.get(&edge).unwrap().magic.count32, 37);
        assert_eq!(bricks.byte_map[brick_chunk], 1);
        assert_eq!(bricks.byte_map[magic_brick_chunk], 1);
        assert!(!visual.is_dirty());
        visual.dirty = true;
        visual.force_rebuild(edge);
        assert!(visual.is_dirty());
        visual.clear_dirty();
        let dirty = bricks
            .dirty_bitset
            .iter()
            .map(|v| v.load(Ordering::Relaxed))
            .collect::<Vec<_>>();
        {
            let mut commands = Commands::new(&mut queue, &world);
            let (_, _, entities, cells, vertices, holes) =
                visual.reset_edge(edge, &mut commands, &mut bricks, &mut sparkles, &mut wood);
            assert!(
                entities.is_empty() && cells.is_empty() && vertices.is_empty() && holes.is_empty()
            );
        }
        assert!(
            world.get_entity(a).is_ok() && world.get_entity(b).is_ok(),
            "Commands 尚未 apply"
        );
        assert!(device.released.lock().unwrap().is_empty());
        assert_eq!(allocation_ids(visual.get(&edge).unwrap()), ids);
        let row = visual.get(&edge).unwrap();
        assert_eq!(
            [
                row.entities.capacity(),
                row.floor_vertices.capacity(),
                row.wall_holes.capacity(),
                row.pillar_proposals.capacity(),
                row.magic.floating_bricks.capacity()
            ],
            vector_capacity
        );
        assert_eq!(row.dirty_cells.capacity(), cell_capacity);
        assert!(row.pillar_proposals.is_empty() && row.magic.floating_bricks.is_empty());
        assert_eq!(
            (row.magic.count32, row.magic.flag, row.word_f8, row.flag_101),
            (0, 0, 0, 0)
        );
        assert!(!row.force_rebuild_requested());
        assert!(!visual.is_dirty());
        for index in [brick_chunk, magic_brick_chunk] {
            assert_eq!(bricks.byte_map[index], 0);
        }
        assert_eq!(sparkles.byte_map[sparkle_chunk], 0);
        assert_eq!(wood.byte_map[wooden_chunk], 0);
        assert!(
            bricks
                .active_bitset
                .iter()
                .all(|v| v.load(Ordering::Relaxed) == 0)
        );
        assert!(
            sparkles
                .active_bitset
                .iter()
                .all(|v| v.load(Ordering::Relaxed) == 0)
        );
        assert!(
            wood.active_bitset
                .iter()
                .all(|v| v.load(Ordering::Relaxed) == 0)
        );
        assert_eq!(
            bricks
                .dirty_bitset
                .iter()
                .map(|v| v.load(Ordering::Relaxed))
                .collect::<Vec<_>>(),
            dirty
        );
        assert_eq!(bricks.data[brick_chunk << 5].seed, 42, "clear 不覆盖旧影子");
        assert_eq!(bricks.data[magic_brick_chunk << 5].seed, 43);
        assert_eq!(sparkles.data[sparkle_chunk << 5].speed_mult, 7.0);
        assert_eq!(wood.data[wooden_chunk << 5].piece_seed, 13);
        queue.apply(&mut world);
        assert!(world.get_entity(a).is_err() && world.get_entity(b).is_err());
        assert_eq!(
            *device.released.lock().unwrap(),
            [mesh_buffer],
            "同 Arc 的最后一个实体仅释放一次"
        );
        {
            let mut commands = Commands::new(&mut queue, &world);
            let (mut brick_writer, mut wooden_writer, ..) =
                visual.reset_edge(edge, &mut commands, &mut bricks, &mut sparkles, &mut wood);
            brick_writer.push(&BrickTransformSsbo::default()).unwrap();
            wooden_writer.push(&WoodenPlankSsbo::default()).unwrap();
        }
        assert_eq!(allocation_ids(visual.get(&edge).unwrap()), ids);
        assert!(
            bricks.waste.lock().is_empty()
                && sparkles.waste.lock().is_empty()
                && wood.waste.lock().is_empty()
        );
        {
            let mut commands = Commands::new(&mut queue, &world);
            visual.remove_edge(edge, &mut commands);
        }
        assert!(!visual.contains(&edge));
        assert!(visual.is_dirty());
        assert_eq!(bricks.waste.lock().len(), 2);
        assert_eq!(sparkles.waste.lock().len(), 1);
        assert_eq!(wood.waste.lock().len(), 1);
        drop((bricks, sparkles, wood));
        assert_eq!(
            device.released.lock().unwrap().len(),
            4,
            "三buffer和一个共享mesh均真实归还"
        );
    }

    #[test]
    fn floor_component_copies_exact_meshlet_words_and_keeps_one_shared_buffer_owner() {
        use core::mem::{align_of, offset_of, size_of};
        assert_eq!(size_of::<StairsFloatingBrickParams>(), 0x40);
        assert_eq!(align_of::<StairsFloatingBrickParams>(), 16);
        assert_eq!(offset_of!(StairsFloatingBrickParams, progress), 0x30);
        assert_eq!(offset_of!(StairsFloatingBrickParams, amplitude), 0x3c);
        assert_eq!(offset_of!(StairsFloorMeshChunk, chunk), 8);
        assert_eq!(offset_of!(StairsFloorMeshChunk, info_28), 0x28);
        let device = Arc::new(RecordDevice::default());
        let mut component = floor_chunk(&device);
        component.chunk.sphere_center.x = f32::from_bits(0x7fc0_1234);
        component.chunk.sphere_radius = -0.0;
        let words = component.instance_words();
        assert_eq!(
            words,
            [
                component.chunk.address as u32,
                (component.chunk.address >> 32) as u32,
                3 | (3 << 16),
                component.chunk.attributes_offset,
                0x7fc0_1234,
                component.chunk.sphere_center.y.to_bits(),
                component.chunk.sphere_center.z.to_bits(),
                0x8000_0000,
                0x89ab_cdef
            ]
        );
        let update = build_stairs_floor(words);
        assert_eq!(&update.words[..9], &words);
        assert_eq!(&update.words[9..], &[0, 0, 0]);
        assert_eq!((update.position, update.sort_key), (Vec3::ZERO, 1.0));
        let other = component.clone();
        assert!(Arc::ptr_eq(&component.buffer, &other.buffer));
        drop(component);
        assert!(device.released.lock().unwrap().is_empty());
        drop(other);
        assert_eq!(device.released.lock().unwrap().len(), 1);
        assert_eq!(
            device.uploads.lock().unwrap().len(),
            1,
            "真实 Trimesh builder 仅上传一次"
        );
    }

    #[test]
    fn missing_remove_and_force_do_not_create_state_or_dirty_and_existing_force_keeps_cached_entities()
     {
        let mut visual = StairsVisualState::default();
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let edge = StairGraphEdge::new(2, 9);
        let mut queue = CommandQueue::default();
        for dirty in [false, true] {
            visual.dirty = dirty;
            let mut commands = Commands::new(&mut queue, &world);
            visual.force_rebuild(edge);
            visual.remove_edge(edge, &mut commands);
            assert_eq!(visual.is_dirty(), dirty);
            assert!(visual.edges().is_empty());
        }
        visual
            .edges_mut()
            .entry(edge)
            .or_default()
            .entities
            .push(entity);
        visual.force_rebuild(edge);
        assert_eq!(visual.get(&edge).unwrap().entities, [entity]);
        assert!(visual.get(&edge).unwrap().force_rebuild_requested());
        {
            let mut commands = Commands::new(&mut queue, &world);
            visual.remove_edge(edge, &mut commands);
        }
        assert!(world.get_entity(entity).is_ok());
        queue.apply(&mut world);
        assert!(world.get_entity(entity).is_err());
    }
}
