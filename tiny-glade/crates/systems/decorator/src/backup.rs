//! 原 B backup.rs：九个资源共同保存最近有效预览，拒绝时回滚全部派生状态。
//! 原1408238a0先消费state，再逐桶、三map、0x88事件、延迟历史及楼梯恢复。
use crate::{
    blueprint::DecoratorBlueprints,
    deferred_decorator_history::{DeferredDecoratorEditHistory, DeferredDecoratorRecord},
};
use bevy_ecs::prelude::*;
use country_core::resources::{
    stairs::stairs_state::StairsState,
    walls::{
        cached_decorator_transforms::CachedDecoratorTransforms,
        decorator::{DecoratorId, DeferredDecoratorOpEvents},
        decorator_archivist::changes::{DecoratorArchivist, TrackedDecorators},
        decorator_storage::{DecoratorStorage, DecoratorStorageAddr, Decorators},
        decorator_subtype::DerivedDecoratorSubtypes,
    },
};

/// 原+18存储、+1d0蓝图、+200变换、+230子类型、+260延迟操作的拥有型快照。
/// 备份桶使用无索引的原tracked_clone_from<()>策略；活存储仍维护唯一d2s索引。
#[derive(Resource, Clone, Debug, Default)]
pub struct DecoratorBackup {
    pub records: Vec<DeferredDecoratorRecord>,
    pub storage_backup: DecoratorStorage,
    pub blueprints: DecoratorBlueprints,
    pub transforms: CachedDecoratorTransforms,
    pub subtypes: DerivedDecoratorSubtypes,
    pub events: DeferredDecoratorOpEvents,
    pub reset_after: bool,
    pub created: bool,
    pub state: u8,
}
impl DecoratorBackup {
    /// 原140823660/140824680：清空拥有值，释放旧Arc，复用可复用的外层分配。
    pub fn clear(&mut self) {
        self.storage_backup.per_wall.clear();
        self.storage_backup.terrain = None;
        self.storage_backup.stairs = None;
        self.storage_backup.removed.clear();
        self.storage_backup.decorator_to_storage = Default::default();
        self.storage_backup.immutable = false;
        self.blueprints.map.clear();
        self.transforms.storage.clone_from(&Default::default());
        self.subtypes.clear();
        self.events.clone_from(&Default::default());
        self.records.clear();
        self.reset_after = false;
        self.created = false;
        self.state = 0;
    }
    /// 原行46的内联consume_after_processing，由释放/取消输入请求帧尾清除。
    pub fn consume_after_processing(&mut self) {
        self.reset_after = true;
    }
}

/// 原九参顺序与140489a80系统构造一致；不把三个真实map降为占位字节。
#[allow(clippy::too_many_arguments)]
pub fn create_or_restore_decorator_backup<C: Send + Sync + 'static>(
    archivist: Res<DecoratorArchivist<C>>,
    mut storage: ResMut<DecoratorStorage>,
    mut subtypes: ResMut<DerivedDecoratorSubtypes>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut transforms: ResMut<CachedDecoratorTransforms>,
    mut events: ResMut<DeferredDecoratorOpEvents>,
    mut deferred: ResMut<DeferredDecoratorEditHistory>,
    mut backup: ResMut<DecoratorBackup>,
    mut stairs: ResMut<StairsState>,
) {
    // 原先置state=0；中途失败不会重试旧请求，也不会执行帧尾reset。
    let state = std::mem::take(&mut backup.state);
    if state == 1 {
        for (address, _) in archivist.all_storages() {
            // 原先创建目标备份桶，再读活桶并unwrap，失败顺序必须保留。
            let target = backup.storage_backup.get_or_insert_default(address);
            let source = storage.get(address).expect("备份装饰目标活存储不存在");
            clone_bucket_without_index(target, source);
        }
        backup.subtypes.clone_from(&subtypes);
        backup.blueprints.map.clone_from(&blueprints.map);
        backup.transforms.storage.clone_from(&transforms.storage);
        backup.events.clone_from(&events);
        backup.records.clone_from(&deferred.records);
        backup.created = true;
    } else if state != 0 && backup.created {
        let clear_preview = deferred.pending;
        // 原第一遍先把archivist的所有活桶清掉，移除相应d2s并登记removed。
        // 不能只覆盖备份中已有的桶，否则本轮新建的预览会残留。
        for (address, _) in archivist.all_storages() {
            clear_live_bucket(&mut storage, address);
        }
        for (address, original) in archivist.all_storages() {
            assert!(storage.get(address).is_some(), "恢复装饰目标活存储不存在");
            if let Some(saved) = backup.storage_backup.get(address) {
                storage.tracked_clone_into(address, saved);
            } else {
                // 1408242ec/2ef CMOVNE：没有最近有效快照时，源是归档初始桶。
                storage.tracked_clone_into(address, &archived_bucket(original));
            }
            if clear_preview {
                let bucket = storage.get_mut(address).unwrap();
                // 原迭代整个map，只有state1才取得可写视图并标modified。
                for (&id, info) in &mut bucket.decorators {
                    if info.state == 1 {
                        info.state = 0;
                        bucket.modified_ids.insert(id);
                    }
                }
            }
        }
        subtypes.clone_from(&backup.subtypes);
        blueprints.map.clone_from(&backup.blueprints.map);
        transforms.storage.clone_from(&backup.transforms.storage);
        events.clone_from(&backup.events);
        deferred.records.clone_from(&backup.records);
        // pending与potentially_surviving集合不是原备份字段，不能一起覆盖。
        stairs.restore_backup();
    }
    if backup.reset_after {
        backup.clear();
    }
}

/// 原1412573b0无d2s策略。非空目标保留removed传播，不能直接赋值clone。
fn clone_bucket_without_index(target: &mut Decorators, source: &Decorators) {
    if target.removed_ids.is_empty()
        && target.modified_ids.is_empty()
        && target.decorators.is_empty()
    {
        target.modified_ids.clone_from(&source.modified_ids);
        target.removed_ids.clone_from(&source.removed_ids);
        target.decorators.clone_from(&source.decorators);
    } else {
        target.modified_ids.clear();
        target.removed_ids.extend(target.decorators.keys().copied());
        target.decorators.clear();
        target
            .removed_ids
            .extend(source.removed_ids.iter().copied());
        for (&id, &info) in &source.decorators {
            target.decorators.insert(id, info);
            target.removed_ids.remove(&id);
            target.modified_ids.insert(id);
        }
    }
}

/// 原141257900 destructive clear；不同于Storage帧末非破坏性的清tracking接口。
fn clear_live_bucket(storage: &mut DecoratorStorage, address: DecoratorStorageAddr) {
    assert!(!storage.immutable, "assertion failed: !self.immutable");
    let DecoratorStorage {
        per_wall,
        terrain,
        stairs,
        decorator_to_storage,
        ..
    } = storage;
    let target = match address {
        DecoratorStorageAddr::Wall(wall) => per_wall.get_mut(&wall),
        DecoratorStorageAddr::Terrain => terrain.as_mut(),
        DecoratorStorageAddr::Stair => stairs.as_mut(),
    };
    let Some(target) = target else {
        return;
    };
    target.modified_ids.clear();
    target.removed_ids.extend(target.decorators.keys().copied());
    for (id, info) in target.decorators.drain() {
        if info.state == 2 {
            break;
        } // 原Option迭代niche，不是合法活装饰。
        decorator_to_storage
            .remove(DecoratorId(id))
            .expect("清除装饰备份桶时d2s缺失");
    }
}
fn archived_bucket(source: &TrackedDecorators) -> Decorators {
    Decorators {
        removed_ids: source.removed.iter().copied().collect(),
        decorators: source
            .items
            .iter()
            .map(|(&id, info)| (id, crate::apply_decorator_op::bytes_to_info(&info.0)))
            .collect(),
        modified_ids: source.added.iter().copied().collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;
    use country_core::resources::walls::{
        WallId,
        cached_decorator_transforms::CachedDecoratorTransform,
        decorator::{DecoratorInfo, DecoratorLocator},
        decorator_archivist::changes::ObservedDecoratorInfo,
        decorator_subtype::DecoratorSubtype,
        publication_fields::WallId as ArchivedWallId,
    };
    use glam::{Quat, Vec3};
    use std::sync::Arc;

    fn world() -> World {
        let mut world = World::new();
        world.init_resource::<DecoratorArchivist<()>>();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<DerivedDecoratorSubtypes>();
        world.init_resource::<DecoratorBlueprints>();
        world.init_resource::<CachedDecoratorTransforms>();
        world.init_resource::<DeferredDecoratorOpEvents>();
        world.init_resource::<DeferredDecoratorEditHistory>();
        world.init_resource::<DecoratorBackup>();
        world.insert_resource(StairsState::new());
        world
    }
    fn info(value: u32, state: u8) -> DecoratorInfo {
        DecoratorInfo {
            u38: value,
            state,
            u40: 6 << 8,
            ..Default::default()
        }
    }
    fn put(world: &mut World, address: DecoratorStorageAddr, id: u32, info: DecoratorInfo) {
        let mut storage = world.resource_mut::<DecoratorStorage>();
        storage
            .get_or_insert_default(address)
            .insert(DecoratorId(id), info);
        if storage.decorator_to_storage.get(DecoratorId(id)).is_none() {
            storage
                .decorator_to_storage
                .insert(DecoratorId(id), address);
        }
    }
    fn archive(world: &mut World, address: DecoratorStorageAddr) {
        let source = world.resource::<DecoratorStorage>().get(address).unwrap();
        let tracked = TrackedDecorators {
            removed: source.removed_ids.iter().copied().collect(),
            added: source.modified_ids.iter().copied().collect(),
            items: source
                .decorators
                .iter()
                .map(|(&id, &info)| {
                    (
                        id,
                        ObservedDecoratorInfo(crate::apply_decorator_op::info_to_bytes(info)),
                    )
                })
                .collect(),
        };
        let mut archivist = world.resource_mut::<DecoratorArchivist<()>>();
        match address {
            DecoratorStorageAddr::Wall(wall) => {
                archivist.walls.insert(ArchivedWallId(wall.0), tracked);
            }
            DecoratorStorageAddr::Terrain => archivist.special1 = Some(tracked),
            DecoratorStorageAddr::Stair => archivist.special2 = Some(tracked),
        }
    }
    fn run(world: &mut World, state: u8) {
        world.resource_mut::<DecoratorBackup>().state = state;
        world
            .run_system_once(create_or_restore_decorator_backup::<()>)
            .unwrap();
    }
    fn transform(x: f32) -> CachedDecoratorTransform {
        CachedDecoratorTransform {
            rotation: Quat::IDENTITY,
            translation: Vec3::new(x, 2.0, 3.0),
            scale: Vec3::ONE,
        }
    }
    fn event(tag: u64, pattern: u8) -> [u8; 0x88] {
        let mut event = [pattern; 0x88];
        event[..8].copy_from_slice(&tag.to_le_bytes());
        event
    }
    fn queued(events: &DeferredDecoratorOpEvents) -> Vec<[u8; 0x88]> {
        let mut output = Vec::new();
        events.clone().unleash(&mut output);
        output
            .into_iter()
            .map(|e| e[..0x88].try_into().unwrap())
            .collect()
    }
    fn set_derived(world: &mut World, value: u8) {
        world
            .resource_mut::<DerivedDecoratorSubtypes>()
            .insert(7, DecoratorSubtype(value));
        world
            .resource_mut::<DecoratorBlueprints>()
            .insert_for(7, DecoratorStorageAddr::Terrain, value)
            .u50 = value as u64;
        world
            .resource_mut::<CachedDecoratorTransforms>()
            .insert(7, transform(value as f32));
        let mut source = DeferredDecoratorOpEvents::default();
        source.send(event(1, value));
        world
            .resource_mut::<DeferredDecoratorOpEvents>()
            .clone_from(&source);
        world.resource_mut::<DeferredDecoratorEditHistory>().records =
            vec![DeferredDecoratorRecord {
                kind: 0,
                f1: value as u64,
                f2: 0x1234,
                f3: 0xabcd,
            }];
    }

    #[test]
    fn backup_nine_owners_restore_nonempty_buckets_maps_queue_history_stairs_and_arc_lifetimes() {
        let mut world = world();
        let addresses = [
            DecoratorStorageAddr::Wall(WallId(10)),
            DecoratorStorageAddr::Terrain,
            DecoratorStorageAddr::Stair,
        ];
        for (i, address) in addresses.into_iter().enumerate() {
            put(&mut world, address, 10 + i as u32, info(100, 0));
            archive(&mut world, address);
            put(&mut world, address, 10 + i as u32, info(200, 1));
        }
        let untouched = DecoratorStorageAddr::Wall(WallId(99));
        put(&mut world, untouched, 99, info(900, 0));
        set_derived(&mut world, 3);
        world.resource_mut::<StairsState>().add_edge(1, 2);
        world.resource_mut::<StairsState>().make_backup();
        let valid = Arc::downgrade(&world.resource::<DecoratorBlueprints>().map[&7]);
        run(&mut world, 1);
        assert!(world.resource::<DecoratorBackup>().created);
        assert_eq!(
            world
                .resource::<DecoratorBackup>()
                .storage_backup
                .per_wall
                .len(),
            1
        );
        assert!(
            world
                .resource::<DecoratorBackup>()
                .storage_backup
                .decorator_to_storage
                .is_empty()
        );
        assert_eq!(valid.strong_count(), 2);
        for (i, address) in addresses.into_iter().enumerate() {
            put(&mut world, address, 10 + i as u32, info(999, 0));
        }
        put(&mut world, addresses[0], 88, info(888, 0));
        put(&mut world, untouched, 99, info(901, 0));
        set_derived(&mut world, 9);
        let rejected = Arc::downgrade(&world.resource::<DecoratorBlueprints>().map[&7]);
        world.resource_mut::<DeferredDecoratorEditHistory>().pending = true;
        let locator = DecoratorLocator::new(untouched, 99);
        world
            .resource_mut::<DeferredDecoratorEditHistory>()
            .add_potentially_surviving_affected_decorator(locator);
        world.resource_mut::<StairsState>().add_edge(2, 3);
        run(&mut world, 2);
        for (i, address) in addresses.into_iter().enumerate() {
            let storage = world.resource::<DecoratorStorage>();
            assert_eq!(
                *storage
                    .get(address)
                    .unwrap()
                    .get(DecoratorId(10 + i as u32))
                    .unwrap(),
                info(200, 0)
            );
            assert_eq!(
                storage.lookup_storage(DecoratorId(10 + i as u32)),
                Some(address)
            );
        }
        assert!(
            world
                .resource::<DecoratorStorage>()
                .lookup_storage(DecoratorId(88))
                .is_none()
        );
        assert_eq!(
            world
                .resource::<DecoratorStorage>()
                .get(untouched)
                .unwrap()
                .get(DecoratorId(99))
                .unwrap()
                .u38,
            901
        );
        assert_eq!(
            world.resource::<DerivedDecoratorSubtypes>().get(7),
            DecoratorSubtype(3)
        );
        assert_eq!(
            world.resource::<CachedDecoratorTransforms>().transform(7),
            transform(3.0)
        );
        assert_eq!(world.resource::<DecoratorBlueprints>().map[&7].u50, 3);
        assert!(rejected.upgrade().is_none());
        assert_eq!(valid.strong_count(), 2);
        assert_eq!(
            queued(world.resource::<DeferredDecoratorOpEvents>()),
            vec![event(1, 3)]
        );
        let deferred = world.resource::<DeferredDecoratorEditHistory>();
        assert_eq!(deferred.records[0].f1, 3);
        assert!(
            deferred.pending
                && deferred
                    .potentially_surviving_affected_decorators
                    .contains_key(&locator)
        );
        assert!(world.resource::<StairsState>().contains_edge_or_stub(1, 2));
        assert!(!world.resource::<StairsState>().contains_edge_or_stub(2, 3));
        world
            .resource_mut::<DecoratorBackup>()
            .consume_after_processing();
        run(&mut world, 0);
        assert!(!world.resource::<DecoratorBackup>().created);
        assert_eq!(valid.strong_count(), 1);
        world.resource_mut::<DecoratorBlueprints>().map.clear();
        assert!(valid.upgrade().is_none());
    }

    #[test]
    fn backup_repeated_create_retains_native_removed_tracking_and_missing_saved_bucket_uses_archive()
     {
        let mut world = world();
        let old = DecoratorStorageAddr::Wall(WallId(1));
        put(&mut world, old, 1, info(10, 0));
        archive(&mut world, old);
        run(&mut world, 1);
        {
            let mut storage = world.resource_mut::<DecoratorStorage>();
            storage.get_mut(old).unwrap().remove(DecoratorId(1));
            storage.decorator_to_storage.remove(DecoratorId(1));
        }
        put(&mut world, old, 2, info(20, 0));
        world
            .resource_mut::<DecoratorStorage>()
            .get_mut(old)
            .unwrap()
            .modified_ids
            .clear();
        run(&mut world, 1);
        let saved = world
            .resource::<DecoratorBackup>()
            .storage_backup
            .get(old)
            .unwrap();
        assert!(saved.removed_ids.contains(&1));
        assert!(saved.modified_ids.contains(&2));
        let late = DecoratorStorageAddr::Wall(WallId(2));
        put(&mut world, late, 3, info(30, 1));
        archive(&mut world, late);
        put(&mut world, late, 3, info(999, 0));
        // 原state!=1的所有非零值都进入RESTORE，created仍为真。
        run(&mut world, 255);
        assert_eq!(
            *world
                .resource::<DecoratorStorage>()
                .get(late)
                .unwrap()
                .get(DecoratorId(3))
                .unwrap(),
            info(30, 1)
        );
        assert_eq!(
            *world
                .resource::<DecoratorStorage>()
                .get(old)
                .unwrap()
                .get(DecoratorId(2))
                .unwrap(),
            info(20, 0)
        );
    }

    #[test]
    fn backup_failure_consumes_request_before_partial_create_and_skips_reset_and_derived_copy() {
        let mut world = world();
        let address = DecoratorStorageAddr::Wall(WallId(5));
        world
            .resource_mut::<DecoratorArchivist<()>>()
            .walls
            .insert(ArchivedWallId(5), TrackedDecorators::default());
        set_derived(&mut world, 3);
        world
            .resource_mut::<DecoratorBackup>()
            .subtypes
            .insert(7, DecoratorSubtype(8));
        world.resource_mut::<DecoratorBackup>().reset_after = true;
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(&mut world, 1))).is_err()
        );
        let backup = world.resource::<DecoratorBackup>();
        assert_eq!(backup.state, 0);
        assert!(!backup.created);
        assert!(backup.reset_after);
        assert!(backup.storage_backup.get(address).is_some());
        assert_eq!(backup.subtypes.get(7), DecoratorSubtype(8));
    }

    #[test]
    fn backup_real_validator_rejection_restores_previous_frame_in_all_nine_owners() {
        use crate::{
            audio::op_preview_audio::MutePreviewDecoratorOpSounds,
            ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher,
        };
        use country_core::{
            resources::mouse_terrain_raycast::CursorPositionSS,
            resources::walls::{
                decorator_subtype::DecoratorMeshLibrary, public_walls::PublicWalls,
            },
            systems::{
                collision::world::RaycastWorld, ui::signifiers::UiSignifierStream,
                wall::structure_outline::OutlineWallCmd,
            },
        };
        let mut world = world();
        let address = DecoratorStorageAddr::Terrain;
        let valid_info = DecoratorInfo {
            u20: 3 << 16,
            ty: 0x3e,
            rank: 1,
            ..info(20, 0)
        };
        put(&mut world, address, 7, valid_info);
        archive(&mut world, address);
        set_derived(&mut world, 25);
        world
            .resource_mut::<DeferredDecoratorEditHistory>()
            .add_potentially_surviving_affected_decorator(DecoratorLocator::new(address, 7));
        world.init_resource::<DecoratorMeshLibrary>();
        world.init_resource::<PublicWalls>();
        world.insert_resource(RaycastWorld::new());
        world.init_resource::<UiSignifierStream>();
        world.insert_resource(CursorPositionSS(glam::Vec2::new(17.0, 23.0)));
        world.insert_resource(DecoratorAffordanceDispatcher {
            state: 0,
            payload: 0,
            ty: 0x38,
            interaction: None,
        });
        world.init_resource::<Events<OutlineWallCmd>>();
        world.init_resource::<Events<MutePreviewDecoratorOpSounds>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                crate::blueprint::validate_blueprints,
                create_or_restore_decorator_backup::<()>,
            )
                .chain(),
        );
        world.resource_mut::<DecoratorBackup>().state = 1;
        schedule.run(&mut world);
        assert!(world.resource::<DecoratorBackup>().created);
        let valid = Arc::downgrade(&world.resource::<DecoratorBlueprints>().map[&7]);
        put(&mut world, address, 7, info(999, 1));
        set_derived(&mut world, 9);
        world.resource_mut::<DecoratorBlueprints>().map.clear();
        world.resource_mut::<StairsState>().add_node(33);
        // 缺蓝图由真实validator拒绝并置RESTORE；测试不直接写state2。
        schedule.run(&mut world);
        assert_eq!(
            world
                .resource::<Events<MutePreviewDecoratorOpSounds>>()
                .len(),
            1
        );
        assert_eq!(
            *world
                .resource::<DecoratorStorage>()
                .get(address)
                .unwrap()
                .get(DecoratorId(7))
                .unwrap(),
            valid_info
        );
        assert_eq!(
            world.resource::<DerivedDecoratorSubtypes>().get(7),
            DecoratorSubtype(25)
        );
        assert_eq!(
            world.resource::<CachedDecoratorTransforms>().transform(7),
            transform(25.0)
        );
        assert_eq!(
            queued(world.resource::<DeferredDecoratorOpEvents>()),
            vec![event(1, 25)]
        );
        assert_eq!(
            world.resource::<DeferredDecoratorEditHistory>().records[0].f1,
            25
        );
        assert!(
            !world
                .resource::<StairsState>()
                .contains_edge_or_stub(33, 33)
        );
        assert!(valid.upgrade().is_some());
    }
    #[derive(Debug)]
    struct NativeBackupCase {
        state: u8,
        created: bool,
        reset: bool,
        pending: bool,
        saved: bool,
        missing_live: bool,
        panic: bool,
        flags: [u8; 3],
        live_maps: [u64; 3],
        backup_maps: [u64; 3],
        info_value: u32,
        info_state: u8,
        live_event: u8,
        backup_event: Option<u8>,
        live_record: u64,
        backup_record: Option<u64>,
        restored_stairs: bool,
    }
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../reports/module-recovery-2026-10-23/m01-backup-native-cases.rs"
    ));
    fn records(value: u64) -> Vec<DeferredDecoratorRecord> {
        (0..2)
            .map(|kind| DeferredDecoratorRecord {
                kind,
                f1: value,
                f2: 0x1234,
                f3: 0xabcd,
            })
            .collect()
    }
    #[test]
    fn backup_128_original_control_flow_cases_match_nine_real_resource_outputs() {
        assert_eq!(NATIVE_BACKUP_CASES.len(), 128);
        for c in NATIVE_BACKUP_CASES {
            let mut world = world();
            let address = DecoratorStorageAddr::Wall(WallId(7));
            if !c.missing_live {
                put(&mut world, address, 7, info(3, 1));
            }
            let mut original = TrackedDecorators::default();
            original.items.insert(
                7,
                ObservedDecoratorInfo(crate::apply_decorator_op::info_to_bytes(info(4, 1))),
            );
            world
                .resource_mut::<DecoratorArchivist<()>>()
                .walls
                .insert(ArchivedWallId(7), original);
            set_derived(&mut world, 3);
            world.resource_mut::<DeferredDecoratorEditHistory>().records = records(3);
            world.resource_mut::<DeferredDecoratorEditHistory>().pending = c.pending;
            world.resource_mut::<StairsState>().add_node(33);
            {
                let mut backup = world.resource_mut::<DecoratorBackup>();
                backup.subtypes.insert(7, DecoratorSubtype(8));
                backup.blueprints.insert_for(7, address, 8).u50 = 8;
                backup.transforms.insert(7, transform(8.0));
                backup.events.send(event(1, 8));
                backup.records = records(8);
                if c.saved {
                    backup
                        .storage_backup
                        .get_or_insert_default(address)
                        .insert(DecoratorId(7), info(8, 1));
                }
                backup.created = c.created;
                backup.reset_after = c.reset;
            }
            let outcome =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(&mut world, c.state)));
            assert_eq!(outcome.is_err(), c.panic, "{c:?}");
            let backup = world.resource::<DecoratorBackup>();
            assert_eq!(
                [backup.reset_after as u8, backup.created as u8, backup.state],
                c.flags,
                "{c:?}"
            );
            assert_eq!(
                [
                    world.resource::<DerivedDecoratorSubtypes>().get(7).0 as u64,
                    world.resource::<DecoratorBlueprints>().map[&7].u50,
                    world
                        .resource::<CachedDecoratorTransforms>()
                        .transform(7)
                        .translation
                        .x as u64
                ],
                c.live_maps,
                "{c:?}"
            );
            if c.backup_maps[0] != 0 {
                assert_eq!(
                    [
                        backup.subtypes.get(7).0 as u64,
                        backup.blueprints.map[&7].u50,
                        backup.transforms.transform(7).translation.x as u64
                    ],
                    c.backup_maps,
                    "{c:?}"
                );
            } else {
                assert!(backup.blueprints.map.is_empty());
                assert!(backup.transforms.try_transform(7).is_none());
            }
            if !c.missing_live {
                let got = world
                    .resource::<DecoratorStorage>()
                    .get(address)
                    .unwrap()
                    .get(DecoratorId(7))
                    .unwrap();
                assert_eq!((got.u38, got.state), (c.info_value, c.info_state), "{c:?}");
            }
            assert_eq!(
                queued(world.resource::<DeferredDecoratorOpEvents>()),
                vec![event(1, c.live_event)],
                "{c:?}"
            );
            assert_eq!(
                queued(&backup.events),
                c.backup_event
                    .into_iter()
                    .map(|v| event(1, v))
                    .collect::<Vec<_>>(),
                "{c:?}"
            );
            assert_eq!(
                world.resource::<DeferredDecoratorEditHistory>().records,
                records(c.live_record),
                "{c:?}"
            );
            assert_eq!(
                backup.records,
                c.backup_record.map(records).unwrap_or_default(),
                "{c:?}"
            );
            assert_eq!(
                !world
                    .resource::<StairsState>()
                    .contains_edge_or_stub(33, 33),
                c.restored_stairs,
                "{c:?}"
            );
        }
    }
    #[test]
    fn backup_restore_missing_d2s_fails_before_derived_copy_and_does_not_clear_request_owner() {
        let mut world = world();
        let address = DecoratorStorageAddr::Wall(WallId(5));
        put(&mut world, address, 5, info(20, 0));
        archive(&mut world, address);
        set_derived(&mut world, 3);
        run(&mut world, 1);
        world
            .resource_mut::<DecoratorStorage>()
            .decorator_to_storage
            .remove(DecoratorId(5));
        set_derived(&mut world, 9);
        world.resource_mut::<StairsState>().add_node(33);
        world.resource_mut::<DecoratorBackup>().reset_after = true;
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(&mut world, 2))).is_err()
        );
        let bucket = world.resource::<DecoratorStorage>().get(address).unwrap();
        assert!(bucket.decorators.is_empty() && bucket.removed_ids.contains(&5));
        assert_eq!(
            world.resource::<DerivedDecoratorSubtypes>().get(7),
            DecoratorSubtype(9)
        );
        assert!(
            world
                .resource::<StairsState>()
                .contains_edge_or_stub(33, 33)
        );
        let backup = world.resource::<DecoratorBackup>();
        assert_eq!(backup.state, 0);
        assert!(backup.created && backup.reset_after);
    }
}
