//! Register real Bevy event maintenance before resources-only domain helpers.
//! Bevy 0.16 App::add_event skips Events<T> that already exists. A helper's
//! init_resource<Events<T>> therefore cannot precede this App registration.
//! This list uses canonical domain identities and leaves all consumers intact.
use bevy_app::App;
use country_core::{resources as res, systems as sys};

pub(super) fn install(app: &mut App) {
    // Register before resources-only domain helpers so these channels receive
    // real per-frame maintenance in both native and disabled audio hosts.
    app.add_event::<sys::ui::resources::widget_params::PlayUiSound>();
    app.add_event::<sys::ui::resources::widget_params::PlayPhotomodeSound>();
    app.add_event::<res::events::InitAdjWallHeightMode>();
    app.add_event::<res::events::FinishAdjWallHeightMode>();
    // Snapshot apply must publish to the maintained channel before raster
    // resources are constructed, so the real staging consumer sees the load.
    app.add_event::<res::world_raster::UploadWorldRasterBuffersToStagingCmd>();
    // res::walls::public_walls
    app.add_event::<res::walls::public_walls::OnWallChanged>();
    app.add_event::<res::walls::public_walls::OnWallDeleted>();
    // 合并提示通道须先登记维护，兼容World初始化本身不注册事件轮换。
    app.add_event::<sys::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>();
    // 原墙构造与半木重建通道必须先注册维护，再进入World资源兼容入口。
    app.add_event::<res::walls::plaster::RecalculatePlasterCmd>();
    app.add_event::<res::walls::plaster::RecalculateHalfTimberMeshCmd>();
    app.add_event::<res::walls::plaster::RecalculateWoodenTurretCmd>();
    app.add_event::<system_wall_constructor::construct_elevation_supports::RecalculateRectangleSupports>();
    app.add_event::<system_wall_constructor::construct_elevation_supports::RecalculateCircleSupports>();
    // sys::wall::calc_wall_bend
    app.add_event::<sys::wall::calc_wall_bend::WallBendChangedEvent>();
    // sys::wall::construct_wall_bricks
    app.add_event::<sys::wall::construct_wall_bricks::WallBricksChanged>();
    // res::rt
    app.add_event::<res::rt::UpdateRtWorldEvent>();
    // sys::wall::adjust_wall_height
    app.add_event::<sys::wall::adjust_wall_height::AdjustWallHeightCmd>();
    app.add_event::<sys::wall::adjust_wall_height::WallHeightAudio>();
    app.add_event::<sys::wall::adjust_wall_height::WallHeightCommandFailed>();
    // sys::wall::snapping
    app.add_event::<sys::wall::snapping::DisableSnapping>();
    // sys::wall::eraser
    app.add_event::<sys::wall::eraser::AnimateWallBulldozeCmd>();
    app.add_event::<sys::wall::eraser::WallBulldozeAudio>();
    app.add_event::<sys::wall::eraser::WallEraserStroke>();
    // 原 add_audio_events（B 140941450）的 16 个通道属于游戏 App，
    // 不以 FMOD 是否启用为条件；工具和建筑系统仍会写入这些事件。
    // 必须先用 App 注册，使后续 World 初始化不能绕过每帧事件维护。
    app.add_event::<sys::terrain::PlayTerrainSound>();
    app.add_event::<sys::audio::SuppressPathSound>();
    app.add_event::<sys::audio::PlayReplayAudio>();
    app.add_event::<sys::audio::PlayDecoratorSoundCmd>();
    app.add_event::<sys::audio::PlayStairsAudioCmd>();
    app.add_event::<sys::audio::PlayWallUiSound>();
    app.add_event::<sys::audio::PlayWallAudio>();
    app.add_event::<sys::audio::RequestWallMaterialAudio>();
    app.add_event::<sys::audio::WallBendAudioParamReset>();
    app.add_event::<sys::audio::PlayTimberframeSound>();
    app.add_event::<sys::audio::PlaySheepReactSound>();
    app.add_event::<sys::audio::PlayMenuUiSound>();
    app.add_event::<sys::audio::PlayBuildingAudio>();
    app.add_event::<sys::audio::PlayElevationSupportSoundCmd>();
    app.add_event::<sys::audio::PlayClutterSoundCmd>();
    app.add_event::<sys::audio::PlayTreeSoundCmd>();
    // sys::wall::shapes
    app.add_event::<sys::wall::shapes::CreateCircleCmd>();
    app.add_event::<sys::wall::shapes::CreateRectangleCmd>();
    app.add_event::<sys::wall::shapes::EditRectangleCmd>();
    app.add_event::<sys::wall::shapes::FinishEditRectangleCmd>();
    app.add_event::<sys::wall::shapes::rectangle::OnRectangleEdited>();
    app.add_event::<sys::wall::shapes::rectangle::ui_edit_rectangle_dims::DoUiEditRectangleDims>();
    app.add_event::<sys::wall::shapes::rectangle::ui_edit_rectangle_orientation::DoUiEditRectangleRotate>();
    app.add_event::<res::events::OnShapeMoved>();
    app.add_event::<res::events::OnFreehandWallEdited>();
    app.add_event::<res::events::ReanchorDecorators>();
    app.add_event::<res::events::OnCircleRotated>();
    app.add_event::<res::events::LightLanternCmd>();
    // sys::wall::history_recording
    app.add_event::<sys::wall::history_recording::RecordWallHistory>();
    app.add_event::<sys::wall::history_recording::RecordColorHistory>();
    // sys::camera_rig::main_camera
    app.add_event::<sys::camera_rig::main_camera::MainCameraMoved>();
    // sys::wall::draw_wall
    app.add_event::<sys::wall::draw_wall::RecordMouseInputIntoWall>();
    // sys::wall::finish_wall
    app.add_event::<sys::wall::finish_wall::FinishWall>();
    // res::color_id
    app.add_event::<res::color_id::SetWallColorIdCmd>();
    app.add_event::<res::color_id::SetGateColorIdCmd>();
    app.add_event::<res::color_id::SetHalftimberColorIdCmd>();
    // res::tree_storage / system_tree / roof command events moved to TreePlugin and
    // RoofSystems (run_game::add_original_plugins), which own them in the original.
    // res::glade_border
    app.add_event::<res::glade_border::DisplayOutOfBorder>();
    app.add_event::<res::glade_border::FeedbackInputOutOfBorder>();
    app.add_event::<res::glade_border::NotInDemoHint>();
    // res::onboarding
    app.add_event::<res::onboarding::NotifyHintSystemToolWasJustUsed>();
    // 原墙擦除器的writer必须在World兼容初始化之前接入App事件维护。
    app.add_event::<res::onboarding::NotifyHintSystemSomethingWasErased>();
    // These are initialized by World-only helpers later in startup; register
    // them here first so Bevy also maintains their event buffers each frame.
    app.add_event::<res::critters::sheep::CmdCheckForSheepInsideBuilding>();
    app.add_event::<sys::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>();
    app.add_event::<sys::ivy::ivy_leaf_animation::IvyLeafIsGrowing>();
    app.add_event::<sys::ivy::ivy_pruner::RemoveIvyCmd>();
    // sys::history::history_process
    app.add_event::<sys::history::history_process::DebugHistoryMsg>();
    // res::roofs::commands
    app.add_event::<res::roofs::commands::SwitchRoofDirectionCmd>();
    app.add_event::<res::roofs::commands::SetFlatRoofStyleCmd>();
    app.add_event::<res::roofs::commands::RoofIsInvalid>();
    app.add_event::<res::roofs::commands::ResetRoofs>();
    app.add_event::<res::roofs::commands::WallRoofPresenceChanged>();
    app.add_event::<res::roofs::commands::WallGableChanged>();
    app.add_event::<res::roofs::commands::RecalculateWallHoles>();
    app.add_event::<res::roofs::commands::RecalculateWallBricks>();
    app.add_event::<res::roofs::commands::RoofEditFeedback>();
    app.add_event::<res::roofs::commands::ReplayRoofEvent>();
    // system_color
    app.add_event::<system_color::ShowColorMenuCmd>();
    app.add_event::<system_color::HideColorMenuCmd>();
    app.add_event::<system_color::SetRoofColorIdCmd>();
    app.add_event::<system_color::SetDecoratorColorIdCmd>();
    app.add_event::<system_color::SetDecoratorBrightnessCmd>();
    app.add_event::<system_color::SetLadderColorIdCmd>();
    app.add_event::<system_color::SetStairRailingCmd>();
    app.add_event::<system_color::SetMagicColorIdCmd>();
    app.add_event::<system_color::SetWindowLightCmd>();
    app.add_event::<system_color::SetFlagConnectionCmd>();
    app.add_event::<system_color::SetChimneySmokeCmd>();
    // sys::wall
    app.add_event::<sys::wall::SetFreehandHeightTypeCmd>();
    app.add_event::<sys::wall::SetPlasterPeelAmountCmd>();
    app.add_event::<sys::wall::SetWallStyleCmd>();
    // sys::ui::resources::widget_params
    app.add_event::<sys::ui::resources::widget_params::PlayUiSound>();
    // toolbar::ui_interaction
    app.add_event::<toolbar::ui_interaction::UiClickEvent>();
    app.add_event::<toolbar::ui_interaction::UiDragEvent>();
    app.add_event::<toolbar::ui_interaction::UiJustBecameInFocusEvent>();
    // toolbar::toolbar_button
    app.add_event::<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>();
    // res::events
    app.add_event::<res::events::ModeJustChanged>();
    // sys::mode_manager
    app.add_event::<sys::mode_manager::RequestToolbarCategoryChange>();
    app.add_event::<sys::mode_manager::RequestAppModeChange>();
    // sys::backend::input::calculate_input_delta
    app.add_event::<sys::backend::input::calculate_input_delta::UiHistoryButtonPressed>();
    // system_decorator::handle_external_decorator_cmds
    app.add_event::<system_decorator::handle_external_decorator_cmds::ExternalDecoratorCmd>();
    app.add_event::<sys::wall::eraser::ui_bulldozer::HandleStairAttachedDecoratorsOnStairPointDeletion>();
    // sys::save::save_game
    app.add_event::<sys::save::save_game::SaveGameCmd>();
    app.add_event::<sys::save::save_game::SavingGameComplete>();
    app.add_event::<sys::save::save_game::DeleteSessionCmd>();
    // sys::save::load_game
    app.add_event::<sys::save::load_game::LoadingGameComplete>();
    app.add_event::<sys::save::load_game::StartupRevealStarted>();
    app.add_event::<sys::save::load_game::StartupRevealFinished>();
    // sys::save::load_completion_checker
    app.add_event::<sys::save::load_completion_checker::DisplayCorruptedSaveFileWarning>();
    // res::session
    app.add_event::<res::session::NewSessionStartedCmd>();
    app.add_event::<res::session::SessionLoadedCmd>();
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::event::Events;
    use bevy_ecs::prelude::*;
    use res::walls::{WallId, plaster::{RecalculatePlasterCmd,
        RecalculateHalfTimberMeshCmd, RecalculateWoodenTurretCmd}};

    #[test]
    fn m6_rebuild_events_are_maintained_before_world_compatibility_initialization() {
        let mut app = App::new();
        // 与build_game保持同一顺序；后面的add_event不能修复已被裸init占用的通道。
        crate::run_game::register_gameplay_events(&mut app);
        sys::wall::registration::register_wall_cpu_resources(app.world_mut());
        crate::app_systems::WallSystems::add_wall_events(&mut app);
        let wall = WallId(7);
        app.world_mut().send_event(RecalculatePlasterCmd(wall));
        app.world_mut().send_event(RecalculateHalfTimberMeshCmd { wall, instances: vec![] });
        app.world_mut().send_event(RecalculateWoodenTurretCmd { wall, instances: vec![] });
        app.world_mut().send_event(sys::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall { wall, flag: 0 });
        for _ in 0..3 { app.update(); }
        assert!(app.world().resource::<Events<RecalculatePlasterCmd>>().is_empty());
        assert!(app.world().resource::<Events<RecalculateHalfTimberMeshCmd>>().is_empty());
        assert!(app.world().resource::<Events<RecalculateWoodenTurretCmd>>().is_empty());
        assert!(app.world().resource::<Events<sys::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>>().is_empty());
    }

    #[derive(Resource, Default)]
    struct EditObservations {
        first: Vec<WallId>,
        second: Vec<WallId>,
        marker_seen_during_second: Vec<bool>,
    }

    #[derive(Resource)]
    struct DeferredEditMarker;

    fn first_edit_reader(
        mut events: EventReader<res::events::OnCircleRotated>,
        mut observations: ResMut<EditObservations>,
        mut commands: Commands,
    ) {
        for event in events.read() {
            observations.first.push(event.0);
            commands.insert_resource(DeferredEditMarker);
        }
    }

    fn second_edit_reader(
        mut events: EventReader<res::events::OnCircleRotated>,
        mut observations: ResMut<EditObservations>,
        marker: Option<Res<DeferredEditMarker>>,
    ) {
        for event in events.read() {
            observations.second.push(event.0);
            observations.marker_seen_during_second.push(marker.is_some());
        }
    }

    #[test]
    fn shared_edit_events_have_independent_readers_and_frame_maintenance() {
        use std::any::TypeId;
        assert_eq!(TypeId::of::<res::events::OnShapeMoved>(),
            TypeId::of::<system_decorator::decorator_on_move_shape::MoveShapeEvent>());
        assert_eq!(TypeId::of::<res::events::OnFreehandWallEdited>(),
            TypeId::of::<system_decorator::decorator_on_freehand_wall_edited::FreehandWallEditedEvent>());
        assert_eq!(TypeId::of::<res::events::OnCircleRotated>(),
            TypeId::of::<system_decorator::decorator_on_freehand_wall_edited::CircleEditedEvent>());
        assert_eq!(TypeId::of::<res::events::OnRectangleEdited>(),
            TypeId::of::<sys::wall::shapes::rectangle::OnRectangleEdited>());

        let mut app = App::new();
        crate::run_game::register_gameplay_events(&mut app);
        sys::wall::registration::register_wall_cpu_resources(app.world_mut());
        app.init_resource::<EditObservations>();
        app.add_systems(bevy_app::Update,
            (first_edit_reader, second_edit_reader).chain_ignore_deferred());
        app.world_mut().send_event(res::events::OnCircleRotated(WallId(17)));
        app.update();
        let observations = app.world().resource::<EditObservations>();
        assert_eq!(observations.first, vec![WallId(17)]);
        assert_eq!(observations.second, vec![WallId(17)]);
        assert_eq!(observations.marker_seen_during_second, vec![false]);
        assert!(app.world().contains_resource::<DeferredEditMarker>());

        app.update();
        app.update();
        let observations = app.world().resource::<EditObservations>();
        assert_eq!(observations.first.len(), 1);
        assert_eq!(observations.second.len(), 1);
        assert!(app.world().resource::<Events<res::events::OnCircleRotated>>().is_empty());
    }
}
