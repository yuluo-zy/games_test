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
            .entry(wall)
            .or_insert_with(|| storage.new_allocation());
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
        let owner = self.support_bricks.entry(wall).or_insert_with(|| storage.new_allocation());
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
    pub fn clear_planks(&mut self, storage: &mut SparseInstanceBuffer<WoodenPlankSsbo>, wall: u64) {
        let owner = self
            .planks
            .entry(wall)
            .or_insert_with(|| storage.new_allocation());
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
        let owner = self.support_planks.entry(wall).or_insert_with(|| storage.new_allocation());
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
        if key.part == ConstructorPart::ElevationSupports {
            self.support_planks.get(&key.wall)
        } else { self.planks.get(&key.wall) }
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
    fn m6_support_bricks_keep_one_writer_owner_and_reset_only_matching_failures() {
        let key = ConstructorKey { wall: 44, part: ConstructorPart::ElevationSupports };
        let other = ConstructorKey { part: ConstructorPart::HalfTimber, ..key };
        let mut owners = ConstructorGpuOutput::default();
        let mut storage = SparseInstanceBuffer::<BrickTransformSsbo>::new_storage_buffer(&TestDevice(1), None);
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _writer = owners.brick_writer(&mut storage, key.wall);
        })).is_err());
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
        let key = ConstructorKey {
            wall: 44,
            part: ConstructorPart::Arches,
        };
        let gates = ConstructorKey {
            part: ConstructorPart::Gates,
            ..key
        };
        let mut owners = ConstructorGpuOutput::default();
        let mut storage =
            SparseInstanceBuffer::<BrickTransformSsbo>::new_storage_buffer(&TestDevice(1), None);
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
        owners.remove_wall(44);
        storage.take_out_waste();
        assert!(storage.to_draw_multi_instanced().is_empty());
        owners.append_bricks(&mut storage, key, &[BrickTransformSsbo::default()]);
        drop(storage); // Avoid simultaneously allocating two full192MiB shadows.
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
        owners.remove_wall(44);
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
