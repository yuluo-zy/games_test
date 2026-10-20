//! Register real Bevy event maintenance before resources-only domain helpers.
//! Bevy 0.16 App::add_event skips Events<T> that already exists. A helper's
//! init_resource<Events<T>> therefore cannot precede this App registration.
//! This list uses canonical domain identities and leaves all consumers intact.
use bevy_app::App;
use country_core::{resources as res, systems as sys};

pub(super) fn install(app: &mut App) {
    // res::walls::public_walls
    app.add_event::<res::walls::public_walls::OnWallChanged>();
    app.add_event::<res::walls::public_walls::OnWallDeleted>();
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
    // sys::audio
    app.add_event::<sys::audio::PlayWallUiSound>();
    app.add_event::<sys::audio::WallBendAudioParamReset>();
    app.add_event::<sys::audio::RequestWallMaterialAudio>();
    // sys::wall::shapes
    app.add_event::<sys::wall::shapes::CreateCircleCmd>();
    app.add_event::<sys::wall::shapes::CreateRectangleCmd>();
    app.add_event::<sys::wall::shapes::EditCircleCmd>();
    app.add_event::<sys::wall::shapes::EditRectangleCmd>();
    app.add_event::<sys::wall::shapes::FinishEditCircleCmd>();
    app.add_event::<sys::wall::shapes::FinishEditRectangleCmd>();
    app.add_event::<sys::wall::shapes::RotateCircleCmd>();
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
