//! 树命令的权威消费链，B 142267430 / 1422686d0。
//!
//! 依次处理 Move、Insert、Delete。插入的主题来自 ActiveSession；
//! is_flowery/is_olden/is_winter 的 ICF 同址符号不能解释成 UI 或存档状态。
//! flags16 全零为原写面，mode 与声音仍保留主题、变体和插入布尔分支。

use bevy_ecs::entity::Entities;
use bevy_ecs::prelude::*;

use country_core::components::{TreeAnimation, TreeComponent, TreeMarkedForDeletion, TreeMoveBend};
use country_core::resources::history::tree_edit::{DeleteTreeCmd, InsertTreeCmd};
use country_core::resources::session::ActiveSession;
use country_core::resources::time_of_day::inverse_lerp;
use country_core::resources::tree_storage::{
    MoveTreeCmd, TreeStorage, TreeStorageEntry, TreeStorageIndex,
};
use country_core::resources::world_raster_defs::WaterRaster;
use country_core::systems::audio::PlayTreeSoundCmd;
use country_core::systems::audio::tree_audio::TreeAmbience;
use country_core::systems::prefab::PrefabInstanceData;
use country_core::systems::prefab::{DespawnPrefabCmd, SpawnPrefabCmd};

use crate::ui_place_tree::random_tree_prefab;

/// despawn α 阈值（O：0x142960674 = 0x3c23d70a = 0.01）。判等：despawn_trees
/// 反编译行 75/122 `*pf <= 0.01 && *pf != 0.01` —— **严格小于**（排除 NaN）。
pub const DESPAWN_ALPHA_THRESHOLD: f32 = 0.01;
pub use DESPAWN_ALPHA_THRESHOLD as DESPAWN_THRESHOLD;

/// Resolved marker/progress deletion rule (B 0x1422686d0).
pub fn should_despawn_tree(marked_for_deletion: bool, animation_value: f32) -> bool {
    marked_for_deletion && animation_value < DESPAWN_ALPHA_THRESHOLD
}

/// B 原码按 Move→Insert→Delete 顺序消费；存储、Prefab placement 和声音
/// 分别沿现有权威所有者提交。原 Query 的缺失组件失败不得用替代状态掩盖。
#[allow(clippy::too_many_arguments)]
pub fn process_tree_cmds<S: Send + Sync + 'static>(
    mut storage: ResMut<TreeStorage>,
    mut instances: Query<&mut PrefabInstanceData>,
    mut moves: EventReader<MoveTreeCmd>,
    mut deletes: EventReader<DeleteTreeCmd>,
    mut inserts: EventReader<InsertTreeCmd>,
    mut spawns: EventWriter<SpawnPrefabCmd>,
    mut commands: Commands,
    mut sounds: EventWriter<PlayTreeSoundCmd>,
    session: Res<ActiveSession>,
    water: Res<WaterRaster>,
    mut despawns: EventWriter<DespawnPrefabCmd>,
    entities: &Entities,
) {
    // ---- Move 分支（O：槽 stride 0x20；decomp 行 187-325）。wave3 落库。
    for cmd in moves.read() {
        if let Some(entry) = storage.get(TreeStorageIndex(cmd.index)) {
            // Original query is the actual 48-byte PrefabInstanceData placement.
            // Entity/component errors reach unwrap_failed @142268583.
            let mut instance = instances
                .get_mut(entry.entity)
                .expect("MoveTreeCmd entity has no PrefabInstanceData placement");
            instance.0.position = cmd.position;
            let size_ratio = inverse_lerp(0.5, 0.9, instance.0.scale);
            // O：水面采样（decomp 行 300-303）。结果 `0.0 < level` 是
            // PlayTreeSoundCmd selector@+9；value@+0xc = inverse_lerp 结果。
            let pos = cmd.position;
            let water_level = (**water)
                .as_ref()
                .bilinear_sample_at_world_xz(pos.x, pos.z, 130.0, 130.0);
            // O：PlayTreeSoundCmd kind=1（decomp 行 314-323 / asm
            // 1422678ab-1422678e3：kind@+8=1、selector@+9=in_water、value@+0xc）。
            sounds.write(PlayTreeSoundCmd {
                kind: 1,
                selector: (water_level > 0.0) as u8,
                extra: 0,
                value: size_ratio,
                position: pos,
            });
        } else {
            // O：`TreeStorage::get` None → 日志 "MoveTreeCmd references a
            // non-existing tree"（decomp 行 247-270，log level 门控）。
            log::warn!("MoveTreeCmd references a non-existing tree");
        }
    }
    // 原事件同时保留唯一存储索引、放置变体、旋转和布尔标志。
    for cmd in inserts.read() {
        let name = cmd.prefab_name.as_str();
        let naked = matches!(
            name,
            "clearing/naked_tree/large"
                | "clearing/naked_tree/naked1"
                | "clearing/naked_tree/naked2"
                | "clearing/naked_tree/naked1_no_mushrooms"
                | "clearing/naked_tree/naked2_no_mushrooms"
        );
        let is_olden = session.is_olden();
        let is_winter = session.is_winter();
        let flowery_extra = session.is_flowery() && (cmd.variant & 3) != 0;
        // B MAXSS(scale,.7) 只作用于 olden 裸树；NaN 时也选 .7。
        let scale = if naked && is_olden {
            cmd.scale.max(0.7)
        } else {
            cmd.scale
        };
        // O：动画 α = inverse_lerp(0.5, 0.9, scale)*0.75 + (1−r)*0.55
        // （常量 0x142925870=0.5、0x142a7c88c=0.9、0x1429edb90=0.75、
        // 0x142a877a0=0.55）。
        let r = inverse_lerp(0.5, 0.9, cmd.scale);
        let alpha = r * 0.75 + (1.0 - r) * 0.55;
        let pos = cmd.position;
        // O：`Commands::spawn_empty` + 逐条 insert（decomp 行 540-609，四段
        // 命令队列，顺序 TreeComponent → TreeAnimation → TreeMoveBend →
        // TreeAmbience）。
        let entity = commands.spawn_empty().id();
        // Original Insert payload bool is retained for replay and preplaced trees.
        commands.entity(entity).insert(TreeComponent {
            id: cmd.id,
            is_preplaced: cmd.is_preplaced,
        });
        commands.entity(entity).insert(TreeAnimation::new(alpha));
        // B：初始缓动位置等于树的世界 XZ 位置。
        commands.entity(entity).insert(TreeMoveBend {
            position: glam::Vec2::new(pos.x, pos.z),
        });
        // O：TreeAmbience 位置 = (x, y + scale*10, z)（常量 0x142a95db0 =
        // {0.0, 10.0} 广播乘 scale；decomp 行 487-503）。
        commands.entity(entity).insert(TreeAmbience {
            position: glam::Vec3::new(pos.x, pos.y + scale * 10.0, pos.z),
        });
        // 缺名回放与实时放置使用同一会话主题，不能读取加载 readiness。
        let prefab_name = if name == country_core::resources::history::tree_edit::missing_prefab() {
            random_tree_prefab(
                (cmd.id & 1) as u8,
                is_olden,
                session.meta.is_halloween_any(),
                is_winter,
                session.is_autumn(),
            )
            .to_string()
        } else {
            cmd.prefab_name.clone()
        };
        // B 142267dcd..ded：olden 裸树优先，其次插入标志，最后 flowery 变体。
        let mode = if naked && is_olden {
            3
        } else if cmd.is_preplaced {
            0
        } else if flowery_extra {
            2
        } else {
            1
        };
        spawns.write(SpawnPrefabCmd {
            prefab_name: Some(
                country_core::resources::render::mesh_atlas_library::PrefabName::new(&prefab_name),
            ),
            entity: Some(entity),
            position: pos,
            scale,
            rotation: cmd.rotation_degrees,
            variant: cmd.variant as u32,
            one: 1.0,
            flags: [0; 0x10],
            mode,
        });
        // O：`TreeStorage::insert`（0x140a38560）；Some(previous) → 日志 +
        // DespawnPrefabCmd{entity: previous.entity}（decomp 行 721 / 747-785；
        // 串 "Tree {} already exists. Replacing it with a new tree"）。
        let previous: Option<TreeStorageEntry> = storage.insert(
            TreeStorageIndex(cmd.id),
            TreeStorageEntry {
                entity,
                position: pos,
            },
        );
        // O：水面采样（decomp 行 722-726）→ `PlayTreeSoundCmd`：kind=0、
        // selector=0.0<水样、extra=naked && (olden||winter)、value=r、
        // position（decomp 行 727-746；slot stride 0x20 与共享型逐一对应）。
        let water_level = (**water)
            .as_ref()
            .bilinear_sample_at_world_xz(pos.x, pos.z, 130.0, 130.0);
        sounds.write(PlayTreeSoundCmd {
            kind: 0,
            selector: (water_level > 0.0) as u8,
            extra: (naked && (is_olden || is_winter)) as u8,
            value: r,
            position: pos,
        });
        // B 142267f72..142268141：先提交声音，随后才回收被替换的旧树。
        if let Some(previous) = previous {
            log::warn!(
                "Tree {} already exists. Replacing it with a new tree",
                cmd.id
            );
            despawns.write(DespawnPrefabCmd {
                entity: Some(previous.entity),
                _gap: 0,
            });
        }
    }
    // ---- Delete 分支（O：槽 stride 0x10；decomp 行 819-896）。wave3 补
    // `Entities::contains` 门控（行 835）。
    for cmd in deletes.read() {
        if let Some(entry) = storage.remove(TreeStorageIndex(cmd.id)) {
            // O：`Entities::contains(entity)` 门控（decomp 行 835）——实体仍存在
            // 才推入 TreeMarkedForDeletion；否则跳过（原以 Commands::entity 直接
            // 排队是近似，分歧 18，已判等修复）。
            if entities.contains(entry.entity) {
                commands.entity(entry.entity).insert(TreeMarkedForDeletion);
                // O：kind=2 + 树变换记录位置/scale（decomp 行 877-896：
                // inverse_lerp(0.5,0.9,scale) → kind@+8=2、value@+0xc、
                // position@+0x10/+0x18）。
                let instance = instances
                    .get(entry.entity)
                    .expect("DeleteTreeCmd live entity has no PrefabInstanceData placement");
                sounds.write(PlayTreeSoundCmd {
                    kind: 2,
                    selector: 0,
                    extra: 0,
                    value: inverse_lerp(0.5, 0.9, instance.0.scale),
                    position: instance.0.position,
                });
            }
        }
    }
}

/// `despawn_trees` @0x1422686d0（O：标记树 α<0.01 回收——`DespawnPrefabCmd`）。
pub fn despawn_trees(
    mut trees: Query<(Entity, &TreeAnimation, &TreeMarkedForDeletion)>,
    mut despawns: EventWriter<DespawnPrefabCmd>,
) {
    for (entity, animation, _marked) in trees.iter_mut() {
        // O：α 严格小于 0.01（`<=0.01 && !=0.01`，排除 NaN；decomp 行 75/122）。
        // 读取生长进度 +4；+0 是时长，不能用于删除判断。
        if should_despawn_tree(true, animation.progress) {
            despawns.write(DespawnPrefabCmd {
                entity: Some(entity),
                _gap: 0,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Original source test `sends_only_below_threshold` and unmarked query case.
    #[test]
    fn original_deletion_boundary_and_marker_contract() {
        for (value, expected) in [
            (0.0, true),
            (0.0099, true),
            (0.01, false),
            (0.5, false),
            (f32::NAN, false),
            (-1.0, true),
        ] {
            assert_eq!(should_despawn_tree(true, value), expected);
            assert!(!should_despawn_tree(false, value));
        }
    }
}

#[cfg(test)]
mod placement_consumer_tests {
    use super::*;
    use country_core::resources::{
        placement_data::PlacementDataSsbo,
        session::{SessionId, SessionMeta},
        world_raster::DirtyTilePublisher,
    };
    fn world() -> World {
        let mut world = World::new();
        world.init_resource::<TreeStorage>();
        world.init_resource::<Events<MoveTreeCmd>>();
        world.init_resource::<Events<DeleteTreeCmd>>();
        world.init_resource::<Events<InsertTreeCmd>>();
        world.init_resource::<Events<SpawnPrefabCmd>>();
        world.init_resource::<Events<DespawnPrefabCmd>>();
        world.init_resource::<Events<PlayTreeSoundCmd>>();
        world.insert_resource(ActiveSession::new(SessionId::new(), SessionMeta::default()));
        world.insert_resource(WaterRaster::new(DirtyTilePublisher::unique()));
        world
    }
    #[test]
    fn move_and_delete_use_the_real_prefab_placement_without_a_transform() {
        let mut world = world();
        let mut placement = PlacementDataSsbo::default();
        placement.position = glam::Vec3::new(1.0, 2.0, 3.0);
        placement.scale = 0.75;
        let entity = world.spawn(PrefabInstanceData(placement)).id();
        world.resource_mut::<TreeStorage>().insert(
            TreeStorageIndex(73),
            TreeStorageEntry {
                entity,
                position: placement.position,
            },
        );
        let target = glam::Vec3::new(8.0, 9.0, 10.0);
        world.send_event(MoveTreeCmd {
            index: 73,
            position: target,
        });
        let mut schedule = Schedule::default();
        schedule.add_systems(process_tree_cmds::<()>);
        schedule.run(&mut world);
        assert_eq!(
            world.get::<PrefabInstanceData>(entity).unwrap().0.position,
            target
        );
        let events = world.resource::<Events<PlayTreeSoundCmd>>();
        let mut cursor = events.get_cursor();
        let sound = cursor.read(events).next().unwrap();
        assert_eq!(sound.kind, 1);
        assert_eq!(sound.position, target);
        assert_eq!(sound.value, inverse_lerp(0.5, 0.9, 0.75));
        world.send_event(DeleteTreeCmd { id: 73 });
        schedule.run(&mut world);
        assert!(
            world
                .resource::<TreeStorage>()
                .get(TreeStorageIndex(73))
                .is_none()
        );
        assert!(world.get::<TreeMarkedForDeletion>(entity).is_some());
        let events = world.resource::<Events<PlayTreeSoundCmd>>();
        let mut cursor = events.get_cursor();
        let sound = cursor.read(events).last().unwrap();
        assert_eq!(sound.kind, 2);
        assert_eq!(sound.position, target);
        assert_eq!(sound.value, inverse_lerp(0.5, 0.9, 0.75));
    }
    #[test]
    fn same_frame_insert_then_delete_uses_new_identity_and_preserves_original_pending_query_failure()
     {
        let mut world = world();
        let mut placement = PlacementDataSsbo::default();
        placement.position = glam::Vec3::new(1.0, 2.0, 3.0);
        placement.scale = 0.75;
        let old = world.spawn(PrefabInstanceData(placement)).id();
        world.resource_mut::<TreeStorage>().insert(
            TreeStorageIndex(73),
            TreeStorageEntry {
                entity: old,
                position: placement.position,
            },
        );
        world.send_event(InsertTreeCmd {
            id: 73,
            position: glam::Vec3::new(8.0, 9.0, 10.0),
            scale: 0.75,
            rotation_degrees: 45.0,
            variant: -17,
            is_preplaced: true,
            prefab_name: "Oak".into(),
        });
        world.send_event(DeleteTreeCmd { id: 73 });
        let mut schedule = Schedule::default();
        schedule.add_systems(process_tree_cmds::<()>);
        // Native contains includes reserved entities; its following Query.get
        // unwrap fails until the actual upper prefab owner creates placement.
        let result =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| schedule.run(&mut world)));
        assert!(result.is_err());
        assert!(
            world
                .resource::<TreeStorage>()
                .get(TreeStorageIndex(73))
                .is_none()
        );
        let spawns = world.resource::<Events<SpawnPrefabCmd>>();
        let mut reader = spawns.get_cursor();
        let created = reader.read(spawns).next().unwrap().entity.unwrap();
        assert_ne!(created, old);
        let despawns = world.resource::<Events<DespawnPrefabCmd>>();
        let mut reader = despawns.get_cursor();
        assert_eq!(reader.read(despawns).next().unwrap().entity, Some(old));
        // The original queues the marker before its failing placement fetch.
        schedule.apply_deferred(&mut world);
        assert!(world.get::<TreeMarkedForDeletion>(created).is_some());
    }
    #[test]
    #[should_panic(expected = "MoveTreeCmd entity has no PrefabInstanceData placement")]
    fn live_move_entity_without_placement_preserves_original_unwrap_failure() {
        let mut world = world();
        let entity = world.spawn_empty().id();
        world.resource_mut::<TreeStorage>().insert(
            TreeStorageIndex(73),
            TreeStorageEntry {
                entity,
                position: glam::Vec3::ZERO,
            },
        );
        world.send_event(MoveTreeCmd {
            index: 73,
            position: glam::Vec3::ONE,
        });
        let mut schedule = Schedule::default();
        schedule.add_systems(process_tree_cmds::<()>);
        schedule.run(&mut world);
    }

    #[test]
    fn actual_insert_mode_uses_theme_variant_and_boolean_without_loading_readiness() {
        use country_core::resources::session::GladeType;
        for (theme, name, variant, is_preplaced, expected_scale, expected_mode, sound_extra) in [
            (GladeType::Summer, "Oak", 1, false, 0.6, 1, 0),
            (GladeType::Summer, "Oak", 1, true, 0.6, 0, 0),
            (GladeType::Flowery, "Oak", 1, false, 0.6, 2, 0),
            (GladeType::Flowery, "Oak", 4, false, 0.6, 1, 0),
            (GladeType::Flowery, "Oak", -17, false, 0.6, 2, 0),
            (GladeType::Flowery, "Oak", 1, true, 0.6, 0, 0),
            (GladeType::Olden, "clearing/naked_tree/naked1", 1, false, 0.7, 3, 1),
            (GladeType::Olden, "clearing/naked_tree/naked1", 1, true, 0.7, 3, 1),
            (GladeType::Winter, "clearing/naked_tree/naked1", 1, false, 0.6, 1, 1),
            (GladeType::Autumn, "clearing/naked_tree/naked1", 1, false, 0.6, 1, 0),
        ] {
            let mut world = world();
            world.resource_mut::<ActiveSession>().meta.glade = theme;
            world.send_event(InsertTreeCmd {
                id: 73,
                position: glam::Vec3::new(2.0, 3.0, 4.0),
                scale: 0.6,
                rotation_degrees: 45.0,
                variant,
                is_preplaced,
                prefab_name: name.into(),
            });
            let mut schedule = Schedule::default();
            schedule.add_systems(process_tree_cmds::<()>);
            schedule.run(&mut world);
            let events = world.resource::<Events<SpawnPrefabCmd>>();
            let mut cursor = events.get_cursor();
            let spawn = cursor.read(events).next().unwrap();
            assert_eq!(spawn.scale, expected_scale);
            assert_eq!(spawn.mode, expected_mode);
            assert_eq!(spawn.flags, [0; 16]);
            assert_eq!(spawn.rotation, 45.0);
            assert_eq!(spawn.variant, variant as u32);
            let entity = spawn.entity.unwrap();
            assert_eq!(world.resource::<TreeStorage>().get(TreeStorageIndex(73)).unwrap().entity, entity);
            assert_eq!(world.get::<TreeComponent>(entity).unwrap().is_preplaced, is_preplaced);
            assert_eq!(world.get::<TreeMoveBend>(entity).unwrap().position, glam::Vec2::new(2.0, 4.0));
            assert_eq!(world.get::<TreeAmbience>(entity).unwrap().position, glam::Vec3::new(2.0, 3.0 + expected_scale * 10.0, 4.0));
            let events = world.resource::<Events<PlayTreeSoundCmd>>();
            let mut cursor = events.get_cursor();
            let sound = cursor.read(events).next().unwrap();
            assert_eq!(sound.extra, sound_extra);
            assert_eq!(sound.value, inverse_lerp(0.5, 0.9, 0.6));
            assert_eq!(sound.position, glam::Vec3::new(2.0, 3.0, 4.0));
        }
    }

    #[test]
    fn missing_prefab_fallback_uses_session_and_keeps_original_name_audio_gate() {
        use country_core::resources::session::GladeType;
        for (theme, id, expected) in [
            (GladeType::Summer, 8, "clearing/tree/branchy2"),
            (GladeType::Olden, 8, "clearing/naked_tree/naked1"),
            (GladeType::Autumn, 9, "clearing/tree/autumn/branchy1"),
            (GladeType::Winter, 8, "clearing/naked_tree/naked1_no_mushrooms"),
            (GladeType::Winter, 9, "clearing/tree/branchy1"),
        ] {
            let mut world = world();
            world.resource_mut::<ActiveSession>().meta.glade = theme;
            world.send_event(InsertTreeCmd {
                id, position: glam::Vec3::ZERO, scale: 0.6,
                rotation_degrees: 90.0, variant: 2, is_preplaced: false,
                prefab_name: country_core::resources::history::tree_edit::missing_prefab().into(),
            });
            let mut schedule = Schedule::default();
            schedule.add_systems(process_tree_cmds::<()>);
            schedule.run(&mut world);
            let events = world.resource::<Events<SpawnPrefabCmd>>();
            let mut cursor = events.get_cursor();
            let spawn = cursor.read(events).next().unwrap();
            assert_eq!(spawn.prefab_name.as_ref().unwrap().as_name_ref().as_str(), expected);
            // 原版按事件旧名判断裸树，不按 fallback 后的 prefab 重算。
            assert_eq!(spawn.mode, 1);
            assert_eq!(spawn.scale, 0.6);
            let events = world.resource::<Events<PlayTreeSoundCmd>>();
            let mut cursor = events.get_cursor();
            assert_eq!(cursor.read(events).next().unwrap().extra, 0);
        }
    }

    #[test]
    fn olden_naked_scale_preserves_original_maxss_nan_result() {
        use country_core::resources::session::GladeType;
        let mut world = world();
        world.resource_mut::<ActiveSession>().meta.glade = GladeType::Olden;
        world.send_event(InsertTreeCmd {
            id: 1, position: glam::Vec3::ZERO, scale: f32::NAN,
            rotation_degrees: 0.0, variant: 0, is_preplaced: false,
            prefab_name: "clearing/naked_tree/naked1".into(),
        });
        let mut schedule = Schedule::default();
        schedule.add_systems(process_tree_cmds::<()>);
        schedule.run(&mut world);
        let events = world.resource::<Events<SpawnPrefabCmd>>();
        let mut cursor = events.get_cursor();
        assert_eq!(cursor.read(events).next().unwrap().scale, 0.7);
        let events = world.resource::<Events<PlayTreeSoundCmd>>();
        let mut cursor = events.get_cursor();
        assert!(cursor.read(events).next().unwrap().value.is_nan());
    }
}
