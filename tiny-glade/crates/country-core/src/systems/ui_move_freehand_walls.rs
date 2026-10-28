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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::walls::{
        WallState,
        public_walls::{WallPublicationDirty, WallSpatialHash},
        publication_source::TerrainHeightGrid512,
    };
    use crate::systems::mode_manager::{AppMode, EditSubject};
    use crate::systems::ui::resources::ui_state::WidgetId;

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
    let p = curve.pos_at_u(0.5);
    let midpoint = Vec2::new(p[0], p[1]);
    let tangent = curve.tangent_at_u(0.5);
    let original_normal = Vec2::new(-tangent[1], tangent[0]);
    let mut normal = original_normal;
    let eye = camera.eye_forward();
    if normal.dot(Vec2::new(eye.x, eye.z)) > 0.0 {
        normal = -normal;
    }
    let terrain_y = terrain.sample_at_world_xz(midpoint.x + normal.x, midpoint.y + normal.y);
    let anchor = Vec3::new(
        midpoint.x + normal.x,
        state.authority.elevation().max(terrain_y),
        midpoint.y + normal.y,
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
    match context.dragged().wrapping_sub(2).min(3) {
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
            if audio_cursor.read().count() != 0 {
                material.write(crate::systems::audio::RequestWallMaterialAudio {
                    id: wall.0,
                    position: anchor,
                    kind: 1,
                    flag: 0,
                });
            }
        }
        2 => {
            let mut candidate = inner
                .get(wall)
                .expect("freehand move authority missing")
                .source()
                .curve
                .clone();
            let first =
                Vec2::from_array(*candidate.points.first().expect("freehand move curve empty"));
            let pos = cursor.position();
            let delta = Vec2::new(pos.x, pos.z) - cache.grab_offset - first;
            for point in &mut candidate.points {
                *point = (Vec2::from_array(*point) + delta).to_array();
            }
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
                crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter::Wall {
                    wall,
                },
            );
            if inside && audio_cursor.read().count() != 0 {
                material.write(crate::systems::audio::RequestWallMaterialAudio {
                    id: wall.0,
                    position: anchor,
                    kind: 1,
                    flag: 0,
                });
            }
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
}
