//! B ui_move_freehand_walls, 0x1409781d0, with shared move and pin events.
use crate::resources::history::History;
use crate::resources::roofs::geometry::surface::Curve2;
use crate::resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData};
use crate::resources::walls::decorator_archivist::{
    DecoratorArchivalResources, LiveDecoratorArchivist,
};
use crate::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
use crate::resources::walls::{InnerWalls, WallId, public_walls::PublicWalls};
use crate::systems::ui::resources::ui_state::UiState;
use crate::systems::ui::signifiers::signifier_create::UiSignifierStream;
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

#[derive(Default)]
pub struct MoveFreehandCache {
    pub offsets: Vec<(u32, f32, f32)>,
    pub initial: Option<Curve2>,
    pub grab_offset: Vec2,
}

pub(crate) fn archive_freehand(
    archivist: &mut LiveDecoratorArchivist,
    storage: &DecoratorStorage,
    resources: &DecoratorArchivalResources<'_>,
    wall: WallId,
    reason: &str,
) {
    archivist.archive_current_decorators_in_storages_with_resources(
        &[DecoratorStorageAddr::Wall(wall), DecoratorStorageAddr::Terrain], storage, resources, reason,
    );
    archivist.decorator_cull_allow_wall(wall, storage);
    if let Some(terrain) = storage.get(DecoratorStorageAddr::Terrain) {
        archivist.culling.allowed_ids.extend(terrain.decorators().keys().copied());
    }
}

pub(crate) fn anchor_xz(info: &crate::resources::walls::decorator::DecoratorInfo) -> Vec2 {
    Vec2::new(f32::from_bits((info.dst_payload >> 32) as u32), f32::from_bits((info.u18 >> 32) as u32))
}

pub(crate) fn wall_anchor(info: &crate::resources::walls::decorator::DecoratorInfo) -> bool {
    match info.dst_tag() {
        2 => false,
        3 => panic!("自由墙装饰不能附着楼梯"),
        4 => unreachable!("自由墙装饰目的地无效"),
        _ => true,
    }
}

fn move_anchor(curve: &Curve2, eye: Vec3) -> (Vec2, Vec2, Vec2) {
    let midpoint = Vec2::from_array(curve.pos_at_u(0.5));
    let (segment, _) = curve.coord_at_u(0.5);
    let tangent = Vec2::from_array(curve.points[segment + 1]) - Vec2::from_array(curve.points[segment]);
    let inverse = 1.0 / (tangent.y * tangent.y + tangent.x * tangent.x).sqrt();
    let normal = Vec2::new(-tangent.y * inverse, tangent.x * inverse);
    let inverse_eye = 1.0 / (eye.z * eye.z + eye.x * eye.x).sqrt();
    let dot = inverse_eye * eye.z * normal.y + inverse_eye * eye.x * normal.x;
    let offset = if dot < 0.0 { normal } else { -normal };
    (midpoint, midpoint + offset, normal)
}

fn translate_curve(curve: &Curve2, cursor: Vec2, offset: Vec2) -> Curve2 {
    let first = curve.points.first().expect("自由墙曲线不能为空");
    let delta = Vec2::new((cursor.x - offset.x) - first[0], (cursor.y - offset.y) - first[1]);
    let mut result = curve.clone();
    translate_points(&mut result.points, delta);
    result
}

fn translate_points(points: &mut [[f32; 2]], delta: Vec2) {
    for point in points { point[0] += delta.x; point[1] += delta.y; }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_scalar_translation_fixture_matches_bits() {
        let cases: serde_json::Value = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"), "/../../reports/module-recovery-2026-10-23/m01-freehand-native.json"
        ))).unwrap();
        let read_pair = |pair: &serde_json::Value| [
            f32::from_bits(pair[0].as_u64().unwrap() as u32),
            f32::from_bits(pair[1].as_u64().unwrap() as u32),
        ];
        for case in cases["translations"].as_array().unwrap() {
            let mut points: Vec<[f32; 2]> = case["points"].as_array().unwrap().iter().map(read_pair).collect();
            translate_points(&mut points, Vec2::from_array(read_pair(&case["delta"])));
            let bits: Vec<u32> = points.iter().flatten().map(|x| x.to_bits()).collect();
            let expected: Vec<u32> = case["output"].as_array().unwrap().iter().map(|x| x.as_u64().unwrap() as u32).collect();
            assert_eq!(bits, expected);
        }
    }
    #[test]
    fn translation_keeps_original_cache_and_grab_offset() {
        let source = Curve2 { points: vec![[1., 2.], [3., 5.], [8., 7.]], points_u: vec![0., 0.271, 1.], length: 123.4 };
        let result = translate_curve(&source, Vec2::new(6., 9.), Vec2::new(2., 3.));
        assert_eq!(result.points, vec![[4., 6.], [6., 9.], [11., 11.]]);
        assert_eq!(result.points_u, source.points_u);
        assert_eq!(result.length.to_bits(), source.length.to_bits());
    }
    use crate::resources::walls::{
        WallState,
        public_walls::{WallPublicationDirty, WallSpatialHash},
        publication_source::TerrainHeightGrid512,
    };
    use crate::systems::mode_manager::{AppMode, EditSubject};
    use crate::systems::ui::resources::ui_state::WidgetId;

    #[test]
    fn real_world_move_edit_border_release_and_unload() {
        use crate::{
            input::action_set::ActionSetStates,
            resources::{
                cursor_terrain_raycast::CursorTerrainRaycast,
                events::{OnFreehandWallEdited, OnShapeMoved},
                glade_border::{DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder},
                history::{EditType, MaybePendingSnapshotId, wall_edit::WallHistoryEdit},
                tree_storage::HideTreesNextToWallCmd,
                window::AudioCursorMoved,
                walls::{
                    WallState, public_walls::WallPublicationDirty,
                    publication_source::TerrainHeightGrid512,
                },
            },
            systems::{
                audio::RequestWallMaterialAudio,
                camera_rig::MainCamera,
                ui::resources::ui_state::{UiInteraction, WidgetId},
                ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter,
                ui_edit_freehand_walls::ui_edit_freehand_walls,
                wall::wall_state::wall_spatial_hash::WallSpatialHash,
            },
        };
        let source = Curve2::try_new(vec![[0., 0.], [2., 0.], [3., 2.]], false).unwrap();
        let mut inner = InnerWalls::new();
        let wall = inner.new_wall(WallState::new(source.clone(), 4., 0, 0, false));
        let mut world = World::new();
        world.insert_resource(inner);
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
            pos: Vec3::ZERO,
            ..Default::default()
        });
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<crate::systems::collision::world::RaycastWorld>();
        #[cfg(feature="native-physics")]
        world.init_resource::<crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
        let mut history = History::<MaybePendingSnapshotId>::empty_glade();
        history.start_input_recording(0).unwrap();
        world.insert_resource(history);
        let mut heights = TerrainHeightsData::<TerrainHeightSsbo>::new();
        heights.set_height_range(0., 0.);
        world.insert_resource(heights);
        world.insert_resource(MainCamera::new(1.));
        world.init_resource::<ActionSetStates>();
        world.init_resource::<UiSignifierStream>();
        world.insert_resource(GladeBorder(
            crate::resources::glade_border::BorderShape::Rectangle(
                ::utils::geometry::Rectangle2d::new_centered_axis_aligned(Vec2::splat(130.)),
            ),
        ));
        world.init_resource::<Events<AudioCursorMoved>>();
        world.init_resource::<Events<RequestWallMaterialAudio>>();
        world.init_resource::<Events<DisplayOutOfBorder>>();
        world.init_resource::<Events<FeedbackInputOutOfBorder>>();
        world.init_resource::<Events<OnShapeMoved>>();
        world.init_resource::<Events<OnFreehandWallEdited>>();
        world.init_resource::<Events<HideTreesNextToWallCmd>>();
        world.init_resource::<Events<ShowPinnedDecoClutter>>();
        let widget = WidgetId::from_name(&format!("move_shape{wall}"));
        let interact = |world: &mut World, phase, widget| {
            let mut ui = UiState::new(UiInteraction {
                native_tag: 2,
                native_phase: phase,
                widget,
            });
            ui.set_intersection_tag(0);
            world.insert_resource(ui);
        };
        let mut movement = Schedule::default();
        movement.add_systems(ui_move_freehand_walls::<MaybePendingSnapshotId>);
        interact(&mut world, 3, widget);
        movement.run(&mut world);
        let registered = world.resource::<UiState>().widget(widget).unwrap();
        assert!(matches!(
            registered.geometry,
            Some(crate::systems::ui::resources::widget_params::WidgetGeometry::TriangleMesh(_))
        ));
        world.resource_mut::<CursorTerrainRaycast>().pos = Vec3::new(4., 0., 2.);
        interact(&mut world, 4, widget);
        movement.run(&mut world);
        let translated = world
            .resource::<InnerWalls>()
            .get(wall)
            .unwrap()
            .source()
            .curve
            .clone();
        assert_eq!(translated.points, vec![[4., 2.], [6., 2.], [7., 4.]]);
        assert_eq!(translated.points_u, source.points_u);
        publish(&mut world);
        world.resource_mut::<CursorTerrainRaycast>().pos = Vec3::new(200., 0., 200.);
        interact(&mut world, 4, widget);
        movement.run(&mut world);
        assert_eq!(
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .source()
                .curve
                .points,
            translated.points
        );
        assert_eq!(
            world.resource::<Events<FeedbackInputOutOfBorder>>().len(),
            1
        );
        interact(&mut world, 0, widget);
        movement.run(&mut world);
        assert_eq!(
            world.resource::<History>().pending().unwrap().edits.len(),
            1
        );
        // 同一World转入逐点编辑；Local跨帧持有真实起点与曲线，而不是注入编辑命令。
        let point_widget = WidgetId::from_name(&format!("edit_wall_{wall}_1"));
        let mut editing = Schedule::default();
        editing.add_systems(ui_edit_freehand_walls::<MaybePendingSnapshotId>);
        world.resource_mut::<CursorTerrainRaycast>().pos = Vec3::new(6., 0., 2.);
        interact(&mut world, 3, point_widget);
        editing.run(&mut world);
        assert_eq!(
            world
                .resource::<UiState>()
                .widget(point_widget)
                .unwrap()
                .volume
                .size
                .x,
            0.75
        );
        world.resource_mut::<CursorTerrainRaycast>().pos = Vec3::new(7., 0., 3.);
        interact(&mut world, 4, point_widget);
        editing.run(&mut world);
        assert_eq!(
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .source()
                .curve
                .points[1],
            [7., 3.]
        );
        publish(&mut world);
        interact(&mut world, 1, point_widget);
        editing.run(&mut world);
        let pending = world.resource::<History>().pending().unwrap();
        assert_eq!(pending.edits.len(), 2);
        let saved = serde_json::to_vec(&pending.edits).unwrap();
        let restored: Vec<EditType> = serde_json::from_slice(&saved).unwrap();
        assert_eq!(restored.len(), 2);
        let EditType::Wall(restored_wall) = &restored[1] else {
            panic!("自由墙历史恢复必须保留业务类型");
        };
        let WallHistoryEdit::MorphFreehandWall(change) = &restored_wall.groups[0][0] else {
            panic!("自由墙历史恢复必须保留曲线编辑");
        };
        let current_curve = &world
            .resource::<InnerWalls>()
            .get(wall)
            .unwrap()
            .source()
            .curve;
        assert_eq!(change.after.points, current_curve.points);
        assert_eq!(change.after.points_u, current_curve.points_u);
        assert_eq!(
            change.after.length.to_bits(),
            current_curve.length.to_bits()
        );
        for edit in &pending.edits {
            let EditType::Wall(edit) = edit else {
                panic!("必须记录墙复合编辑");
            };
            assert_eq!(edit.groups.len(), 1);
            assert!(matches!(
                edit.groups[0][0],
                WallHistoryEdit::MorphFreehandWall(_)
            ));
        }
        assert_eq!(world.resource::<Events<OnShapeMoved>>().len(), 2);
        assert_eq!(world.resource::<Events<OnFreehandWallEdited>>().len(), 2);
        let tree: Vec<_> = world
            .resource_mut::<Events<HideTreesNextToWallCmd>>()
            .drain()
            .collect();
        assert_eq!(tree.iter().filter(|e| e.finished).count(), 2);
        world.resource_mut::<UiSignifierStream>().take_entries();
        // 卸载World和两个Schedule会销毁Local曲线/闭包/归档，无裸指针借用残留。
        drop(editing);
        drop(movement);
        drop(world);
    }
    #[test]
    fn real_picker_moves_freehand_and_records_history() {
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let mut inner = InnerWalls::new();
        let wall = inner.new_wall_from_curve(WallState::new(curve.clone(), 4.0, 0, 0, false));
        inner.mark_as_non_enclosed(wall);
        let grid = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: true,
        };
        let mut public = PublicWalls::default();
        public.consume_with_paths(
            &mut inner,
            &grid,
            None,
            &WallPublicationDirty::default(),
            &WallSpatialHash::default(),
        );
        let mut world = World::new();
        world.init_resource::<Events<crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter>>();
        world.insert_resource(public);
        world.insert_resource(inner);
        world.insert_resource(AppMode::Editing(EditSubject::Wall(wall.0)));
        world.insert_resource(UiState::at_startup());
        world.init_resource::<UiSignifierStream>();
        world.init_resource::<TerrainHeightsData<TerrainHeightSsbo>>();
        world.init_resource::<crate::resources::cursor_terrain_raycast::CursorTerrainRaycast>();
        world.init_resource::<crate::input::action_set::ActionSetStates>();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<LiveDecoratorArchivist>();
        world
            .resource_mut::<DecoratorStorage>()
            .get_or_insert_default(DecoratorStorageAddr::Terrain)
            .insert(
                crate::resources::walls::decorator::DecoratorId(91),
                crate::resources::walls::decorator::DecoratorInfo::default(),
            );
        world.init_resource::<crate::systems::collision::world::RaycastWorld>();
        #[cfg(feature = "native-physics")]
        world.init_resource::<crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
        world.insert_resource(History::<()>::after_observed_future_clear());
        world.insert_resource(crate::resources::glade_border::GladeBorder(
            crate::resources::glade_border::BorderShape::Rectangle(
                ::utils::geometry::Rectangle2d::new_centered_axis_aligned(Vec2::splat(100.0)),
            ),
        ));
        world.init_resource::<Events<crate::resources::events::OnShapeMoved>>();
        world.init_resource::<Events<crate::resources::tree_storage::HideTreesNextToWallCmd>>();
        world.init_resource::<Events<crate::resources::glade_border::DisplayOutOfBorder>>();
        world.init_resource::<Events<crate::resources::glade_border::FeedbackInputOutOfBorder>>();
        world.init_resource::<Events<crate::resources::window::AudioCursorMoved>>();
        world.init_resource::<Events<crate::systems::audio::RequestWallMaterialAudio>>();
        world.insert_resource(crate::systems::camera_rig::main_camera::MainCamera::new(
            1.0,
        ));
        let mut producer = Schedule::default();
        producer.add_systems(ui_move_freehand_walls::<()>);
        producer.run(&mut world);
        let widget = WidgetId::from_name(&format!("move_shape{wall}"));
        let params = world.resource::<UiState>().widget(widget).unwrap();
        let Some(crate::systems::ui::resources::widget_params::WidgetGeometry::TriangleMesh(mesh)) =
            &params.geometry
        else {
            panic!("freehand gizmo mesh")
        };
        let center = mesh
            .iter()
            .flat_map(|t| t.points)
            .fold(Vec3::ZERO, |a, p| a + p)
            / (mesh.len() * 3) as f32;
        let inverse = glam::Mat4::from_translation(center + Vec3::Z * 10.0);
        let mut camera = crate::systems::camera_rig::main_camera::MainCamera::new(1.0);
        camera.write_matrix(0, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x40, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x80, &inverse.inverse());
        camera.write_matrix(0xc0, &inverse);
        world.insert_resource(camera);
        world.insert_resource(crate::resources::window::WindowSize::new(100, 100));
        world.insert_resource(crate::resources::mouse_terrain_raycast::CursorPositionSS(
            Vec2::splat(50.0),
        ));
        world.insert_resource(crate::resources::read_mouse_depth_value::MouseDepthValue {
            view_depth: 100.0,
            ..Default::default()
        });
        world.init_resource::<Events<crate::resources::window::WindowFocusEvent>>();
        world.init_resource::<Events<crate::resources::events::EscapeMenuJustOpened>>();
        world.init_resource::<Events<crate::resources::events::ShutdownGameCmd>>();
        world.init_resource::<Events<crate::systems::brush_preview::OverwriteBrushPreview>>();
        world.init_resource::<Events<crate::resources::mouse_capture::OsCursorVisibilityCmd>>();
        world.init_resource::<Events<crate::systems::ui::resources::widget_params::PlayUiSound>>();
        bevy_tasks::ComputeTaskPool::get_or_init(|| {
            bevy_tasks::TaskPoolBuilder::new().num_threads(2).build()
        });
        let mut picker = Schedule::default();
        picker.add_systems(
            (
                crate::systems::ui::hover_intersection::hover_intersection,
                crate::systems::ui::ui_update::ui_update,
            )
                .chain_ignore_deferred(),
        );
        picker.run(&mut world);
        producer.run(&mut world);
        assert_eq!(world.resource::<UiState>().active_widget(), widget);
        world
            .resource_mut::<crate::input::action_set::ActionSetStates>()
            .digital[0]
            .phase = crate::input::action_set::DigitalActionPhase::JustPressed;
        picker.run(&mut world);
        let mut start = Schedule::default();
        start.add_systems(crate::systems::history::recording::history_start_recording::<()>);
        start.run(&mut world);
        producer.run(&mut world);
        assert!(
            world
                .resource::<LiveDecoratorArchivist>()
                .extract_before_after(DecoratorStorageAddr::Wall(wall))
                .is_some()
        );
        assert!(
            world
                .resource::<LiveDecoratorArchivist>()
                .is_decorator_cull_allowed(91)
        );
        world
            .resource_mut::<crate::resources::cursor_terrain_raycast::CursorTerrainRaycast>()
            .update(Some(Vec3::new(2.0, 0.0, 3.0)));
        world
            .resource_mut::<crate::input::action_set::ActionSetStates>()
            .digital[0]
            .phase = crate::input::action_set::DigitalActionPhase::Held;
        picker.run(&mut world);
        producer.run(&mut world);
        assert_eq!(
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .source()
                .curve
                .points,
            vec![[2.0, 3.0], [6.0, 3.0]]
        );
        world
            .resource_mut::<crate::resources::cursor_terrain_raycast::CursorTerrainRaycast>()
            .update(Some(Vec3::new(100.0, 0.0, 0.0)));
        world.send_event(crate::resources::window::AudioCursorMoved);
        picker.run(&mut world);
        producer.run(&mut world);
        assert_eq!(
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .source()
                .curve
                .points,
            vec![[2.0, 3.0], [6.0, 3.0]]
        );
        assert_eq!(
            world
                .resource_mut::<Events<crate::resources::glade_border::DisplayOutOfBorder>>()
                .drain()
                .count(),
            1
        );
        assert!(
            world
                .resource::<Events<crate::systems::audio::RequestWallMaterialAudio>>()
                .is_empty()
        );
        world.resource_scope(|world, mut public: Mut<PublicWalls>| {
            public.consume_with_paths(
                &mut world.resource_mut::<InnerWalls>(),
                &grid,
                None,
                &WallPublicationDirty::default(),
                &WallSpatialHash::default(),
            )
        });
        world
            .resource_mut::<crate::input::action_set::ActionSetStates>()
            .digital[0]
            .phase = crate::input::action_set::DigitalActionPhase::JustReleased;
        picker.run(&mut world);
        producer.run(&mut world);
        assert_eq!(
            world
                .resource::<History<()>>()
                .pending()
                .unwrap()
                .edits
                .len(),
            1
        );
        assert!(
            world
                .resource_mut::<Events<crate::resources::tree_storage::HideTreesNextToWallCmd>>()
                .drain()
                .any(|e| e.finished)
        );
        world.init_resource::<crate::resources::stairs::stairs_state::StairsState>();
        world.init_resource::<crate::resources::stairs::stairs_removed_supports::StairsRemovedSupports>();
        world
            .init_resource::<Events<crate::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>>();
        world.init_resource::<Events<crate::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>>();
        world
            .init_resource::<Events<crate::systems::history::recording::FinishInputRecordingCmd>>();
        world.init_resource::<Events<crate::systems::backend::input::input_record_replay::InputReplayCommand>>();
        let mut finish = Schedule::default();
        finish
            .add_systems(crate::systems::history::recording::history_finish_recording_system::<()>);
        finish.run(&mut world);
        world.insert_resource(
            crate::systems::wall::wall_state::publish_public_wall_state::WallPublicationTerrain(
                grid,
            ),
        );
        world.init_resource::<crate::systems::history::PendingHistoryCommands>();
        world.insert_resource(crate::resources::history::ReplayBuffer::new(1.0));
        world.init_resource::<crate::resources::time::GameTime>();
        world
            .resource_mut::<crate::resources::time::GameTime>()
            .delta = 1.0;
        world.init_resource::<Events<crate::systems::mode_manager::HistoryCommand>>();
        world.init_resource::<Events<crate::systems::history::history_process::DebugHistoryMsg>>();
        world.init_resource::<Events<crate::systems::history::history_process::NewReplayCommandEvent>>();
        world.init_resource::<Events<crate::systems::audio::PlayReplayAudio>>();
        world.init_resource::<Events<crate::systems::timelapse_replay::InterruptTimelapse>>();
        world.init_resource::<Events<crate::systems::history::HistoryReplayFailed>>();
        world
            .init_resource::<Events<crate::systems::history::replay_wall::WallHistoryReplayAudio>>(
            );
        world
            .init_resource::<Events<crate::systems::history::replay_wall::WallHistoryRestoreColor>>(
            );
        world.init_resource::<Events<crate::systems::history::ReplayBucketFinished>>();
        world
            .init_resource::<Events<crate::systems::wall::adjust_wall_height::AdjustWallHeightCmd>>(
            );
        world.init_resource::<crate::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>();
        let mut replay = Schedule::default();
        replay.add_systems(
            (
                crate::systems::history::history_process_system::<()>,
                crate::systems::history::replay_wall::replay_wall,
                crate::systems::history::pop_empty_replay_bucket,
            )
                .chain(),
        );
        world.send_event(crate::systems::mode_manager::HistoryCommand::UNDO);
        for _ in 0..400 {
            replay.run(&mut world);
        }
        assert_eq!(
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .source()
                .curve
                .points,
            curve.points
        );
        world.send_event(crate::systems::mode_manager::HistoryCommand::REDO);
        for _ in 0..400 {
            replay.run(&mut world);
        }
        assert_eq!(
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .source()
                .curve
                .points,
            vec![[2.0, 3.0], [6.0, 3.0]]
        );
    }
}

pub fn ui_move_freehand_walls<S: Send + Sync + 'static>(
    (mode, public, mut inner, camera, cursor, terrain, actions, ui): (
        Res<crate::systems::mode_manager::AppMode>,
        Res<PublicWalls>,
        ResMut<InnerWalls>,
        Res<crate::systems::camera_rig::main_camera::MainCamera>,
        Res<crate::resources::cursor_terrain_raycast::CursorTerrainRaycast>,
        Res<TerrainHeightsData<TerrainHeightSsbo>>,
        Res<crate::input::action_set::ActionSetStates>,
        Res<UiState>,
    ),
    (mut cache, mut storage, mut archivist, archival, mut signifiers, mut history): (
        Local<MoveFreehandCache>,
        ResMut<DecoratorStorage>,
        ResMut<LiveDecoratorArchivist>,
        DecoratorArchivalResources,
        ResMut<UiSignifierStream>,
        ResMut<History<S>>,
    ),
    (
        border,
        mut moved,
        mut trees,
        mut display,
        mut feedback,
        mut audio_cursor,
        mut material,
        mut pins,
    ): (
        Res<crate::resources::glade_border::GladeBorder>,
        EventWriter<crate::resources::events::OnShapeMoved>,
        EventWriter<crate::resources::tree_storage::HideTreesNextToWallCmd>,
        EventWriter<crate::resources::glade_border::DisplayOutOfBorder>,
        EventWriter<crate::resources::glade_border::FeedbackInputOutOfBorder>,
        EventReader<crate::resources::window::AudioCursorMoved>,
        EventWriter<crate::systems::audio::RequestWallMaterialAudio>,
        EventWriter<crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter>,
    ),
) {
    use crate::systems::mode_manager::{AppMode, EditSubject};
    let AppMode::Editing(EditSubject::Wall(raw)) = *mode else {
        return;
    };
    let wall = WallId(raw);
    let state = public.get(wall).expect("freehand move wall missing");
    let curve = state.simplified_curve_2d();
    assert!(
        curve.points.len() >= 2 && curve.length > 0.0,
        "invalid freehand wall curve"
    );
    if state.shape().tag() < 2 {
        return;
    }
    let (midpoint, handle, original_normal) = move_anchor(curve, camera.eye_forward());
    let terrain_y = terrain.sample_at_world_xz(handle.x, handle.y);
    let elevation = state.authority.elevation();
    let height = if terrain_y.is_nan() || elevation > terrain_y { elevation } else { terrain_y };
    let anchor = Vec3::new(
        handle.x,
        height,
        handle.y,
    );
    let rotation =
        ::utils::xz_projective_rotation_bad(Vec3::new(original_normal.x, 0.0, original_normal.y));
    let context = crate::systems::ui_move_shapes::move_gizmo_and_trigger_vol(
        anchor,
        rotation,
        &actions,
        &ui,
        wall,
        state,
        terrain_y,
        &mut signifiers,
    );
    let dragged = context.dragged();
    match dragged.wrapping_sub(2).min(3) {
        1 => {
            archivist.archive_current_decorators_in_storages_with_resources(
                &[
                    DecoratorStorageAddr::Wall(wall),
                    DecoratorStorageAddr::Terrain,
                ],
                &storage,
                &archival,
                "move_freehand_wall",
            );
            archivist.decorator_cull_allow_wall(wall, &storage);
            if let Some(terrain) = storage.get(DecoratorStorageAddr::Terrain) {
                archivist
                    .culling
                    .allowed_ids
                    .extend(terrain.decorators().keys().copied());
            }
            cache.offsets.clear();
            if let Some(bucket) = storage.wall_decorators_mut(wall) {
                for (id, info) in bucket.iter_maybe_mut() {
                    match info.dst_tag() {
                        0 | 1 => cache.offsets.push((
                            id,
                            f32::from_bits((info.dst_payload >> 32) as u32) - midpoint.x,
                            f32::from_bits((info.u18 >> 32) as u32) - midpoint.y,
                        )),
                        2 => {}
                        _ => panic!("invalid freehand wall decorator destination"),
                    }
                }
            }
            cache.initial = Some(curve.clone());
            let pos = cursor.position();
            cache.grab_offset = Vec2::new(pos.x, pos.z) - Vec2::from_array(curve.points[0]);
        }
        2 => {
            let current = &inner
                .get(wall)
                .expect("freehand move authority missing")
                .source()
                .curve;
            let pos = cursor.position();
            let candidate = translate_curve(current, Vec2::new(pos.x, pos.z), cache.grab_offset);
            let inside = candidate
                .points
                .iter()
                .all(|point| border.is_pos_inside(Vec2::from_array(*point)));
            if inside {
                moved.write(crate::resources::events::OnShapeMoved {
                    offsets: cache.offsets.clone(),
                    wall,
                    center: midpoint,
                });
                inner.update_wall_state_curve(wall, candidate);
            } else {
                display.write(crate::resources::glade_border::DisplayOutOfBorder);
                feedback.write(crate::resources::glade_border::FeedbackInputOutOfBorder);
            }
            trees.write(crate::resources::tree_storage::HideTreesNextToWallCmd {
                wall,
                finished: false,
            });
            pins.write(
                crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter::for_wall(wall),
            );
            if !inside { return; }
        }
        3 => {
            use crate::resources::history::{
                EditType,
                wall_edit::{
                    WallCompoundHistoryEdit, WallCurveHistory, WallHistoryEdit, WallValueEdit,
                },
            };
            moved.write(crate::resources::events::OnShapeMoved {
                offsets: cache.offsets.clone(),
                wall,
                center: midpoint,
            });
            if let Some(before) = cache.initial.take() {
                let history_curve = |curve: &Curve2| WallCurveHistory {
                    points: curve.points.clone(),
                    points_u: curve.points_u.clone(),
                    length: curve.length,
                };
                let edit = WallHistoryEdit::MorphFreehandWall(WallValueEdit {
                    wall_index: wall.0,
                    before: history_curve(&before),
                    after: history_curve(curve),
                });
                history
                    .add(EditType::Wall(std::sync::Arc::new(
                        WallCompoundHistoryEdit {
                            groups: vec![vec![edit]],
                        },
                    )))
                    .expect("freehand move history recording");
            }
            trees.write(crate::resources::tree_storage::HideTreesNextToWallCmd {
                wall,
                finished: true,
            });
        }
        _ => {}
    }
    if dragged != 2 && audio_cursor.read().last().is_some() {
        material.write(crate::systems::audio::RequestWallMaterialAudio {
            id: wall.0,
            position: anchor,
            kind: 1,
            flag: 0,
        });
    }
}
