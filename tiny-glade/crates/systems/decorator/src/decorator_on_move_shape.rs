//! `decorator_on_move_shape.rs` — 形状移动 → 装饰位置偏移（0x140811fc0）。
//!
//! 证据：`toolchain/decor-trunk/decompiled/140811fc0_decorator_on_move_shape.c`
//! （O 全文）。
//!
//! 事件（O 布局）：
//! - 事件步长 0x30；+0x08 = 元素数组指针，+0x10 = 元素数，元素步长 12B
//!   `{id: u32, offset: Vec2}`；+0x20 = 事件级 `Vec2` 偏移。
//! - 每个元素：`modified_ids.insert(id)` → 查内嵌 map → side 公式；
//!   side 0（墙）→ 写 `Info+0x14..0x20 = xvy(elem.offset + event.offset)`；
//!   side 1（屋顶）→ 无操作；side 2/3 → panic（tag 3 panic()、tag 4
//!   panic_fmt unreachable）。
//!
//! 写面（O）：`*(u64*)(value+0x14) = vec.x/vec.y` + `*(u32*)(value+0x1c) = vec.z`
//! = 3 个 f32（anchor_ws Vec3）。
//!
//! 原RNE 140812227从Info+0x18加载XMM2，证实xvy保留既有anchor_ws.y。

use bevy_ecs::prelude::*;
use glam::Vec2;

use country_core::resources::walls::decorator::{DecoratorId, DecoratorInfo};
use country_core::resources::walls::decorator_storage::DecoratorStorage;

use crate::decorator_on_wall_height_changed::{get_anchor_ws, set_anchor_ws, side_from_tag_byte};

// 权威生产者与消费者共享同一事件，不增加原版没有的kind字段。
pub use country_core::resources::events::OnShapeMoved as MoveShapeEvent;

/// 位置写入（O：anchor_ws = xvy(elem.offset + event.offset)，y由原140812227直证保留）。
#[inline]
pub fn apply_move_shape_offset(info: &mut DecoratorInfo, element_offset: Vec2, event_offset: Vec2) {
    let delta = element_offset + event_offset;
    let y = get_anchor_ws(info).y;
    set_anchor_ws(info, glam::Vec3::new(delta.x, y, delta.y));
}

/// `decorator_on_move_shape` @0x140811fc0（O 结构）。
pub fn decorator_on_move_shape(
    mut storage: ResMut<DecoratorStorage>,
    mut events: EventReader<MoveShapeEvent>,
) {
    for event in events.read() {
        let Some(bucket) = storage.wall_decorators_mut(event.wall) else {
            continue;
        };
        // 原逐项标记后立即分派，避免失败时预先修改尚未访问的条目。
        for &(id, x, z) in &event.offsets {
            bucket.modified_ids.insert(id);
            let Some(info) = bucket.get_mut(DecoratorId(id)) else {
                continue;
            };
            let side = side_from_tag_byte(info.dst_tag());
            match side {
                0 => apply_move_shape_offset(info, Vec2::new(x, z), event.center),
                1 => {}
                2 => panic!(
                    "assertion failed: move_shape decorator {} has unsupported destination side 2",
                    id
                ),
                _ => unreachable!(
                    "internal error: entered unreachable code (decorator_on_move_shape.rs)"
                ),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn real_event_updates_wall_decorators_and_preserves_y() {
        use country_core::resources::walls::WallId;
        use country_core::resources::walls::decorator_storage::DecoratorStorageAddr;
        let wall = WallId(7);
        let mut world = World::new();
        let mut storage = DecoratorStorage::default();
        let mut info = DecoratorInfo::default();
        set_anchor_ws(&mut info, glam::Vec3::new(5.0, 3.0, 9.0));
        storage
            .get_or_insert_default(DecoratorStorageAddr::Wall(wall))
            .insert(DecoratorId(11), info);
        world.insert_resource(storage);
        world.init_resource::<Events<MoveShapeEvent>>();
        world
            .resource_mut::<Events<MoveShapeEvent>>()
            .send(MoveShapeEvent {
                offsets: vec![(11, 1.0, 2.0)],
                wall,
                center: Vec2::new(4.0, 5.0),
            });
        let mut schedule = Schedule::default();
        schedule.add_systems(decorator_on_move_shape);
        schedule.run(&mut world);
        let info = world
            .resource::<DecoratorStorage>()
            .get(DecoratorStorageAddr::Wall(wall))
            .unwrap()
            .get(DecoratorId(11))
            .unwrap();
        assert_eq!(get_anchor_ws(info), glam::Vec3::new(5.0, 3.0, 7.0));
    }

    use crate::decorator_on_wall_height_changed::{get_wall_coord, set_wall_coord};
    use utils::wall_space::WallSpaceCoord;

    #[test]
    fn freehand_real_ui_publication_history_and_both_consumers() {
        use country_core::{
            input::action_set::ActionSetStates,
            resources::{
                cursor_terrain_raycast::CursorTerrainRaycast,
                glade_border::{
                    BorderShape, DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder,
                },
                history::{History, MaybePendingSnapshotId},
                roofs::geometry::surface::Curve2,
                terrain_height::{TerrainHeightSsbo, TerrainHeightsData},
                tree_storage::HideTreesNextToWallCmd,
                walls::{
                    InnerWalls, WallId, WallState,
                    decorator_archivist::LiveDecoratorArchivist,
                    decorator_storage::DecoratorStorageAddr,
                    public_walls::{PublicWalls, WallPublicationDirty},
                    publication_source::TerrainHeightGrid512,
                },
                window::AudioCursorMoved,
            },
            systems::{
                audio::RequestWallMaterialAudio,
                camera_rig::MainCamera,
                mode_manager::{AppMode, EditSubject},
                ui::{
                    resources::ui_state::{UiInteraction, UiState, WidgetId},
                    signifiers::UiSignifierStream,
                },
                ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter,
                ui_edit_freehand_walls::ui_edit_freehand_walls,
                ui_move_freehand_walls::ui_move_freehand_walls,
                wall::wall_state::wall_spatial_hash::WallSpatialHash,
            },
        };
        let mut inner = InnerWalls::new();
        let wall = inner.new_wall(WallState::new(
            Curve2::try_new(vec![[0., 0.], [4., 0.]], false).unwrap(),
            4.,
            0,
            0,
            false,
        ));
        let mut storage = DecoratorStorage::default();
        let mut info = DecoratorInfo::default();
        info.dst_secondary = wall.0;
        info.ty = 0x38;
        info.u40 = 6 << 8;
        set_wall_coord(&mut info, WallSpaceCoord::new_unchecked(1., 3.));
        set_anchor_ws(&mut info, glam::Vec3::new(1., 3., 0.));
        storage
            .get_or_insert_default(DecoratorStorageAddr::Wall(wall))
            .insert(DecoratorId(7), info);
        storage
            .decorator_to_storage
            .insert(DecoratorId(7), DecoratorStorageAddr::Wall(wall));
        let mut world = World::new();
        world.insert_resource(inner);
        world.insert_resource(storage);
        world.init_resource::<PublicWalls>();
        fn publish(world: &mut World) {
            let mut inner = world.remove_resource::<InnerWalls>().unwrap();
            let terrain = TerrainHeightGrid512 {
                encoded: vec![0u16; 512 * 512].into(),
                minimum_y: 0.,
                maximum_y: 0.,
                sampling_fix: false,
            };
            world.resource_mut::<PublicWalls>().consume_with_paths(
                &mut inner,
                &terrain,
                None,
                &WallPublicationDirty::default(),
                &WallSpatialHash::default(),
            );
            world.insert_resource(inner);
        }
        publish(&mut world);
        world.insert_resource(AppMode::Editing(EditSubject::Wall(wall.0)));
        world.insert_resource(CursorTerrainRaycast {
            tag: 1,
            pos: glam::Vec3::ZERO,
            ..Default::default()
        });
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<country_core::systems::collision::world::RaycastWorld>();
        world.init_resource::<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
        let mut history = History::<MaybePendingSnapshotId>::empty_glade();
        history.start_input_recording(0).unwrap();
        world.insert_resource(history);
        let mut terrain = TerrainHeightsData::<TerrainHeightSsbo>::new();
        terrain.set_height_range(0., 0.);
        world.insert_resource(terrain);
        world.insert_resource(MainCamera::new(1.));
        world.init_resource::<ActionSetStates>();
        world.init_resource::<UiSignifierStream>();
        world.insert_resource(GladeBorder(BorderShape::Rectangle(
            utils::geometry::Rectangle2d::new_centered_axis_aligned(Vec2::splat(130.)),
        )));
        world.init_resource::<Events<AudioCursorMoved>>();
        world.init_resource::<Events<RequestWallMaterialAudio>>();
        world.init_resource::<Events<DisplayOutOfBorder>>();
        world.init_resource::<Events<FeedbackInputOutOfBorder>>();
        world.init_resource::<Events<country_core::resources::events::OnShapeMoved>>();
        world.init_resource::<Events<country_core::resources::events::OnFreehandWallEdited>>();
        world.init_resource::<Events<HideTreesNextToWallCmd>>();
        world.init_resource::<Events<ShowPinnedDecoClutter>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                ui_move_freehand_walls::<MaybePendingSnapshotId>,
                ui_edit_freehand_walls::<MaybePendingSnapshotId>,
                publish,
                decorator_on_move_shape,
                crate::decorator_on_freehand_wall_edited::decorator_on_freehand_wall_edited,
            )
                .chain(),
        );
        let interact = |world: &mut World, phase, widget, position| {
            let mut ui = UiState::new(UiInteraction {
                native_tag: 2,
                native_phase: phase,
                widget,
            });
            ui.set_intersection_tag(0);
            world.insert_resource(ui);
            world.resource_mut::<CursorTerrainRaycast>().pos = position;
        };
        let widget = WidgetId::from_name(&format!("move_shape{wall}"));
        for (phase, position) in [
            (3, glam::Vec3::ZERO),
            (4, glam::Vec3::new(3., 0., 2.)),
            (0, glam::Vec3::new(3., 0., 2.)),
        ] {
            interact(&mut world, phase, widget, position);
            schedule.run(&mut world);
            world.resource_mut::<UiSignifierStream>().take_entries();
        }
        let moved = world
            .resource::<DecoratorStorage>()
            .wall_decorators(wall)
            .unwrap()
            .get(DecoratorId(7))
            .unwrap();
        assert_eq!(get_anchor_ws(moved), glam::Vec3::new(4., 3., 2.));
        assert_eq!(get_wall_coord(moved), WallSpaceCoord::new_unchecked(1., 3.));
        world.resource_scope(|world, mut archivist: Mut<LiveDecoratorArchivist>| {
            drop(archivist.extract_diff(world.resource::<DecoratorStorage>(), false));
        });
        let widget = WidgetId::from_name(&format!("edit_wall_{wall}_1"));
        for (phase, position) in [
            (3, glam::Vec3::new(7., 0., 2.)),
            (4, glam::Vec3::new(9., 0., 4.)),
            (1, glam::Vec3::new(9., 0., 4.)),
        ] {
            interact(&mut world, phase, widget, position);
            schedule.run(&mut world);
            world.resource_mut::<UiSignifierStream>().take_entries();
        }
        let edited = world
            .resource::<DecoratorStorage>()
            .wall_decorators(wall)
            .unwrap()
            .get(DecoratorId(7))
            .unwrap();
        assert_eq!(get_anchor_ws(edited), glam::Vec3::new(4.5, 3., 2.5));
        assert_eq!(get_wall_coord(edited).y, 3.);
        assert_eq!(
            world.resource::<History>().pending().unwrap().edits.len(),
            2
        );
        world.resource_scope(|world, mut archivist: Mut<LiveDecoratorArchivist>| {
            let differences = archivist.extract_diff(world.resource::<DecoratorStorage>(), false);
            assert!(!differences.is_empty());
            assert!(archivist.walls.is_empty());
            assert!(archivist.special1.is_none());
        });
        drop(schedule);
        drop(world);
    }

    #[test]
    fn core_move_event_reaches_live_storage_in_order() {
        use bevy_ecs::system::RunSystemOnce;
        use country_core::resources::walls::{WallId, decorator_storage::DecoratorStorageAddr};
        let wall = WallId(8);
        let mut storage = DecoratorStorage::default();
        let mut info = DecoratorInfo::default();
        set_anchor_ws(&mut info, glam::Vec3::new(10., 3., 10.));
        storage
            .get_or_insert_default(DecoratorStorageAddr::Wall(wall))
            .insert(DecoratorId(7), info);
        let mut world = World::new();
        world.insert_resource(storage);
        world.init_resource::<Events<MoveShapeEvent>>();
        for offset in [Vec2::ZERO, Vec2::new(4., 5.)] {
            world.send_event(MoveShapeEvent {
                wall,
                center: offset,
                offsets: vec![(7, 1., 2.)],
            });
        }
        world.run_system_once(decorator_on_move_shape).unwrap();
        let bucket = world
            .resource::<DecoratorStorage>()
            .wall_decorators(wall)
            .unwrap();
        assert_eq!(
            get_anchor_ws(bucket.get(DecoratorId(7)).unwrap()),
            glam::Vec3::new(5., 3., 7.)
        );
        assert!(bucket.modified_ids.contains(&7));
        drop(world);
    }

    #[test]
    fn offset_adds_element_and_event_and_preserves_height() {
        let mut info = DecoratorInfo::default();
        set_anchor_ws(&mut info, glam::Vec3::new(10.0, 3.0, 10.0));
        apply_move_shape_offset(&mut info, Vec2::new(1.0, 2.0), Vec2::new(-0.5, 0.5));
        let anchor = get_anchor_ws(&info);
        // O（0x140811fc0）：写面 = xvy(elem.offset + event.offset)，**覆盖**为
        // 偏移和（不叠加旧锚）；y 保留原锚高（U：xvy 中间实参不可见）。
        assert_eq!(anchor, glam::Vec3::new(0.5, 3.0, 2.5));
    }

    #[test]
    fn move_shape_does_not_touch_wall_coord() {
        let mut info = DecoratorInfo::default();
        set_wall_coord(&mut info, WallSpaceCoord::new_unchecked(1.0, 2.0));
        apply_move_shape_offset(&mut info, Vec2::new(1.0, 1.0), Vec2::ZERO);
        assert_eq!(
            get_wall_coord(&info),
            WallSpaceCoord::new_unchecked(1.0, 2.0)
        );
    }

    #[test]
    fn side_classification_for_event_targets() {
        assert_eq!(side_from_tag_byte(0), 0);
        assert_eq!(side_from_tag_byte(2), 1);
        assert_eq!(side_from_tag_byte(3), 2);
        assert_eq!(side_from_tag_byte(4), 3);
    }
}
