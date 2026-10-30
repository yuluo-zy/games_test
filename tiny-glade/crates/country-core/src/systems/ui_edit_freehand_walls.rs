//! B ui_edit_freehand_walls, 0x140ac98e0, with owned point factories and shared events.
use crate::resources::history::History;
use crate::resources::roofs::geometry::surface::Curve2;
use crate::resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData};
use crate::resources::walls::decorator_archivist::{
    DecoratorArchivalResources, LiveDecoratorArchivist,
};
use crate::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
use crate::resources::walls::{InnerWalls, WallId, public_walls::PublicWalls};
use crate::systems::ui::resources::ui_state::UiState;
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

#[derive(Default)]
pub struct EditFreehandCache {
    pub offsets: Vec<(u32, u32, f32)>,
    pub initial: Option<Curve2>,
    pub grab_delta: Vec2,
    pub selected_point: Vec2,
}

fn edit_point(curve: &Curve2, index: usize, target: Vec2, shape_tag: u32) -> Curve2 {
    let mut points = curve.points.clone();
    points[index] = target.to_array();
    if shape_tag != 2 {
        let last = points.len() - 1;
        if index == 0 { points[last] = target.to_array(); }
        else if index == last { points[0] = target.to_array(); }
    }
    Curve2::try_new(points, false).expect("原自由墙曲线重建失败")
}

fn adjacent_edges_valid(curve: &Curve2, index: usize, target: Vec2) -> bool {
    let valid = |neighbor: usize| curve.points.get(neighbor).is_none_or(|point| {
        let dx = point[0] - target.x;
        let dy = point[1] - target.y;
        !((dy * dy + dx * dx).sqrt() < 0.1)
    });
    valid(index + 1) && (index == 0 || valid(index - 1))
}

pub fn ui_edit_freehand_walls<S: Send + Sync + 'static>(
    (mode, public, mut inner, cursor, terrain, ui): (
        Res<crate::systems::mode_manager::AppMode>,
        Res<PublicWalls>,
        ResMut<InnerWalls>,
        Res<crate::resources::cursor_terrain_raycast::CursorTerrainRaycast>,
        Res<TerrainHeightsData<TerrainHeightSsbo>>,
        Res<UiState>,
    ),
    (mut cache, mut storage, mut archivist, archival, mut history, mut signifiers): (
        Local<EditFreehandCache>,
        ResMut<DecoratorStorage>,
        ResMut<LiveDecoratorArchivist>,
        DecoratorArchivalResources,
        ResMut<History<S>>,
        ResMut<crate::systems::ui::signifiers::signifier_create::UiSignifierStream>,
    ),
    (border, mut edited, mut trees, mut display, mut feedback, mut pins, mut audio_moved, mut audio): (
        Res<crate::resources::glade_border::GladeBorder>,
        EventWriter<crate::resources::events::OnFreehandWallEdited>,
        EventWriter<crate::resources::tree_storage::HideTreesNextToWallCmd>,
        EventWriter<crate::resources::glade_border::DisplayOutOfBorder>,
        EventWriter<crate::resources::glade_border::FeedbackInputOutOfBorder>,
        EventWriter<crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter>,
        EventReader<crate::resources::window::AudioCursorMoved>,
        EventWriter<crate::systems::audio::RequestWallMaterialAudio>,
    ),
) {
    use crate::systems::mode_manager::{AppMode, EditSubject};
    use crate::systems::ui::resources::widget_params::{WidgetParams, WidgetVolumeShape};
    let AppMode::Editing(EditSubject::Wall(raw)) = *mode else {
        return;
    };
    let wall = WallId(raw);
    let state = public.get(wall).expect("freehand edit wall missing");
    let curve = state.simplified_curve_2d();
    assert!(
        curve.points.len() >= 2 && curve.length > 0.0,
        "invalid freehand wall curve"
    );
    if state.shape().tag() < 2 {
        return;
    }
    for (index, point) in curve.points.iter().enumerate() {
        let point = Vec2::from_array(*point);
        let y = terrain.sample_at_world_xz(point.x, point.y);
        let mut params = WidgetParams::new(
            WidgetVolumeShape::sphere(Vec3::new(point.x, y, point.y), 0.75),
            [0; 2],
        );
        params.occlude_by_world = false;
        for (slot, kind) in params.sounds.slots.iter_mut().zip([1u32, 2, 3, 3]) {
            slot[..4].copy_from_slice(&kind.to_le_bytes());
        }
        let context = ui.add(&format!("edit_wall_{wall}_{index}"), params);
        let position = Vec3::new(point.x, y, point.y);
        let scale = if context.is_active() { 0.3f32 } else { 0.15f32 };
        let name = format!("wall_edit_pt_{position:?}{scale:?}");
        let mut factory =
            crate::systems::ui::signifiers::signifier_factory::SignifierFactory::from_closure(
                &name,
                std::sync::Arc::new(move |meshes, shaders, commands| {
                    use crate::systems::ui::signifiers::signifier_factory::{
                        SignifierEntityBuilder, SignifierMesh,
                    };
                    vec![
                        SignifierEntityBuilder::new(SignifierMesh::Named("temp_sphere".into()))
                            .position(position)
                            .scale(scale)
                            .animation_duration(0.0)
                            .custom_shader("vertex_color_shader")
                            .build(meshes, shaders, commands),
                    ]
                }),
            );
        factory.marker = 0x142ae0940;
        signifiers.write_factory(&factory);
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
                    "edit_freehand_wall",
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
                            0 | 1 => {
                                let pos = Vec2::new(
                                    f32::from_bits((info.dst_payload >> 32) as u32),
                                    f32::from_bits((info.u18 >> 32) as u32),
                                );
                                let coord =
                                    ::utils::curve::approx_curve_coord(&curve.points, pos, |p| {
                                        Vec2::from_array(*p)
                                    });
                                cache.offsets.push((id, coord.segment as u32, coord.t));
                            }
                            2 => {}
                            _ => panic!("invalid freehand edit decorator destination"),
                        }
                    }
                }
                cache.initial = Some(curve.clone());
                cache.selected_point = point;
                let pos = cursor.position();
                cache.grab_delta = Vec2::new(pos.x, pos.z) - point;
            }
            2 => {
                let current = &inner
                    .get(wall)
                    .expect("freehand edit authority missing")
                    .source()
                    .curve;
                let pos = cursor.position();
                let candidate = Vec2::new(pos.x, pos.z) - cache.grab_delta;
                let new_curve = edit_point(current, index, candidate, state.shape().tag());
                let too_close = !adjacent_edges_valid(&new_curve, index, candidate);
                let outside = new_curve
                    .points
                    .iter()
                    .any(|p| !border.is_pos_inside(Vec2::from_array(*p)));
                if outside {
                    display.write(crate::resources::glade_border::DisplayOutOfBorder);
                    feedback.write(crate::resources::glade_border::FeedbackInputOutOfBorder);
                }
                if !outside && !too_close {
                    edited.write(crate::resources::events::OnFreehandWallEdited {
                        offsets: cache.offsets.clone(),
                        wall,
                    });
                    inner.update_wall_state_curve(wall, new_curve);
                }
                trees.write(crate::resources::tree_storage::HideTreesNextToWallCmd {
                    wall,
                    finished: false,
                });
                pins.write(crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter::for_wall(wall));
                if outside || too_close { continue; }
            }
            3 => {
                use crate::resources::history::{
                    EditType,
                    wall_edit::{
                        WallCompoundHistoryEdit, WallCurveHistory, WallHistoryEdit, WallValueEdit,
                    },
                };
                edited.write(crate::resources::events::OnFreehandWallEdited {
                    offsets: cache.offsets.clone(),
                    wall,
                });
                if let Some(before) = cache.initial.take() {
                    let hist = |c: &Curve2| WallCurveHistory {
                        points: c.points.clone(),
                        points_u: c.points_u.clone(),
                        length: c.length,
                    };
                    let edit = WallHistoryEdit::MorphFreehandWall(WallValueEdit {
                        wall_index: wall.0,
                        before: hist(&before),
                        after: hist(curve),
                    });
                    history
                        .add(EditType::Wall(std::sync::Arc::new(
                            WallCompoundHistoryEdit {
                                groups: vec![vec![edit]],
                            },
                        )))
                        .expect("freehand edit history recording");
                }
                trees.write(crate::resources::tree_storage::HideTreesNextToWallCmd {
                    wall,
                    finished: true,
                });
            }
            _ => {}
        }
        if dragged != 2 && audio_moved.read().last().is_some() {
            audio.write(crate::systems::audio::RequestWallMaterialAudio {
                id: wall.0,
                position,
                kind: 0,
                flag: state.authority.source_flag_65().wrapping_mul(2),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_neighbor_threshold_fixture_matches_branch() {
        let cases: serde_json::Value = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../reports/module-recovery-2026-10-23/m01-freehand-native.json"
        )))
        .unwrap();
        let pair = |p: &serde_json::Value| {
            [
                f32::from_bits(p[0].as_u64().unwrap() as u32),
                f32::from_bits(p[1].as_u64().unwrap() as u32),
            ]
        };
        for case in cases["edges"].as_array().unwrap() {
            let target = Vec2::from_array(pair(&case["target"]));
            let curve = Curve2 {
                points: vec![target.to_array(), pair(&case["neighbor"])],
                points_u: vec![0., 1.],
                length: 1.,
            };
            assert_eq!(
                !adjacent_edges_valid(&curve, 0, target),
                case["rejected"].as_bool().unwrap()
            );
        }
    }
    #[test]
    fn closed_endpoints_and_neighbor_minimum_follow_original_branches() {
        let source = Curve2::try_new(vec![[0., 0.], [2., 0.], [2., 2.], [0., 0.]], false).unwrap();
        let first = edit_point(&source, 0, Vec2::new(1., 1.), 3);
        assert_eq!(first.points[0], first.points[3]);
        let last = edit_point(&source, 3, Vec2::new(1., 1.), 3);
        assert_eq!(last.points[0], last.points[3]);
        assert!(adjacent_edges_valid(&source, 0, Vec2::new(1.9, 0.)));
        assert!(!adjacent_edges_valid(&source, 0, Vec2::new(1.95, 0.)));
        let open = edit_point(&source, 0, Vec2::new(1., 1.), 2);
        assert_eq!(open.points[3], source.points[3]);
    }
    use crate::resources::walls::{
        WallState,
        public_walls::{WallPublicationDirty, WallSpatialHash},
        publication_source::TerrainHeightGrid512,
    };
    use crate::systems::mode_manager::{AppMode, EditSubject};
    use crate::systems::ui::resources::ui_state::WidgetId;

    #[test]
    fn real_picker_edits_a_freehand_point_and_records_history() {
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0], [8.0, 0.0]], false).unwrap();
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
        world.init_resource::<TerrainHeightsData<TerrainHeightSsbo>>();
        world.init_resource::<crate::resources::cursor_terrain_raycast::CursorTerrainRaycast>();
        world
            .init_resource::<crate::systems::ui::signifiers::signifier_create::UiSignifierStream>();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<crate::systems::collision::world::RaycastWorld>();
        #[cfg(feature = "native-physics")]
        world.init_resource::<crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
        world.insert_resource(History::<()>::after_observed_future_clear());
        world.insert_resource(crate::resources::glade_border::GladeBorder(
            crate::resources::glade_border::BorderShape::Rectangle(
                ::utils::geometry::Rectangle2d::new_centered_axis_aligned(Vec2::splat(100.0)),
            ),
        ));
        world.init_resource::<Events<crate::resources::events::OnFreehandWallEdited>>();
        world.init_resource::<Events<crate::resources::window::AudioCursorMoved>>();
        world.init_resource::<Events<crate::systems::audio::RequestWallMaterialAudio>>();
        world.init_resource::<Events<crate::resources::tree_storage::HideTreesNextToWallCmd>>();
        world.init_resource::<Events<crate::resources::glade_border::DisplayOutOfBorder>>();
        world.init_resource::<Events<crate::resources::glade_border::FeedbackInputOutOfBorder>>();
        world.init_resource::<crate::input::action_set::ActionSetStates>();
        let inverse = glam::Mat4::from_translation(Vec3::new(0.0, 0.0, 10.0));
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
        let mut producer = Schedule::default();
        producer.add_systems(ui_edit_freehand_walls::<()>);
        producer.run(&mut world);
        let entries = world
            .resource_mut::<crate::systems::ui::signifiers::signifier_create::UiSignifierStream>()
            .take_entries();
        assert_eq!(entries.len(), 3);
        let crate::systems::ui::signifiers::UiSignifierEntry::Factory(factory) = &entries[0] else {
            panic!("freehand point factory")
        };
        assert_eq!(factory.marker, 0x142ae0940);
        let meshes = crate::asset_libraries::mesh_library::AssetMeshLibrary::default();
        meshes.add(crate::asset_libraries::mesh_library::MeshAsset::new(
            crate::render::mesh::Mesh::new("sphere"),
            Some("temp_sphere".into()),
        ));
        let shaders = crate::asset_libraries::shader_library::AssetShaderLibrary::new();
        shaders.add_raster(
            crate::render::shader::RasterShaderProgram::new_draw(
                "vertex_color_shader",
                Default::default(),
                &crate::render::test_device::TestDevice(1),
            ),
            false,
        );
        let mut spawned = World::new();
        let mut queue = bevy_ecs::world::CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &spawned);
        let child = factory.wake(&meshes, &shaders, &mut commands)[0];
        drop(commands);
        queue.apply(&mut spawned);
        let transform = spawned
            .get::<crate::utils::components::transform::Transform>(child)
            .unwrap();
        assert_eq!(transform.scale, Vec3::ZERO);
        let animation = spawned
            .get::<crate::systems::ui::signifiers::animation::SignifierAnimation>(child)
            .unwrap();
        assert_eq!(
            (animation.flag_8, animation.flag_9, animation.duration),
            (0, 1, 0.0)
        );
        assert_eq!(
            spawned
                .get::<crate::systems::ui::signifiers::animation::SignifierAnimation>(child)
                .unwrap()
                .value,
            0.15
        );
        use bevy_ecs::system::RunSystemOnce;
        spawned.insert_resource(UiState::at_startup());
        spawned.init_resource::<crate::resources::time::GameTime>();
        spawned
            .run_system_once(crate::systems::ui::signifiers::animation::signifier_animation_t)
            .unwrap();
        spawned
            .run_system_once(crate::systems::ui::signifiers::animation::signifier_animation)
            .unwrap();
        assert_eq!(
            spawned
                .get::<crate::utils::components::transform::Transform>(child)
                .unwrap()
                .scale,
            Vec3::splat(0.15)
        );
        let widget = WidgetId::from_name(&format!("edit_wall_{wall}_0"));
        let center = world
            .resource::<UiState>()
            .widget(widget)
            .unwrap()
            .volume
            .center;
        let inverse = glam::Mat4::from_translation(center + Vec3::Z * 10.0);
        world
            .resource_mut::<crate::systems::camera_rig::main_camera::MainCamera>()
            .write_matrix(0x80, &inverse.inverse());
        world
            .resource_mut::<crate::systems::camera_rig::main_camera::MainCamera>()
            .write_matrix(0xc0, &inverse);
        let mut picker = Schedule::default();
        picker.add_systems(
            (
                crate::systems::ui::hover_intersection::hover_intersection,
                crate::systems::ui::ui_update::ui_update,
            )
                .chain_ignore_deferred(),
        );
        picker.run(&mut world);
        assert_eq!(world.resource::<UiState>().active_widget(), widget);
        producer.run(&mut world);
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
        *world.resource_mut::<crate::resources::glade_border::GladeBorder>() =
            crate::resources::glade_border::GladeBorder(
                crate::resources::glade_border::BorderShape::Rectangle(
                    ::utils::geometry::Rectangle2d::new_centered_axis_aligned(Vec2::splat(4.0)),
                ),
            );
        world
            .resource_mut::<crate::resources::cursor_terrain_raycast::CursorTerrainRaycast>()
            .update(Some(Vec3::new(3.95, 0.0, 0.0)));
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
            curve.points
        );
        assert_eq!(
            world
                .resource_mut::<Events<crate::resources::glade_border::DisplayOutOfBorder>>()
                .drain()
                .count(),
            1
        );
        assert_eq!(
            world
                .resource_mut::<Events<crate::resources::glade_border::FeedbackInputOutOfBorder>>()
                .drain()
                .count(),
            1
        );
        assert!(
            world
                .resource::<Events<crate::resources::events::OnFreehandWallEdited>>()
                .is_empty()
        );
        *world.resource_mut::<crate::resources::glade_border::GladeBorder>() =
            crate::resources::glade_border::GladeBorder(
                crate::resources::glade_border::BorderShape::Rectangle(
                    ::utils::geometry::Rectangle2d::new_centered_axis_aligned(Vec2::splat(100.0)),
                ),
            );
        world
            .resource_mut::<crate::resources::cursor_terrain_raycast::CursorTerrainRaycast>()
            .update(Some(Vec3::new(1.0, 0.0, 2.0)));
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
                .points[0],
            [1.0, 2.0]
        );
        assert_eq!(
            world
                .resource_mut::<Events<crate::resources::events::OnFreehandWallEdited>>()
                .drain()
                .count(),
            1
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
                .points[0],
            [1.0, 2.0]
        );
    }
}
