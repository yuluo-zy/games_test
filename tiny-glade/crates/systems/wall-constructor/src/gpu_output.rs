//! Source-owned constructor output -> current sparse SSBO library. These owners
//! complement the main wall InstancedWall; they never replace its allocation.
//! Original source points clear and append shared per-wall owners directly.
//! Capacity failure keeps successful prefixes; physical upload failures keep
//! dirty chunks for the real device retry. Deletion releases the owned chunks.
use bevy_ecs::prelude::*;
use country_core::{
    asset_libraries::{runtime::AssetLibraryRuntime, ssbo_library::AssetSsboLibrary},
    geometry::instanced_wall::BrickTransformSsbo,
    render::sparse_instance_buffer::{SparseInstanceBuffer, SparseInstanceBufferAllocation},
    resources::{roofs::roof_buffers::WoodenPlankSsbo, walls::public_walls::OnWallDeleted},
};
use hashbrown::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ConstructorPart {
    Arches,
    Gates,
    Lintels,
    Fences,
    HalfTimber,
    ElevationSupports,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ConstructorKey {
    pub wall: u64,
    pub part: ConstructorPart,
}
#[derive(Resource, Default)]
pub struct ConstructorGpuOutput {
    // 原PublicWall+120/+130：拱、门、过梁和通用柱共享。
    bricks: HashMap<u64, SparseInstanceBufferAllocation<BrickTransformSsbo>>,
    planks: HashMap<u64, SparseInstanceBufferAllocation<WoodenPlankSsbo>>,
    // 原PublicWall+150/+140：高程支撑与上方两个分配独立。
    support_bricks: HashMap<u64, SparseInstanceBufferAllocation<BrickTransformSsbo>>,
    support_planks: HashMap<u64, SparseInstanceBufferAllocation<WoodenPlankSsbo>>,
    // 原 PublicWallState+0x180：与支撑/门楣的 plank 分配独立。
    half_timber_planks: HashMap<u64, SparseInstanceBufferAllocation<WoodenPlankSsbo>>,
    /// Capacity and physical-storage errors are observable; never discarded.
    pub failures: HashMap<ConstructorKey, String>,
    pub last_device_error: Option<String>,
}
impl ConstructorGpuOutput {
    pub fn append_bricks(
        &mut self,
        storage: &mut SparseInstanceBuffer<BrickTransformSsbo>,
        key: ConstructorKey,
        records: &[BrickTransformSsbo],
    ) -> usize {
        let owners = if key.part == ConstructorPart::ElevationSupports {
            &mut self.support_bricks
        } else { &mut self.bricks };
        let owner = owners
            .entry(key.wall)
            .or_insert_with(|| storage.new_allocation());
        assert!(
            storage.owns_allocation(owner),
            "arch/gate owner belongs to another physical storage"
        );
        let mut writer = storage.append_writer(owner);
        let mut failures = 0usize;
        for record in records {
            if writer.push(record).is_err() {
                failures += 1;
            }
        }
        if failures != 0 {
            self.failures.insert(key, format!("{failures} constructor brick pushes exhausted capacity; successful prefix remains"));
        }
        failures
    }
    /// Original22 arch constructor clears the one shared arch/gate owner.
    pub fn clear_bricks(
        &mut self,
        storage: &mut SparseInstanceBuffer<BrickTransformSsbo>,
        wall: u64,
    ) {
        let owner = self
            .bricks
            .get_mut(&wall)
            .expect("原通用墙砖分配缺失");
        assert!(
            storage.owns_allocation(owner),
            "arch/gate owner belongs to another physical storage"
        );
        storage.writer(owner).clear();
        self.failures.retain(|key, _| {
            key.wall != wall
                || !matches!(key.part, ConstructorPart::Arches | ConstructorPart::Gates)
        });
    }
    pub fn clear_support_bricks(&mut self, storage: &mut SparseInstanceBuffer<BrickTransformSsbo>, wall: u64) {
        let owner = self.support_bricks.get_mut(&wall).expect("原支撑墙砖分配缺失");
        assert!(storage.owns_allocation(owner), "支撑墙砖分配属于另一物理存储");
        storage.writer(owner).clear();
        self.failures.remove(&ConstructorKey { wall, part: ConstructorPart::ElevationSupports });
    }
    /// 原+120通用柱追加到拱已经初始化的同一墙砖分配。
    pub fn wall_brick_writer<'a>(&'a self, storage: &'a mut SparseInstanceBuffer<BrickTransformSsbo>, wall: u64)
        -> country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<'a, BrickTransformSsbo> {
        let owner = self.bricks.get(&wall).expect("原通用墙砖分配缺失");
        assert!(storage.owns_allocation(owner), "通用墙砖分配属于另一物理存储");
        storage.append_writer(owner)
    }
    /// 原支撑生成器在整个随机切分与逐砖生成阶段持有同一分配锁。
    /// 分配由原清理/初始化阶段建立；这里不创建另一份墙输出。
    pub fn brick_writer<'a>(
        &'a self,
        storage: &'a mut SparseInstanceBuffer<BrickTransformSsbo>,
        wall: u64,
    ) -> country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<'a, BrickTransformSsbo> {
        let owner = self.support_bricks.get(&wall).expect("原支撑墙砖分配缺失");
        assert!(storage.owns_allocation(owner), "支撑墙砖分配属于另一物理存储");
        storage.append_writer(owner)
    }
    /// 所有逐项写入均已尝试；只记录容量失败，不回滚已生成的成功前缀。
    pub fn record_brick_failures(&mut self, key: ConstructorKey, count: usize) {
        if count != 0 {
            self.failures.insert(key,
                format!("{count} 支撑砖写入超出容量；成功前缀保留"));
        }
    }
    /// Original per-wall64B owner is shared by lintels/fences. Source systems
    /// append at their actual22/28 execution points, never deferred/reordered by
    /// HashMap iteration during44 upload. Every failed push is still attempted.
    pub fn append_planks(
        &mut self,
        storage: &mut SparseInstanceBuffer<WoodenPlankSsbo>,
        key: ConstructorKey,
        records: &[WoodenPlankSsbo],
    ) -> usize {
        let owners = if key.part == ConstructorPart::ElevationSupports {
            &mut self.support_planks
        } else { &mut self.planks };
        let owner = owners
            .entry(key.wall)
            .or_insert_with(|| storage.new_allocation());
        assert!(
            storage.owns_allocation(owner),
            "plank owner belongs to another physical storage"
        );
        let mut writer = storage.append_writer(owner);
        let mut failures = 0usize;
        for record in records {
            if writer.push(record).is_err() {
                failures += 1;
            }
        }
        if failures != 0 {
            self.failures.insert(key, format!("{failures} constructor plank pushes exhausted capacity; successful prefix remains"));
        }
        failures
    }
    /// B 140a8d18a..d200 / d9c8..d9dd: each successful publication
    /// installs a fresh +0x130 allocation and drops the replaced row's owner.
    pub fn replace_published_plank_owner(&mut self, storage: &SparseInstanceBuffer<WoodenPlankSsbo>, wall: u64) {
        drop(self.planks.remove(&wall));
        self.planks.insert(wall, storage.new_allocation());
    }
    pub fn clear_planks(&mut self, storage: &mut SparseInstanceBuffer<WoodenPlankSsbo>, wall: u64) {
        let owner = self.planks.get_mut(&wall).expect("原通用木条分配缺失");
        assert!(
            storage.owns_allocation(owner),
            "plank owner belongs to another physical storage"
        );
        storage.writer(owner).clear();
        self.failures.retain(|key, _| {
            key.wall != wall
                || !matches!(key.part, ConstructorPart::Lintels | ConstructorPart::Fences)
        });
    }
    pub fn clear_support_planks(&mut self, storage: &mut SparseInstanceBuffer<WoodenPlankSsbo>, wall: u64) {
        let owner = self.support_planks.get_mut(&wall).expect("原支撑木条分配缺失");
        assert!(storage.owns_allocation(owner), "支撑木条分配属于另一物理存储");
        storage.writer(owner).clear();
        self.failures.remove(&ConstructorKey { wall, part: ConstructorPart::ElevationSupports });
    }
    /// 原+130通用柱与过梁/围栏追加到同一木条分配。
    pub fn wall_plank_writer<'a>(&'a self, storage: &'a mut SparseInstanceBuffer<WoodenPlankSsbo>, wall: u64)
        -> country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<'a, WoodenPlankSsbo> {
        let owner = self.planks.get(&wall).expect("原通用木条分配缺失");
        assert!(storage.owns_allocation(owner), "通用木条分配属于另一物理存储");
        storage.append_writer(owner)
    }
    /// 原支撑木条沿同一+140分配持锁逐项写入，和半木的+180分配独立。
    pub fn plank_writer<'a>(&'a self, storage: &'a mut SparseInstanceBuffer<WoodenPlankSsbo>, wall: u64)
        -> country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<'a, WoodenPlankSsbo> {
        let owner = self.support_planks.get(&wall).expect("原支撑木条分配缺失");
        assert!(storage.owns_allocation(owner), "支撑木条分配属于另一物理存储");
        storage.append_writer(owner)
    }
    /// 记录一次完整原生成循环的失败，不忽略后续写入或丢弃成功前缀。
    pub fn record_plank_failures(&mut self, key: ConstructorKey, count: usize) {
        if count != 0 {
            self.failures.insert(key, format!("{count} 支撑木条写入超出容量；成功前缀保留"));
        }
    }
    /// 为一个墙状态建立半木分配；同墙已存在时保持原 owner。
    pub fn init_half_timber_planks(&mut self, storage: &SparseInstanceBuffer<WoodenPlankSsbo>, wall: u64) {
        let owner = self.half_timber_planks.entry(wall).or_insert_with(|| storage.new_allocation());
        assert!(storage.owns_allocation(owner), "半木分配属于另一物理存储");
    }
    /// 保持原半木生成期间同一分配锁；调用方在同一个 writer 上清除并逐条写入。
    pub fn half_timber_writer<'a>(&'a self, storage: &'a mut SparseInstanceBuffer<WoodenPlankSsbo>, wall: u64)
        -> country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<'a, WoodenPlankSsbo> {
        let owner = self.half_timber_planks.get(&wall).expect("原半木墙分配缺失");
        assert!(storage.owns_allocation(owner), "半木分配属于另一物理存储");
        storage.writer(owner)
    }
    /// 屋顶轮廓在半木写入之后重新加锁，并从原分配最后一块继续追加。
    pub fn half_timber_append_writer<'a>(&'a self, storage: &'a mut SparseInstanceBuffer<WoodenPlankSsbo>, wall: u64)
        -> country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<'a, WoodenPlankSsbo> {
        let owner = self.half_timber_planks.get(&wall).expect("原半木墙分配缺失");
        assert!(storage.owns_allocation(owner), "半木分配属于另一物理存储");
        storage.append_writer(owner)
    }
    /// 原半木函数要求原有分配，空槽是失败而不是隐式构造。
    pub fn clear_half_timber_planks(&mut self, storage: &mut SparseInstanceBuffer<WoodenPlankSsbo>, wall: u64) {
        let owner = self.half_timber_planks.get(&wall).expect("原半木墙分配缺失");
        assert!(storage.owns_allocation(owner), "半木分配属于另一物理存储");
        storage.writer(owner).clear();
        self.failures.remove(&ConstructorKey { wall, part: ConstructorPart::HalfTimber });
    }
    pub fn append_half_timber_planks(&mut self, storage: &mut SparseInstanceBuffer<WoodenPlankSsbo>, wall: u64,
        records: &[WoodenPlankSsbo]) -> usize {
        let owner = self.half_timber_planks.get(&wall).expect("原半木墙分配缺失");
        assert!(storage.owns_allocation(owner), "半木分配属于另一物理存储");
        let mut writer = storage.append_writer(owner);
        let mut failures = 0usize;
        for record in records { if writer.push(record).is_err() { failures += 1; } }
        if failures != 0 {
            self.failures.insert(ConstructorKey { wall, part: ConstructorPart::HalfTimber },
                format!("{failures} 半木梁写入超出容量；成功前缀保留"));
        }
        failures
    }
    pub fn half_timber_allocation(&self, wall: u64) -> Option<&SparseInstanceBufferAllocation<WoodenPlankSsbo>> {
        self.half_timber_planks.get(&wall)
    }
    /// 原有效PublicWall行替换会drop旧+180分配；无效source保留旧行与旧几何。
    pub fn retire_half_timber_planks(&mut self, wall: u64) {
        self.half_timber_planks.remove(&wall);
        self.failures.remove(&ConstructorKey { wall, part: ConstructorPart::HalfTimber });
    }
    pub fn brick_allocation(
        &self,
        key: ConstructorKey,
    ) -> Option<&SparseInstanceBufferAllocation<BrickTransformSsbo>> {
        if key.part == ConstructorPart::ElevationSupports {
            self.support_bricks.get(&key.wall)
        } else { self.bricks.get(&key.wall) }
    }
    pub fn plank_allocation(
        &self,
        key: ConstructorKey,
    ) -> Option<&SparseInstanceBufferAllocation<WoodenPlankSsbo>> {
        match key.part {
            ConstructorPart::ElevationSupports => self.support_planks.get(&key.wall),
            ConstructorPart::HalfTimber => self.half_timber_planks.get(&key.wall),
            _ => self.planks.get(&key.wall),
        }
    }
    pub fn remove_wall(&mut self, wall: u64) {
        // 原逐墙析构只释放该墙；批量删除不能为每堵墙扫描全部分配。
        self.bricks.remove(&wall);
        self.planks.remove(&wall);
        self.support_bricks.remove(&wall);
        self.support_planks.remove(&wall);
        self.half_timber_planks.remove(&wall);
        self.failures.retain(|key, _| key.wall != wall);
    }
}
/// Run after source constructors and before the canonical sparse upload. There
/// is one physical buffer: its active chunks are consumed by W21/W27 and Nani.
pub fn flush_constructor_output(
    runtime: Res<AssetLibraryRuntime>,
    mut library: ResMut<AssetSsboLibrary>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    let ConstructorGpuOutput {
        bricks,
        planks,
        support_bricks,
        support_planks,
        half_timber_planks,
        failures: _,
        last_device_error,
    } = &mut output.gpu;
    *last_device_error = None;
    if let Some(storage) = library.get_mut::<BrickTransformSsbo>() {
        if let Err(error) = storage.gpu_flush(runtime.buffer_device()) {
            *last_device_error = Some(error.to_string());
        }
    } else if !bricks.is_empty() || !support_bricks.is_empty() {
        *last_device_error = Some("BrickTransformSsbo is not registered".into());
    }
    if let Some(storage) = library.get_mut::<WoodenPlankSsbo>() {
        if let Err(error) = storage.gpu_flush(runtime.buffer_device()) {
            *last_device_error = Some(error.to_string());
        }
    } else if !planks.is_empty() || !support_planks.is_empty() || !half_timber_planks.is_empty() {
        *last_device_error = Some("WoodenPlankSsbo is not registered".into());
    }
}
/// Original19 clear, after publication and before22/28 source appenders.
pub fn clear_constructor_planks(
    mut changed: EventReader<country_core::resources::walls::public_walls::OnWallChanged>,
    walls: Res<country_core::resources::walls::public_walls::PublicWalls>,
    mut library: ResMut<AssetSsboLibrary>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    for event in changed.read() {
        walls
            .get(event.0)
            .expect("plank clear changed event requires published wall");
        let storage = library
            .get_mut::<WoodenPlankSsbo>()
            .expect("canonical WoodenPlankSsbo must exist before wall construction");
        output.gpu.clear_planks(storage, event.0.0);
    }
}
pub fn delete_constructor_output(
    mut deleted: EventReader<OnWallDeleted>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    for event in deleted.read() {
        output.gpu.remove_wall(event.0.0);
    }
}

/// Host adapter for the native publication-owned allocations. Only
/// successful replacement rows create owners; independent changed events do not.
pub fn replace_published_plank_owners(
    public: Res<country_core::resources::walls::public_walls::PublicWalls>,
    library: Res<AssetSsboLibrary>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    if public.last_delta.changed.is_empty() { return; }
    for &wall in &public.last_delta.changed {
        if public.get(wall).is_none() { continue; }
        // Native drops the old public row before constructing any new owner.
        drop(output.gpu.bricks.remove(&wall.0));
        drop(output.gpu.planks.remove(&wall.0));
        drop(output.gpu.support_planks.remove(&wall.0));
        drop(output.gpu.support_bricks.remove(&wall.0));
        drop(output.gpu.half_timber_planks.remove(&wall.0));
        let bricks = library.get::<BrickTransformSsbo>()
            .expect("published wall requires BrickTransformSsbo");
        let common_bricks = bricks.new_allocation();
        let support_bricks = bricks.new_allocation();
        let planks = library.get::<WoodenPlankSsbo>()
            .expect("published wall requires WoodenPlankSsbo");
        let common_planks = planks.new_allocation();
        let support_planks = planks.new_allocation();
        let half_timber_planks = planks.new_allocation();
        output.gpu.bricks.insert(wall.0, common_bricks);
        output.gpu.planks.insert(wall.0, common_planks);
        output.gpu.support_planks.insert(wall.0, support_planks);
        output.gpu.support_bricks.insert(wall.0, support_bricks);
        output.gpu.half_timber_planks.insert(wall.0, half_timber_planks);
    }
}

/// 当前CPU PublicWall与GPU sidecar分置；在实际publisher之后同步原行析构副作用。
/// OnWallChanged还可由其他入口产生，只有本次确已替换的权威行可以退役旧半木槽。
pub fn retire_republished_half_timber(
    mut changed: EventReader<country_core::resources::walls::public_walls::OnWallChanged>,
    public: Res<country_core::resources::walls::public_walls::PublicWalls>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    for event in changed.read() {
        if public.last_delta.changed.contains(&event.0) {
            output.gpu.retire_half_timber_planks(event.0.0);
        }
    }
}

/// The frozen geometry owner already emits all64 meaningful bytes. Preserve
/// their bit patterns instead of treating the affine as a row-major matrix or
/// applying default flags (original support/fence flags are zero).
pub fn wooden_plank(
    record: &country_core::systems::wall::construction::structures::WallWoodenPlankInstance64B,
) -> WoodenPlankSsbo {
    let a = &record.affine;
    WoodenPlankSsbo {
        transform: country_core::resources::render::Affine3Packed {
            cols: std::array::from_fn(|i| glam::Vec3::new(a[i * 3], a[i * 3 + 1], a[i * 3 + 2])),
        },
        gate_seed: record.segment_ordinal as i32,
        piece_seed: record.source_index as i32,
        wall_id: record.wall_id,
        flags: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::render::ssbo::rhapsody_boundary::{
        BackendError, Buffer, BufferDesc, GraphicsDevice,
    };
    /// CPU-shadow ownership fixture only. An attempted GPU upload fails
    /// explicitly; this test cannot accidentally count a fake upload as passed.
    struct TestDevice(u64);
    impl GraphicsDevice for TestDevice {
        fn create_buffer(
            &self,
            desc: &BufferDesc,
            _name: &str,
            _data: Option<&[u8]>,
        ) -> Result<Buffer, BackendError> {
            Ok(Buffer {
                id: self.0,
                byte_size: desc.size as u64,
                ..Default::default()
            })
        }
        fn update_buffer_bytes(
            &self,
            _buffer: &Buffer,
            _data: &[u8],
            _ranges: &[std::ops::Range<usize>],
            _offset: usize,
        ) -> Result<(), BackendError> {
            Err(BackendError::invalid_usage(
                "CPU-shadow fixture has no GPU upload device",
            ))
        }
    }
    #[test]
    fn republishing_retires_only_replaced_half_slot_and_invalid_source_keeps_it() {
        use country_core::resources::{roofs::geometry::surface::Curve2,
            walls::{InnerWalls, WallState, public_walls::{PublicWalls, OnWallChanged, WallPublicationDirty, WallSpatialHash},
                publication_source::TerrainHeightGrid512}};
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(),
            minimum_y: 0., maximum_y: 0., sampling_fix: false };
        let curve = Curve2::try_new(vec![[0., 0.], [2., 0.]], false).unwrap();
        let mut inner = InnerWalls::default();
        let wall = inner.new_wall(WallState::new(curve.clone(), 2., 0, 0, false));
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        // 初始发布已处理完，下面只检查后续替换，不把任意changed都当作替换。
        public.last_delta = Default::default();
        let mut storage = SparseInstanceBuffer::<WoodenPlankSsbo>::new_storage_buffer(&TestDevice(1), None);
        let mut output = crate::construct_arches::WallConstructorOutput::default();
        output.gpu.init_half_timber_planks(&storage, wall.0);
        output.gpu.append_half_timber_planks(&mut storage, wall.0, &[WoodenPlankSsbo::default()]);
        output.gpu.append_planks(&mut storage, ConstructorKey { wall: wall.0, part: ConstructorPart::Lintels }, &[WoodenPlankSsbo::default()]);
        let mut world = World::new();
        world.insert_resource(public);
        world.insert_resource(output);
        world.init_resource::<Events<OnWallChanged>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(retire_republished_half_timber);
        world.send_event(OnWallChanged(wall));
        schedule.run(&mut world);
        assert!(world.resource::<crate::construct_arches::WallConstructorOutput>().gpu.half_timber_allocation(wall.0).is_some());
        let mut invalid = curve.clone(); invalid.length = 0.;
        inner.update_wall_state_curve(wall, invalid);
        world.resource_mut::<PublicWalls>().consume_with_paths(&mut inner, &terrain, None,
            &WallPublicationDirty::default(), &WallSpatialHash::default());
        world.send_event(OnWallChanged(wall));
        schedule.run(&mut world);
        assert!(world.resource::<crate::construct_arches::WallConstructorOutput>().gpu.half_timber_allocation(wall.0).is_some());
        inner.update_wall_state_curve(wall, curve);
        world.resource_mut::<PublicWalls>().consume_with_paths(&mut inner, &terrain, None,
            &WallPublicationDirty::default(), &WallSpatialHash::default());
        world.send_event(OnWallChanged(wall));
        schedule.run(&mut world);
        let owners = &world.resource::<crate::construct_arches::WallConstructorOutput>().gpu;
        assert!(owners.half_timber_allocation(wall.0).is_none());
        assert!(owners.plank_allocation(ConstructorKey { wall: wall.0, part: ConstructorPart::Lintels }).is_some());
        storage.take_out_waste();
        assert_eq!(storage.to_draw_multi_instanced().iter().map(|(_, count)| *count).sum::<u32>(), 1);
        // 完整场景退出清理所有本场景GPU sidecar，两个槽均不得带入下一会话。
        world.resource_mut::<crate::construct_arches::WallConstructorOutput>().gpu = Default::default();
        storage.take_out_waste();
        assert!(storage.to_draw_multi_instanced().is_empty());
    }

    #[test]
    fn missing_wood_does_not_commit_partial_published_brick_owners() {
        use bevy_ecs::system::RunSystemOnce;
        use country_core::resources::{roofs::geometry::surface::Curve2,
            walls::{InnerWalls, WallState, public_walls::{PublicWalls, WallPublicationDirty, WallSpatialHash},
                publication_source::TerrainHeightGrid512}};
        let mut inner = InnerWalls::default();
        let wall = inner.new_wall(WallState::new(Curve2::try_new(vec![[0., 0.], [2., 0.]], false).unwrap(), 2., 0, 0, false));
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &TerrainHeightGrid512 { encoded: vec![0;512*512].into(), minimum_y: 0., maximum_y: 0., sampling_fix: false },
            None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        let library = AssetSsboLibrary::new();
        library.register::<BrickTransformSsbo>(SparseInstanceBuffer::new_storage_buffer(&TestDevice(30), None));
        let mut world = World::new(); world.insert_resource(public); world.insert_resource(library);
        world.init_resource::<crate::construct_arches::WallConstructorOutput>();
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            world.run_system_once(replace_published_plank_owners).unwrap();
        })).is_err());
        let gpu = &world.resource::<crate::construct_arches::WallConstructorOutput>().gpu;
        for part in [ConstructorPart::Arches, ConstructorPart::ElevationSupports] {
            assert!(gpu.brick_allocation(ConstructorKey { wall: wall.0, part }).is_none());
        }
    }

    #[test]
    fn published_owners_replace_only_successful_rows_and_missing_clear_panics() {
        use bevy_ecs::system::RunSystemOnce;
        use country_core::resources::{roofs::geometry::surface::Curve2,
            walls::{InnerWalls, WallState, public_walls::{PublicWalls, WallPublicationDirty, WallSpatialHash},
                publication_source::TerrainHeightGrid512}};
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(),
            minimum_y: 0., maximum_y: 0., sampling_fix: false };
        let curve = Curve2::try_new(vec![[0., 0.], [2., 0.]], false).unwrap();
        let mut inner = InnerWalls::default();
        let wall = inner.new_wall(WallState::new(curve.clone(), 2., 0, 0, false));
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        let library = AssetSsboLibrary::new();
        library.register::<BrickTransformSsbo>(SparseInstanceBuffer::new_storage_buffer(&TestDevice(30), None));
        library.register::<WoodenPlankSsbo>(SparseInstanceBuffer::new_storage_buffer(&TestDevice(31), None));
        let mut world = World::new();
        world.insert_resource(public); world.insert_resource(library);
        world.init_resource::<crate::construct_arches::WallConstructorOutput>();
        world.run_system_once(replace_published_plank_owners).unwrap();
        let mut main_wall = country_core::geometry::instanced_wall::InstancedWall::new(
            wall.0, vec![BrickTransformSsbo { seed: 31, ..Default::default() }]);
        world.resource_scope(|world, mut output: Mut<crate::construct_arches::WallConstructorOutput>| {
            let mut library = world.resource_mut::<AssetSsboLibrary>();
            let storage = library.get_mut::<WoodenPlankSsbo>().unwrap();
            output.gpu.wall_plank_writer(storage, wall.0).push(&WoodenPlankSsbo::default()).unwrap();
            output.gpu.plank_writer(storage, wall.0).push(&WoodenPlankSsbo::default()).unwrap();
            output.gpu.half_timber_writer(storage, wall.0).push(&WoodenPlankSsbo::default()).unwrap();
            let bricks = library.get_mut::<BrickTransformSsbo>().unwrap();
            main_wall.upload_to_storage_buffer(bricks).unwrap();
            output.gpu.wall_brick_writer(bricks, wall.0).push(&BrickTransformSsbo { seed: 32, ..Default::default() }).unwrap();
            output.gpu.brick_writer(bricks, wall.0).push(&BrickTransformSsbo::default()).unwrap();
        });
        let key = ConstructorKey { wall: wall.0, part: ConstructorPart::Lintels };
        let brick_key = ConstructorKey { wall: wall.0, part: ConstructorPart::Arches };
        let initial_brick_ranges = {
            let output = world.resource::<crate::construct_arches::WallConstructorOutput>();
            let library = world.resource::<AssetSsboLibrary>();
            let bricks = library.get::<BrickTransformSsbo>().unwrap();
            let common = bricks.allocation_to_draw_multi_instanced(output.gpu.brick_allocation(brick_key).unwrap());
            let main = bricks.allocation_to_draw_multi_instanced(main_wall.allocation().unwrap());
            assert_eq!(common.iter().map(|r| r.1).sum::<u32>(), 1);
            assert_eq!(main.iter().map(|r| r.1).sum::<u32>(), 1);
            assert_ne!(common, main);
            common
        };
        let owner_address = world.resource::<crate::construct_arches::WallConstructorOutput>().gpu
            .plank_allocation(key).unwrap().handle.0.as_ref().unwrap().as_ref() as *const _ as usize;
        // No new changes: the same publisher delta reset used by the live path.
        world.resource_mut::<PublicWalls>().consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        world.run_system_once(replace_published_plank_owners).unwrap();
        assert_eq!(world.resource::<crate::construct_arches::WallConstructorOutput>().gpu
            .plank_allocation(key).unwrap().handle.0.as_ref().unwrap().as_ref() as *const _ as usize, owner_address);
        {
            let output = world.resource::<crate::construct_arches::WallConstructorOutput>();
            let library = world.resource::<AssetSsboLibrary>();
            assert_eq!(library.get::<BrickTransformSsbo>().unwrap().allocation_to_draw_multi_instanced(
                output.gpu.brick_allocation(brick_key).unwrap()), initial_brick_ranges);
        }
        let mut invalid = curve.clone(); invalid.length = 0.;
        inner.update_wall_state_curve(wall, invalid);
        world.resource_mut::<PublicWalls>().consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        world.run_system_once(replace_published_plank_owners).unwrap();
        {
            let output = world.resource::<crate::construct_arches::WallConstructorOutput>();
            let library = world.resource::<AssetSsboLibrary>();
            assert_eq!(library.get::<BrickTransformSsbo>().unwrap().allocation_to_draw_multi_instanced(
                output.gpu.brick_allocation(brick_key).unwrap()), initial_brick_ranges);
        }
        { let library = world.resource::<AssetSsboLibrary>();
          assert_eq!(library.get::<WoodenPlankSsbo>().unwrap().to_draw_multi_instanced().iter().map(|x| x.1).sum::<u32>(), 3);
          assert_eq!(library.get::<BrickTransformSsbo>().unwrap().to_draw_multi_instanced().iter().map(|x| x.1).sum::<u32>(), 3); }
        inner.update_wall_state_curve(wall, curve);
        world.resource_mut::<PublicWalls>().consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        world.run_system_once(replace_published_plank_owners).unwrap();
        world.resource_scope(|world, mut output: Mut<crate::construct_arches::WallConstructorOutput>| {
            let mut library = world.resource_mut::<AssetSsboLibrary>();
            { let bricks = library.get_mut::<BrickTransformSsbo>().unwrap();
              bricks.take_out_waste();
              assert!(bricks.allocation_to_draw_multi_instanced(output.gpu.brick_allocation(brick_key).unwrap()).is_empty());
              assert_eq!(bricks.allocation_to_draw_multi_instanced(main_wall.allocation().unwrap()).iter().map(|r| r.1).sum::<u32>(), 1);
              assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| output.gpu.clear_bricks(bricks, wall.0 + 1))).is_err());
              output.gpu.wall_brick_writer(bricks, wall.0).push(&BrickTransformSsbo::default()).unwrap(); }
            let storage = library.get_mut::<WoodenPlankSsbo>().unwrap();
            storage.take_out_waste();
            assert!(storage.to_draw_multi_instanced().is_empty());
            output.gpu.wall_plank_writer(storage, wall.0).push(&WoodenPlankSsbo::default()).unwrap();
            assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| output.gpu.clear_planks(storage, wall.0 + 1))).is_err());
            assert_eq!(storage.to_draw_multi_instanced().iter().map(|x| x.1).sum::<u32>(), 1);
        });
        // Update then remove in the same frame: there is no live publication.
        inner.update_wall_state_curve(wall, Curve2::try_new(vec![[0., 0.], [3., 0.]], false).unwrap());
        inner.remove(wall);
        world.resource_mut::<PublicWalls>().consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        assert!(world.resource::<PublicWalls>().last_delta.changed.is_empty());
        assert!(world.resource::<PublicWalls>().last_delta.removed.contains(&wall));
        world.init_resource::<Events<OnWallDeleted>>();
        world.send_event(OnWallDeleted(wall));
        world.run_system_once(delete_constructor_output).unwrap();
        world.run_system_once(replace_published_plank_owners).unwrap();
        assert!(world.resource::<crate::construct_arches::WallConstructorOutput>().gpu.plank_allocation(key).is_none());
        assert!(world.resource::<crate::construct_arches::WallConstructorOutput>().gpu.brick_allocation(brick_key).is_none());
        let mut library = world.resource_mut::<AssetSsboLibrary>();
        { let bricks = library.get_mut::<BrickTransformSsbo>().unwrap();
          bricks.take_out_waste();
          assert_eq!(bricks.allocation_to_draw_multi_instanced(main_wall.allocation().unwrap()).iter().map(|r| r.1).sum::<u32>(), 1);
          drop(main_wall);
          bricks.take_out_waste();
          assert!(bricks.to_draw_multi_instanced().is_empty()); }
        let storage = library.get_mut::<WoodenPlankSsbo>().unwrap();
        storage.take_out_waste();
        assert!(storage.to_draw_multi_instanced().is_empty());
    }

    #[test]
    fn m6_support_bricks_keep_one_writer_owner_and_reset_only_matching_failures() {
        let key = ConstructorKey { wall: 44, part: ConstructorPart::ElevationSupports };
        let other = ConstructorKey { part: ConstructorPart::HalfTimber, ..key };
        let mut owners = ConstructorGpuOutput::default();
        let mut storage = SparseInstanceBuffer::<BrickTransformSsbo>::new_storage_buffer(&TestDevice(1), None);
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _writer = owners.brick_writer(&mut storage, key.wall);
        })).is_err());
        owners.support_bricks.insert(key.wall, storage.new_allocation());
        owners.clear_support_bricks(&mut storage, key.wall);
        {
            let mut writer = owners.brick_writer(&mut storage, key.wall);
            for seed in 0..35 { writer.push(&BrickTransformSsbo { seed, ..Default::default() }).unwrap(); }
        }
        owners.record_brick_failures(key, 2);
        owners.failures.insert(other, "独立半木失败".into());
        assert!(owners.failures.contains_key(&key));
        assert_eq!(storage.to_draw_multi_instanced(), vec![(0, 35)]);
        assert_eq!(storage.data[34].seed, 34);
        owners.clear_support_bricks(&mut storage, key.wall);
        assert!(!owners.failures.contains_key(&key));
        assert!(owners.failures.contains_key(&other));
        assert!(storage.to_draw_multi_instanced().is_empty());
    }

    #[test]
    fn arch_gate_tail_owner_clear_delete_and_foreign_storage_are_real() {
        use bevy_ecs::system::RunSystemOnce;
        use country_core::resources::{roofs::geometry::surface::Curve2,
            walls::{InnerWalls, WallState, public_walls::{PublicWalls, WallPublicationDirty, WallSpatialHash},
                publication_source::TerrainHeightGrid512}};
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(), minimum_y: 0., maximum_y: 0., sampling_fix: false };
        let mut inner = InnerWalls::default();
        let wall = inner.new_wall(WallState::new(Curve2::try_new(vec![[0., 0.], [2., 0.]], false).unwrap(), 2., 0, 0, false));
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        let library = AssetSsboLibrary::new();
        library.register::<BrickTransformSsbo>(SparseInstanceBuffer::new_storage_buffer(&TestDevice(1), None));
        library.register::<WoodenPlankSsbo>(SparseInstanceBuffer::new_storage_buffer(&TestDevice(2), None));
        let mut world = World::new();
        world.insert_resource(public); world.insert_resource(library);
        world.init_resource::<crate::construct_arches::WallConstructorOutput>();
        world.run_system_once(replace_published_plank_owners).unwrap();
        let key = ConstructorKey {
            wall: wall.0,
            part: ConstructorPart::Arches,
        };
        let gates = ConstructorKey {
            part: ConstructorPart::Gates,
            ..key
        };
        world.resource_scope(|world, mut output: Mut<crate::construct_arches::WallConstructorOutput>| {
        let owners = &mut output.gpu;
        let mut library = world.resource_mut::<AssetSsboLibrary>();
        let mut storage = library.get_mut::<BrickTransformSsbo>().unwrap();
        owners.clear_bricks(&mut storage, key.wall);
        assert_eq!(
            owners.append_bricks(
                &mut storage,
                key,
                &vec![
                    BrickTransformSsbo {
                        seed: 7,
                        ..Default::default()
                    };
                    35
                ]
            ),
            0
        );
        assert_eq!(
            owners.append_bricks(
                &mut storage,
                gates,
                &vec![
                    BrickTransformSsbo {
                        seed: 9,
                        ..Default::default()
                    };
                    2
                ]
            ),
            0
        );
        assert!(std::ptr::eq(
            owners.brick_allocation(key).unwrap(),
            owners.brick_allocation(gates).unwrap()
        ));
        assert_eq!(storage.to_draw_multi_instanced(), vec![(0, 37)]);
        assert_eq!(storage.data[34].seed, 7);
        assert_eq!(storage.data[35].seed, 9);
        owners.clear_bricks(&mut storage, key.wall);
        assert!(storage.to_draw_multi_instanced().is_empty());
        assert_eq!(
            owners.append_bricks(
                &mut storage,
                key,
                &vec![
                    BrickTransformSsbo {
                        seed: 11,
                        ..Default::default()
                    };
                    2
                ]
            ),
            0
        );
        assert_eq!(storage.to_draw_multi_instanced(), vec![(0, 2)]);
        owners.remove_wall(wall.0);
        storage.take_out_waste();
        assert!(storage.to_draw_multi_instanced().is_empty());
        });
        inner.update_wall_state_curve(wall, Curve2::try_new(vec![[0., 0.], [3., 0.]], false).unwrap());
        world.resource_mut::<PublicWalls>().consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        world.run_system_once(replace_published_plank_owners).unwrap();
        let mut owners = world.remove_resource::<crate::construct_arches::WallConstructorOutput>().unwrap().gpu;
        drop(world); // Avoid simultaneously allocating two full192MiB shadows.
        let mut foreign =
            SparseInstanceBuffer::<BrickTransformSsbo>::new_storage_buffer(&TestDevice(2), None);
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| owners.append_bricks(
                &mut foreign,
                key,
                &[BrickTransformSsbo::default()]
            )))
            .is_err()
        );
        assert!(foreign.to_draw_multi_instanced().is_empty());
        owners.remove_wall(wall.0);
    }
    #[test]
    fn wall_support_and_half_timber_owners_survive_each_others_clear_and_retire_together() {
        let wall = 71;
        let common = ConstructorKey { wall, part: ConstructorPart::Arches };
        let support = ConstructorKey { wall, part: ConstructorPart::ElevationSupports };
        let mut owners = ConstructorGpuOutput::default();
        let mut bricks = SparseInstanceBuffer::<BrickTransformSsbo>::new_storage_buffer(&TestDevice(1), None);
        let mut planks = SparseInstanceBuffer::<WoodenPlankSsbo>::new_storage_buffer(&TestDevice(2), None);
        owners.append_bricks(&mut bricks, common, &[BrickTransformSsbo { seed: 11, ..Default::default() }]);
        owners.append_bricks(&mut bricks, support, &[BrickTransformSsbo { seed: 22, ..Default::default() }]);
        owners.append_planks(&mut planks, common, &[WoodenPlankSsbo::default()]);
        owners.append_planks(&mut planks, support, &[WoodenPlankSsbo::default()]);
        owners.init_half_timber_planks(&planks, wall);
        owners.append_half_timber_planks(&mut planks, wall, &[WoodenPlankSsbo::default()]);
        let wall_bricks = bricks.allocation_to_draw_multi_instanced(owners.brick_allocation(common).unwrap());
        let support_bricks = bricks.allocation_to_draw_multi_instanced(owners.brick_allocation(support).unwrap());
        let wall_planks = planks.allocation_to_draw_multi_instanced(owners.plank_allocation(common).unwrap());
        let support_planks = planks.allocation_to_draw_multi_instanced(owners.plank_allocation(support).unwrap());
        let half = planks.allocation_to_draw_multi_instanced(owners.half_timber_allocation(wall).unwrap());
        let half_key = ConstructorKey { wall, part: ConstructorPart::HalfTimber };
        assert_eq!(planks.allocation_to_draw_multi_instanced(owners.plank_allocation(half_key).unwrap()), half);
        assert_eq!(wall_bricks.iter().map(|r| r.1).sum::<u32>(), 1);
        assert_eq!(support_bricks.iter().map(|r| r.1).sum::<u32>(), 1);
        assert_ne!(wall_bricks, support_bricks);
        assert_ne!(wall_planks, support_planks);
        assert_ne!(support_planks, half);
        owners.clear_support_bricks(&mut bricks, wall);
        owners.clear_support_planks(&mut planks, wall);
        assert_eq!(bricks.allocation_to_draw_multi_instanced(owners.brick_allocation(common).unwrap()), wall_bricks);
        assert_eq!(planks.allocation_to_draw_multi_instanced(owners.plank_allocation(common).unwrap()), wall_planks);
        assert_eq!(planks.allocation_to_draw_multi_instanced(owners.half_timber_allocation(wall).unwrap()), half);
        owners.append_bricks(&mut bricks, support, &[BrickTransformSsbo::default()]);
        owners.append_planks(&mut planks, support, &[WoodenPlankSsbo::default()]);
        owners.clear_bricks(&mut bricks, wall);
        owners.clear_planks(&mut planks, wall);
        assert!(!bricks.allocation_to_draw_multi_instanced(owners.brick_allocation(support).unwrap()).is_empty());
        assert!(!planks.allocation_to_draw_multi_instanced(owners.plank_allocation(support).unwrap()).is_empty());
        owners.remove_wall(wall);
        assert!(owners.brick_allocation(common).is_none());
        assert!(owners.brick_allocation(support).is_none());
        assert!(owners.plank_allocation(common).is_none());
        assert!(owners.plank_allocation(support).is_none());
        assert!(owners.half_timber_allocation(wall).is_none());
        assert!(owners.plank_allocation(half_key).is_none());
        bricks.take_out_waste();
        planks.take_out_waste();
        assert!(bricks.to_draw_multi_instanced().is_empty());
        assert!(planks.to_draw_multi_instanced().is_empty());
    }

    #[test]
    fn half_timber_and_support_planks_have_independent_wall_allocations() {
        let mut storage = SparseInstanceBuffer::<WoodenPlankSsbo>::new_storage_buffer(&TestDevice(5), None);
        let mut owners = ConstructorGpuOutput::default();
        let wall = 44;
        let key = ConstructorKey { wall, part: ConstructorPart::Fences };
        owners.init_half_timber_planks(&storage, wall);
        assert_eq!(owners.append_planks(&mut storage, key, &[WoodenPlankSsbo::default()]), 0);
        assert_eq!(owners.append_half_timber_planks(&mut storage, wall, &[WoodenPlankSsbo::default(); 2]), 0);
        assert!(!std::ptr::eq(owners.plank_allocation(key).unwrap(), owners.half_timber_allocation(wall).unwrap()));
        let half_before = storage.allocation_to_draw_multi_instanced(owners.half_timber_allocation(wall).unwrap());
        let other_before = storage.allocation_to_draw_multi_instanced(owners.plank_allocation(key).unwrap());
        assert_eq!(half_before.iter().map(|range| range.1).sum::<u32>(), 2);
        assert_eq!(other_before.iter().map(|range| range.1).sum::<u32>(), 1);
        owners.clear_planks(&mut storage, wall);
        assert!(storage.allocation_to_draw_multi_instanced(owners.plank_allocation(key).unwrap()).is_empty());
        assert_eq!(storage.allocation_to_draw_multi_instanced(owners.half_timber_allocation(wall).unwrap()), half_before);
        owners.append_planks(&mut storage, key, &[WoodenPlankSsbo::default()]);
        let other_after = storage.allocation_to_draw_multi_instanced(owners.plank_allocation(key).unwrap());
        owners.clear_half_timber_planks(&mut storage, wall);
        assert!(storage.allocation_to_draw_multi_instanced(owners.half_timber_allocation(wall).unwrap()).is_empty());
        assert_eq!(storage.allocation_to_draw_multi_instanced(owners.plank_allocation(key).unwrap()), other_after);
        {
            let mut writer = owners.half_timber_writer(&mut storage, wall);
            writer.clear();
            writer.push(&WoodenPlankSsbo::default()).unwrap();
        }
        let before = storage.allocation_to_draw_multi_instanced(owners.half_timber_allocation(wall).unwrap());
        assert_eq!(before.iter().map(|range| range.1).sum::<u32>(), 1);
        {
            let mut writer = owners.half_timber_append_writer(&mut storage, wall);
            writer.push(&WoodenPlankSsbo::default()).unwrap();
        }
        let after = storage.allocation_to_draw_multi_instanced(owners.half_timber_allocation(wall).unwrap());
        assert_eq!(after.len(), 1);
        assert_eq!(after[0].0, before[0].0);
        assert_eq!(after[0].1, 2);
        owners.remove_wall(wall);
        assert!(owners.half_timber_allocation(wall).is_none());
        assert!(owners.plank_allocation(key).is_none());
        storage.take_out_waste();
        assert!(storage.to_draw_multi_instanced().is_empty());
    }
    #[test]
    fn plank_projection_preserves_all_sixty_four_output_bytes() {
        let source =
            country_core::systems::wall::construction::structures::WallWoodenPlankInstance64B {
                affine: std::array::from_fn(|i| f32::from_bits(0x3f000000 + i as u32)),
                segment_ordinal: u32::MAX,
                source_index: 0xdeadbeef,
                wall_id: 43,
            };
        let plank = wooden_plank(&source);
        let mut bytes = [0; 64];
        for (i, v) in plank
            .transform
            .cols
            .iter()
            .flat_map(|v| v.to_array())
            .enumerate()
        {
            bytes[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
        }
        bytes[48..52].copy_from_slice(&plank.gate_seed.to_le_bytes());
        bytes[52..56].copy_from_slice(&plank.piece_seed.to_le_bytes());
        bytes[56..60].copy_from_slice(&plank.wall_id.to_le_bytes());
        bytes[60..64].copy_from_slice(&plank.flags.to_le_bytes());
        assert_eq!(bytes, source.observed_bytes());
    }
}
