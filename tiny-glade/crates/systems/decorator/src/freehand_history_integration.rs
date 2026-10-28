use bevy_ecs::prelude::*;
use country_core::resources::history::History;
use country_core::resources::roofs::geometry::surface::Curve2;
use country_core::resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData};
use country_core::resources::walls::{
    InnerWalls, WallState,
    decorator_archivist::LiveDecoratorArchivist,
    decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
    public_walls::{PublicWalls, WallPublicationDirty, WallSpatialHash},
    publication_source::TerrainHeightGrid512,
};
use country_core::systems::mode_manager::{AppMode, EditSubject};
use country_core::systems::ui::resources::ui_state::{UiState, WidgetId};
use glam::{Vec2, Vec3};

#[test]
fn nonempty_decorator_real_freehand_edit_and_history() {
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
    world.init_resource::<Events<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter>>();
    world.insert_resource(public);
    world.insert_resource(inner);
    world.insert_resource(AppMode::Editing(EditSubject::Wall(wall.0)));
    world.insert_resource(UiState::at_startup());
    world.init_resource::<TerrainHeightsData<TerrainHeightSsbo>>();
    world.init_resource::<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>();
    world.init_resource::<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>();
    world.init_resource::<DecoratorStorage>();
    world.init_resource::<LiveDecoratorArchivist>();
    world.init_resource::<country_core::systems::collision::world::RaycastWorld>();
    let mut deco = country_core::resources::walls::decorator::DecoratorInfo::default();
    crate::decorator_on_wall_height_changed::set_anchor_ws(&mut deco, Vec3::new(2.0, 3.0, 0.0));
    crate::decorator_on_wall_height_changed::set_wall_coord(
        &mut deco,
        utils::wall_space::WallSpaceCoord::new_unchecked(2.0, 3.0),
    );
    world
        .resource_mut::<DecoratorStorage>()
        .get_or_insert_default(DecoratorStorageAddr::Wall(wall))
        .insert(
            country_core::resources::walls::decorator::DecoratorId(9),
            deco,
        );
    world
        .resource_mut::<DecoratorStorage>()
        .decorator_to_storage
        .insert(
            country_core::resources::walls::decorator::DecoratorId(9),
            DecoratorStorageAddr::Wall(wall),
        );
    world.init_resource::<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
    world.insert_resource(History::<()>::after_observed_future_clear());
    world.insert_resource(country_core::resources::glade_border::GladeBorder(
        country_core::resources::glade_border::BorderShape::Rectangle(
            ::utils::geometry::Rectangle2d::new_centered_axis_aligned(Vec2::splat(100.0)),
        ),
    ));
    world.init_resource::<Events<country_core::resources::events::OnFreehandWallEdited>>();
    world.init_resource::<Events<country_core::resources::tree_storage::HideTreesNextToWallCmd>>();
    world.init_resource::<Events<country_core::resources::glade_border::DisplayOutOfBorder>>();
    world
        .init_resource::<Events<country_core::resources::glade_border::FeedbackInputOutOfBorder>>();
    world.init_resource::<country_core::input::action_set::ActionSetStates>();
    let inverse = glam::Mat4::from_translation(Vec3::new(0.0, 0.0, 10.0));
    let mut camera = country_core::systems::camera_rig::main_camera::MainCamera::new(1.0);
    camera.write_matrix(0, &glam::Mat4::IDENTITY);
    camera.write_matrix(0x40, &glam::Mat4::IDENTITY);
    camera.write_matrix(0x80, &inverse.inverse());
    camera.write_matrix(0xc0, &inverse);
    world.insert_resource(camera);
    world.insert_resource(country_core::resources::window::WindowSize::new(100, 100));
    world.insert_resource(
        country_core::resources::mouse_terrain_raycast::CursorPositionSS(Vec2::splat(50.0)),
    );
    world.insert_resource(
        country_core::resources::read_mouse_depth_value::MouseDepthValue {
            view_depth: 100.0,
            ..Default::default()
        },
    );
    world.init_resource::<Events<country_core::resources::window::WindowFocusEvent>>();
    world.init_resource::<Events<country_core::resources::events::EscapeMenuJustOpened>>();
    world.init_resource::<Events<country_core::resources::events::ShutdownGameCmd>>();
    world.init_resource::<Events<country_core::systems::brush_preview::OverwriteBrushPreview>>();
    world.init_resource::<Events<country_core::resources::mouse_capture::OsCursorVisibilityCmd>>();
    world
        .init_resource::<Events<country_core::systems::ui::resources::widget_params::PlayUiSound>>(
        );
    bevy_tasks::ComputeTaskPool::get_or_init(|| {
        bevy_tasks::TaskPoolBuilder::new().num_threads(2).build()
    });
    let mut producer = Schedule::default();
    producer
        .add_systems(country_core::systems::ui_edit_freehand_walls::ui_edit_freehand_walls::<()>);
    producer.run(&mut world);
    let widget = WidgetId::from_name(&format!("edit_wall_{wall}_0"));
    let center = world
        .resource::<UiState>()
        .widget(widget)
        .unwrap()
        .volume
        .center;
    let inverse = glam::Mat4::from_translation(center + Vec3::Z * 10.0);
    world
        .resource_mut::<country_core::systems::camera_rig::main_camera::MainCamera>()
        .write_matrix(0x80, &inverse.inverse());
    world
        .resource_mut::<country_core::systems::camera_rig::main_camera::MainCamera>()
        .write_matrix(0xc0, &inverse);
    let mut picker = Schedule::default();
    picker.add_systems(
        (
            country_core::systems::ui::hover_intersection::hover_intersection,
            country_core::systems::ui::ui_update::ui_update,
        )
            .chain_ignore_deferred(),
    );
    picker.run(&mut world);
    assert_eq!(world.resource::<UiState>().active_widget(), widget);
    producer.run(&mut world);
    world
        .resource_mut::<country_core::input::action_set::ActionSetStates>()
        .digital[0]
        .phase = country_core::input::action_set::DigitalActionPhase::JustPressed;
    picker.run(&mut world);
    let mut start = Schedule::default();
    start.add_systems(country_core::systems::history::recording::history_start_recording::<()>);
    start.run(&mut world);
    producer.run(&mut world);
    assert!(
        world
            .resource::<LiveDecoratorArchivist>()
            .extract_before_after(DecoratorStorageAddr::Wall(wall))
            .is_some()
    );
    *world.resource_mut::<country_core::resources::glade_border::GladeBorder>() =
        country_core::resources::glade_border::GladeBorder(
            country_core::resources::glade_border::BorderShape::Rectangle(
                ::utils::geometry::Rectangle2d::new_centered_axis_aligned(Vec2::splat(4.0)),
            ),
        );
    world
        .resource_mut::<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>()
        .update(Some(Vec3::new(3.95, 0.0, 0.0)));
    world
        .resource_mut::<country_core::input::action_set::ActionSetStates>()
        .digital[0]
        .phase = country_core::input::action_set::DigitalActionPhase::Held;
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
            .resource_mut::<Events<country_core::resources::glade_border::DisplayOutOfBorder>>()
            .drain()
            .count(),
        1
    );
    assert_eq!(world.resource_mut::<Events<country_core::resources::glade_border::FeedbackInputOutOfBorder>>().drain().count(),1);
    assert!(
        world
            .resource::<Events<country_core::resources::events::OnFreehandWallEdited>>()
            .is_empty()
    );
    *world.resource_mut::<country_core::resources::glade_border::GladeBorder>() =
        country_core::resources::glade_border::GladeBorder(
            country_core::resources::glade_border::BorderShape::Rectangle(
                ::utils::geometry::Rectangle2d::new_centered_axis_aligned(Vec2::splat(100.0)),
            ),
        );
    world
        .resource_mut::<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>()
        .update(Some(Vec3::new(1.0, 0.0, 2.0)));
    world
        .resource_mut::<country_core::input::action_set::ActionSetStates>()
        .digital[0]
        .phase = country_core::input::action_set::DigitalActionPhase::Held;
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
            .resource::<Events<country_core::resources::events::OnFreehandWallEdited>>()
            .len(),
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
    let mut consumer = Schedule::default();
    consumer
        .add_systems(crate::decorator_on_freehand_wall_edited::decorator_on_freehand_wall_edited);
    consumer.run(&mut world);
    let anchor = |world: &World| {
        crate::decorator_on_wall_height_changed::get_anchor_ws(
            world
                .resource::<DecoratorStorage>()
                .get(DecoratorStorageAddr::Wall(wall))
                .unwrap()
                .get(country_core::resources::walls::decorator::DecoratorId(9))
                .unwrap(),
        )
    };
    assert_eq!(anchor(&world), Vec3::new(2.5, 3.0, 1.0));
    world
        .resource_mut::<country_core::input::action_set::ActionSetStates>()
        .digital[0]
        .phase = country_core::input::action_set::DigitalActionPhase::JustReleased;
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
            .resource_mut::<Events<country_core::resources::tree_storage::HideTreesNextToWallCmd>>()
            .drain()
            .any(|e| e.finished)
    );
    world.init_resource::<country_core::resources::stairs::stairs_state::StairsState>();
    world.init_resource::<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>();
    world.init_resource::<Events<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>>();
    world.init_resource::<Events<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>>();
    world.init_resource::<Events<country_core::systems::history::recording::FinishInputRecordingCmd>>();
    world.init_resource::<Events<country_core::systems::backend::input::input_record_replay::InputReplayCommand>>();
    let mut finish = Schedule::default();
    finish.add_systems(
        country_core::systems::history::recording::history_finish_recording_system::<()>,
    );
    finish.run(&mut world);
    world.insert_resource(
        country_core::systems::wall::wall_state::publish_public_wall_state::WallPublicationTerrain(
            grid,
        ),
    );
    world.init_resource::<country_core::systems::history::PendingHistoryCommands>();
    world.insert_resource(country_core::resources::history::ReplayBuffer::new(1.0));
    world.init_resource::<country_core::resources::walls::decorator::DecoratorIdGen>();
    world.init_resource::<country_core::resources::time::GameTime>();
    world
        .resource_mut::<country_core::resources::time::GameTime>()
        .delta = 1.0;
    world.init_resource::<Events<country_core::systems::mode_manager::HistoryCommand>>();
    world
        .init_resource::<Events<country_core::systems::history::history_process::DebugHistoryMsg>>(
        );
    world.init_resource::<Events<country_core::systems::history::history_process::NewReplayCommandEvent>>();
    world.init_resource::<Events<country_core::systems::audio::PlayReplayAudio>>();
    world.init_resource::<Events<country_core::systems::timelapse_replay::InterruptTimelapse>>();
    world.init_resource::<Events<country_core::systems::history::HistoryReplayFailed>>();
    world.init_resource::<Events<country_core::systems::history::replay_wall::WallHistoryReplayAudio>>();
    world.init_resource::<Events<country_core::systems::history::replay_wall::WallHistoryRestoreColor>>();
    world.init_resource::<Events<country_core::systems::history::ReplayBucketFinished>>();
    world.init_resource::<Events<country_core::systems::wall::adjust_wall_height::AdjustWallHeightCmd>>();
    world.init_resource::<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>();
    let mut replay = Schedule::default();
    replay.add_systems(
        (
            (|mut storage: ResMut<DecoratorStorage>| storage.mark_mutable()),
            country_core::systems::history::history_process_system::<()>,
            country_core::systems::history::replay_wall::replay_wall,
            country_core::systems::history::replay_decorator::replay_decorator,
            country_core::systems::history::replay_pop_bucket_system,
        )
            .chain(),
    );
    world.send_event(country_core::systems::mode_manager::HistoryCommand::UNDO);
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
    assert_eq!(anchor(&world), Vec3::new(2.0, 3.0, 0.0));
    world.send_event(country_core::systems::mode_manager::HistoryCommand::REDO);
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
    assert_eq!(anchor(&world), Vec3::new(2.5, 3.0, 1.0));
}
