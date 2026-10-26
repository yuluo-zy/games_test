# M01 形状编辑/提示系统的原版 SystemParam 列表（2026-10-26）

来源：skeleton/B（PDB 导出的 FunctionSystem 类型拼写）与 public-symbols.json 的 drop_in_place<FunctionSystem<fn(..),name>> 符号。这是重写时的参数真值，旧版 Rust 的自创事件/资源以此为准纠正。

# A. skeleton/B

## country-core/src/systems/ui_move_shapes.rs
  fn = void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,tuple$<bevy_ecs::change_   (30 个参数)
     1. Res<<cc::systems::mode_manager::AppMode> >
     2. Res<cc::resources::walls::public_walls::PublicWalls>
     3. ResMut<cc::resources::walls::inner_walls::InnerWalls>
     4. Query<&<cc::resources::roofs::roof_shape::Roof>,() >
     5. Res<cc::resources::mouse_terrain_raycast::CameraCursorRay>
     6. Res<cc::resources::cursor_terrain_raycast::CursorTerrainRaycast>
     7. EventReader<cc::resources::window::AudioCursorMoved>
     8. Res<cc::systems::collision::world::RaycastWorld>
     9. ResMut<cc::resources::walls::decorator_storage::DecoratorStorage>
    10. ResMut<cc::resources::walls::decorator_archivist::DecoratorArchivist>
    11. Res<cc::systems::ui::resources::ui_state::UiState>
    12. Res<cc::systems::ui::signifiers::signifier_create::UiSignifierStream>
    13. Local<cc::systems::ui_move_shapes::MoveShapeCache>
    14. ResMut<cc::resources::history::History>
    15. Res<cc::input::action_set::ActionSetStates>
    16. EventWriter<cc::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
    17. EventWriter<cc::systems::audio::building_audio::PlayBuildingAudio>
    18. EventWriter<cc::systems::audio::wall_audio_ui::PlayWallUiSound>
    19. EventWriter<cc::resources::events::OnShapeMoved>
    20. EventWriter<cc::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
    21. EventWriter<cc::resources::tree_storage::HideTreesinAreaCmd>
    22. EventWriter<<cc::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
    23. cc::resources::walls::decorator_archivist::DecoratorArchivalResources
    24. Res<cc::resources::terrain_height::TerrainHeightsData<cc::resources::terrain_height::TerrainHeightSsbo> >
    25. Res<cc::resources::glade_border::GladeBorder>
    26. EventWriter<cc::resources::glade_border::DisplayOutOfBorder>
    27. EventWriter<cc::resources::glade_border::FeedbackInputOutOfBorder>
    28. Res<cc::resources::time::GameTime>
    29. Res<cc::systems::mod_key::Modifiers>
    30. Res<cc::resources::shape_intersections::ShapeIntersections>

## country-core/src/systems/ui_move_freehand_walls.rs
  fn = void (*)(bevy_ecs::system::system_param::Local<country_core::systems::ui_move_freehand_walls::MoveWallCache>,bevy_ecs::c   (22 个参数)
     1. Local<cc::systems::ui_move_freehand_walls::MoveWallCache>
     2. Res<<cc::systems::mode_manager::AppMode> >
     3. Res<cc::resources::walls::public_walls::PublicWalls>
     4. ResMut<cc::resources::walls::inner_walls::InnerWalls>
     5. Res<cc::resources::cursor_terrain_raycast::CursorTerrainRaycast>
     6. EventReader<cc::resources::window::AudioCursorMoved>
     7. ResMut<cc::resources::walls::decorator_storage::DecoratorStorage>
     8. ResMut<cc::resources::walls::decorator_archivist::DecoratorArchivist>
     9. Res<cc::systems::ui::resources::ui_state::UiState>
    10. ResMut<cc::resources::history::History>
    11. EventWriter<cc::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
    12. cc::resources::walls::decorator_archivist::DecoratorArchivalResources
    13. Res<cc::input::action_set::ActionSetStates>
    14. Res<cc::resources::terrain_height::TerrainHeightsData<cc::resources::terrain_height::TerrainHeightSsbo> >
    15. Res<cc::systems::camera_rig::MainCamera>
    16. Res<cc::systems::ui::signifiers::signifier_create::UiSignifierStream>
    17. Res<cc::resources::glade_border::GladeBorder>
    18. EventWriter<cc::resources::glade_border::DisplayOutOfBorder>
    19. EventWriter<cc::resources::glade_border::FeedbackInputOutOfBorder>
    20. EventWriter<cc::resources::events::OnShapeMoved>
    21. EventWriter<cc::resources::tree_storage::HideTreesNextToWallCmd>
    22. EventWriter<<cc::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >

## country-core/src/systems/ui_edit_freehand_walls.rs
  fn = void (*)(bevy_ecs::system::system_param::Local<country_core::systems::ui_edit_freehand_walls::EditFreehandWallCache>,bev   (20 个参数)
     1. Local<cc::systems::ui_edit_freehand_walls::EditFreehandWallCache>
     2. Res<<cc::systems::mode_manager::AppMode> >
     3. Res<cc::resources::walls::public_walls::PublicWalls>
     4. ResMut<cc::resources::walls::inner_walls::InnerWalls>
     5. Res<cc::resources::cursor_terrain_raycast::CursorTerrainRaycast>
     6. EventReader<cc::resources::window::AudioCursorMoved>
     7. Res<cc::resources::walls::decorator_storage::DecoratorStorage>
     8. ResMut<cc::resources::walls::decorator_archivist::DecoratorArchivist>
     9. Res<cc::resources::terrain_height::TerrainHeightsData<cc::resources::terrain_height::TerrainHeightSsbo> >
    10. cc::resources::walls::decorator_archivist::DecoratorArchivalResources
    11. Res<cc::systems::ui::resources::ui_state::UiState>
    12. ResMut<cc::resources::history::History>
    13. EventWriter<cc::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
    14. EventWriter<cc::resources::events::OnFreehandWallEdited>
    15. Res<cc::systems::ui::signifiers::signifier_create::UiSignifierStream>
    16. Res<cc::resources::glade_border::GladeBorder>
    17. EventWriter<cc::resources::glade_border::DisplayOutOfBorder>
    18. EventWriter<cc::resources::glade_border::FeedbackInputOutOfBorder>
    19. EventWriter<<cc::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
    20. EventWriter<cc::resources::tree_storage::HideTreesNextToWallCmd>

## country-core/src/systems/wall/shapes/circle/ui_edit_circle.rs
  fn = void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detecti   (30 个参数)
     1. Res<<cc::systems::mode_manager::AppMode> >
     2. ResMut<cc::resources::walls::inner_walls::InnerWalls>
     3. Res<cc::resources::walls::public_walls::PublicWalls>
     4. Res<cc::resources::walls::public_walls::WallEntities>
     5. Query<&<cc::resources::walls::wall_trigger_volume::WallTriggerVolume>,() >
     6. cc::resources::walls::decorator_archivist::DecoratorArchivalResources
     7. Res<cc::resources::glade_border::GladeBorder>
     8. Res<cc::systems::ui::resources::ui_state::UiState>
     9. Res<cc::systems::ui::signifiers::signifier_create::UiSignifierStream>
    10. Local<cc::systems::wall::shapes::circle::ui_edit_circle::CircleGrabParams>
    11. Local<cc::systems::wall::shapes::circle::ui_edit_circle::CircleSignifierParams>
    12. ResMut<cc::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>
    13. Res<cc::resources::mouse_terrain_raycast::CursorPositionSS>
    14. Res<cc::resources::window::WindowSize>
    15. Res<cc::systems::camera_rig::MainCamera>
    16. ResMut<cc::resources::walls::decorator_archivist::DecoratorArchivist>
    17. ResMut<cc::resources::walls::decorator_storage::DecoratorStorage>
    18. Res<cc::resources::time::GameTime>
    19. Res<bevy_time::time::Time<() > >
    20. EventReader<cc::systems::main_camera::MainCameraMoved>
    21. EventWriter<cc::resources::glade_border::DisplayOutOfBorder>
    22. EventWriter<cc::resources::glade_border::FeedbackInputOutOfBorder>
    23. EventWriter<cc::systems::wall::wall_highlight::HighlightWallCmd>
    24. EventWriter<cc::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>
    25. EventWriter<cc::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
    26. EventWriter<cc::systems::audio::building_audio::PlayBuildingAudio>
    27. EventWriter<cc::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
    28. EventWriter<cc::resources::tree_storage::HideTreesinAreaCmd>
    29. EventWriter<<cc::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
    30. ResMut<cc::resources::history::History>
  fn = void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_li   (4 个参数)
     1. Query<tuple$<&mut <cc::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,&<cc::asset_libraries::Handle<cc::render::mesh::Mesh> >,&<cc::systems::ui::signifiers::SignifierAnimation> >,() >
     2. Res<cc::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>
     3. ResMut<cc::asset_libraries::mesh_library::AssetMeshLibrary>
     4. Res<cc::systems::ui::signifiers::signifier_create::UiSignifierStream>
  fn = void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::wall::shapes::circle::signifiers::edit_arrow:   (3 个参数)
     1. Query<tuple$<&<cc::systems::wall::shapes::circle::signifiers::edit_arrow::EditCircleSignifier>,&mut <utils::components::transform::Transform> >,() >
     2. Res<cc::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>
     3. Res<cc::systems::ui::signifiers::signifier_create::UiSignifierStream>

## country-core/src/systems/wall/shapes/circle/ui_rotate_circle.rs
  fn = void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detecti   (21 个参数)
     1. Res<<cc::systems::mode_manager::AppMode> >
     2. ResMut<cc::resources::walls::inner_walls::InnerWalls>
     3. Res<cc::resources::walls::public_walls::PublicWalls>
     4. cc::resources::walls::decorator_archivist::DecoratorArchivalResources
     5. ResMut<cc::resources::walls::decorator_archivist::DecoratorArchivist>
     6. ResMut<cc::resources::walls::decorator_storage::DecoratorStorage>
     7. Res<cc::systems::ui::resources::ui_state::UiState>
     8. Res<cc::systems::ui::signifiers::signifier_create::UiSignifierStream>
     9. Res<cc::resources::window::WindowSize>
    10. Res<cc::systems::camera_rig::MainCamera>
    11. Res<cc::resources::mouse_terrain_raycast::CameraCursorRay>
    12. ResMut<cc::resources::history::History>
    13. EventWriter<cc::resources::events::OnCircleRotated>
    14. EventWriter<cc::resources::roofs::commands::RecalculateRoofCmd>
    15. EventWriter<<cc::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
    16. EventWriter<cc::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>
    17. EventWriter<cc::systems::audio::building_audio::PlayBuildingAudio>
    18. EventWriter<<cc::systems::ui::resources::widget_params::PlayUiSound> >
    19. Local<cc::systems::wall::shapes::circle::ui_rotate_circle::RotateCircleCache>
    20. Local<cc::systems::wall::shapes::circle::ui_rotate_circle::CurrentAngleGizmo>
    21. Res<cc::resources::time::GameTime>

## country-core/src/systems/wall/shapes/circle/signifiers/edit_arrow.rs
  fn = void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::wall::shapes::circle::signifiers::edit_arrow:   (3 个参数)
     1. Query<tuple$<&<cc::systems::wall::shapes::circle::signifiers::edit_arrow::EditCircleSignifier>,&mut <utils::components::transform::Transform> >,() >
     2. Res<cc::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>
     3. Res<cc::systems::ui::signifiers::signifier_create::UiSignifierStream>

## country-core/src/systems/wall/shapes/circle/signifiers/edit_line.rs
  fn = void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_li   (4 个参数)
     1. Query<tuple$<&mut <cc::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,&<cc::asset_libraries::Handle<cc::render::mesh::Mesh> >,&<cc::systems::ui::signifiers::SignifierAnimation> >,() >
     2. Res<cc::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>
     3. ResMut<cc::asset_libraries::mesh_library::AssetMeshLibrary>
     4. Res<cc::systems::ui::signifiers::signifier_create::UiSignifierStream>

## country-core/src/systems/wall/shapes/rectangle/ui_edit_rectangle_dims.rs
  fn = void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detecti   (11 个参数)
     1. Res<<cc::systems::mode_manager::AppMode> >
     2. Res<cc::systems::ui::signifiers::signifier_create::UiSignifierStream>
     3. Res<cc::systems::ui::resources::ui_state::UiState>
     4. Res<cc::resources::mouse_terrain_raycast::CameraCursorRay>
     5. Res<cc::resources::walls::public_walls::PublicWalls>
     6. Res<cc::resources::walls::public_walls::WallEntities>
     7. Query<&<cc::resources::walls::wall_trigger_volume::WallTriggerVolume>,() >
     8. EventReader<cc::systems::main_camera::MainCameraMoved>
     9. ResMut<cc::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::UiEditRectangleActiveWidgetResource>
    10. EventWriter<cc::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::DoUiEditRectangleDims>
    11. bevy_ecs::system::commands::Commands
  fn = void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_det   (12 个参数)
     1. Res<cc::resources::walls::public_walls::PublicWalls>
     2. ResMut<cc::resources::walls::decorator_archivist::DecoratorArchivist>
     3. ResMut<<cc::systems::wall::shapes::rectangle::WipRectangle> >
     4. EventWriter<cc::systems::wall::shapes::rectangle::EditRectangleCmd>
     5. EventWriter<cc::systems::wall::shapes::rectangle::FinishEditRectangleCmd>
     6. cc::systems::ui::resources::slider3d::Slider3d
     7. Res<cc::resources::glade_border::GladeBorder>
     8. EventWriter<cc::resources::glade_border::DisplayOutOfBorder>
     9. EventWriter<cc::resources::glade_border::FeedbackInputOutOfBorder>
    10. EventWriter<cc::resources::tree_storage::HideTreesinAreaCmd>
    11. EventWriter<<cc::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
    12. EventReader<cc::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::DoUiEditRectangleDims>
  fn = void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::UiEditR   (4 个参数)
     1. Res<cc::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::UiEditRectangleActiveWidgetResource>
     2. Res<cc::systems::ui::signifiers::signifier_create::UiSignifierStream>
     3. Query<tuple$<&<cc::systems::wall::shapes::rectangle::signifier_edit_dims_line::EditRectangleLinePreview>,&<cc::asset_libraries::Handle<cc::render::mesh::Mesh> >,&<cc::systems::ui::signifiers::SignifierAnimation>,&mut <utils::components::transform::Transform> >,() >
     4. ResMut<cc::asset_libraries::mesh_library::AssetMeshLibrary>

## country-core/src/systems/wall/shapes/rectangle/ui_edit_rectangle_orientation.rs
  fn = void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detecti   (10 个参数)
     1. Res<<cc::systems::mode_manager::AppMode> >
     2. Res<cc::resources::walls::public_walls::PublicWalls>
     3. Res<cc::systems::ui::signifiers::signifier_create::UiSignifierStream>
     4. Res<cc::systems::ui::resources::ui_state::UiState>
     5. Res<cc::systems::camera_rig::MainCamera>
     6. Res<cc::resources::mouse_terrain_raycast::CameraCursorRay>
     7. EventReader<cc::systems::main_camera::MainCameraMoved>
     8. Local<cc::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ClosestCornerCache>
     9. EventWriter<cc::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::DoUiEditRectangleRotate>
    10. bevy_ecs::system::commands::Commands
  fn = void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_det   (12 个参数)
     1. Res<cc::resources::walls::public_walls::PublicWalls>
     2. ResMut<<cc::systems::wall::shapes::rectangle::WipRectangle> >
     3. EventWriter<cc::systems::wall::shapes::rectangle::EditRectangleCmd>
     4. EventWriter<cc::systems::wall::shapes::rectangle::FinishEditRectangleCmd>
     5. Res<cc::resources::time::GameTime>
     6. Res<cc::resources::mouse_terrain_raycast::CameraCursorRay>
     7. Res<cc::resources::glade_border::GladeBorder>
     8. EventWriter<cc::resources::glade_border::DisplayOutOfBorder>
     9. EventWriter<cc::resources::glade_border::FeedbackInputOutOfBorder>
    10. EventWriter<cc::resources::tree_storage::HideTreesinAreaCmd>
    11. EventWriter<<cc::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
    12. EventReader<cc::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::DoUiEditRectangleRotate>

## country-core/src/systems/wall/shapes/rectangle/signifier_edit_dims_line.rs
  fn = void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::UiEditR   (4 个参数)
     1. Res<cc::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::UiEditRectangleActiveWidgetResource>
     2. Res<cc::systems::ui::signifiers::signifier_create::UiSignifierStream>
     3. Query<tuple$<&<cc::systems::wall::shapes::rectangle::signifier_edit_dims_line::EditRectangleLinePreview>,&<cc::asset_libraries::Handle<cc::render::mesh::Mesh> >,&<cc::systems::ui::signifiers::SignifierAnimation>,&mut <utils::components::transform::Transform> >,() >
     4. ResMut<cc::asset_libraries::mesh_library::AssetMeshLibrary>

## country-core/src/systems/wall/shapes/rectangle/edit_rectangle.rs
  (未找到 FunctionSystem 类型拼写)

## country-core/src/systems/wall/shapes/rectangle/finish_edit_rectangle.rs
  (未找到 FunctionSystem 类型拼写)

## country-core/src/systems/wall/adjust_wall_height/ui_adjust_wall_height.rs
  (未找到 FunctionSystem 类型拼写)

## country-core/src/systems/ui_edit_focus.rs
  fn = void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detecti   (17 个参数)
     1. Res<<cc::systems::mode_manager::AppMode> >
     2. Res<cc::systems::ui::resources::ui_state::UiState>
     3. Res<cc::systems::ui::resources::widget_params::UiNavState>
     4. Res<cc::input::action_set::ActionSetStates>
     5. EventWriter<cc::systems::mode_manager::RequestAppModeChange>
     6. EventWriter<cc::systems::ui_edit_focus::ToolSelectionCancelled>
     7. EventWriter<cc::systems::wall::structure_outline::OutlineWallCmd>
     8. Res<cc::resources::mouse_terrain_raycast::CameraCursorRay>
     9. Res<cc::systems::collision::world::RaycastWorld>
    10. ResMut<cc::resources::walls::public_walls::PublicWalls>
    11. Res<cc::resources::walls::decorator_storage::DecoratorStorage>
    12. EventWriter<cc::systems::ui_clutter::ui_autoclutter::EvRightClickedAutoClutter>
    13. Res<cc::resources::history::History>
    14. Res<cc::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>
    15. Res<cc::resources::cursor_terrain_raycast::CursorTerrainRaycast>
    16. EventWriter<cc::resources::onboarding::NotifyHintSystemTerrain>
    17. Local<cc::systems::ui_edit_focus::TerrainFocusModeOrigin>
  fn = void (*)(bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::system::system_param::Loc   (9 个参数)
     1. ResMut<cc::game_profile::GameProfile>
     2. Local<system_onboarding::exponential_back_off::TimeSinceMouseMoved>
     3. EventReader<cc::resources::window::CursorMovedScreenSpace>
     4. Res<cc::resources::time::GameTime>
     5. ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>
     6. ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>
     7. EventWriter<cc::game_profile::SaveGameProfileCmd>
     8. EventReader<cc::systems::ui_edit_focus::ToolSelectionCancelled>
     9. EventReader<cc::resources::events::ModeJustChanged>
  fn = void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detecti   (11 个参数)
     1. Res<<cc::systems::mode_manager::AppMode> >
     2. ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>
     3. Res<cc::game_profile::GameProfile>
     4. EventReader<cc::resources::events::ModeJustChanged>
     5. EventReader<cc::systems::ui_edit_focus::ToolSelectionCancelled>
     6. EventReader<cc::resources::onboarding::NotifyHintSystemToolWasJustUsed>
     7. Res<cc::input::action_set::ActionSetStates>
     8. EventWriter<system_onboarding::hint_orchestrator::DisplayHintRequest>
     9. EventWriter<system_onboarding::tutorial_completion::TutorialHintComplete>
    10. EventReader<cc::systems::wall::shapes::circle::create_circle::CreateCircleCmd>
    11. EventReader<cc::systems::wall::shapes::rectangle::CreateRectangleCmd>
  fn = void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detecti   (14 个参数)
     1. Res<<cc::systems::mode_manager::AppMode> >
     2. Res<cc::game_profile::GameProfile>
     3. ResMut<system_onboarding::hint_stairs::StairsHintConditions>
     4. EventWriter<system_onboarding::tutorial_completion::TutorialHintComplete>
     5. EventWriter<system_onboarding::hint_orchestrator::DisplayHintRequest>
     6. EventReader<cc::resources::onboarding::NotifyHintSystemStairsPlaced>
     7. EventReader<cc::resources::onboarding::NotifyHintSystemHoveringOverStairs>
     8. EventReader<cc::resources::onboarding::NotifyHintSystemStartedStairsContinue>
     9. EventReader<cc::resources::onboarding::NotifyHintSystemCanContinueStairs>
    10. EventReader<cc::resources::onboarding::NotifyHintSystemStairsDetachedFromWall>
    11. EventReader<cc::resources::onboarding::NotifyHintSystemStairsCanDetachFromWall>
    12. EventReader<cc::systems::ui_edit_focus::ToolSelectionCancelled>
    13. EventReader<cc::resources::events::ModeJustChanged>
    14. Res<cc::resources::time::GameTime>

## systems/onboarding/src/hint_deselect_tool.rs
  fn = void (*)(bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::system::system_param::Loc   (9 个参数)
     1. ResMut<cc::game_profile::GameProfile>
     2. Local<system_onboarding::exponential_back_off::TimeSinceMouseMoved>
     3. EventReader<cc::resources::window::CursorMovedScreenSpace>
     4. Res<cc::resources::time::GameTime>
     5. ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>
     6. ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>
     7. EventWriter<cc::game_profile::SaveGameProfileCmd>
     8. EventReader<cc::systems::ui_edit_focus::ToolSelectionCancelled>
     9. EventReader<cc::resources::events::ModeJustChanged>
  fn = void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detecti   (11 个参数)
     1. Res<<cc::systems::mode_manager::AppMode> >
     2. ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>
     3. Res<cc::game_profile::GameProfile>
     4. EventReader<cc::resources::events::ModeJustChanged>
     5. EventReader<cc::systems::ui_edit_focus::ToolSelectionCancelled>
     6. EventReader<cc::resources::onboarding::NotifyHintSystemToolWasJustUsed>
     7. Res<cc::input::action_set::ActionSetStates>
     8. EventWriter<system_onboarding::hint_orchestrator::DisplayHintRequest>
     9. EventWriter<system_onboarding::tutorial_completion::TutorialHintComplete>
    10. EventReader<cc::systems::wall::shapes::circle::create_circle::CreateCircleCmd>
    11. EventReader<cc::systems::wall::shapes::rectangle::CreateRectangleCmd>
  fn = void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::onboarding::TutorialOp>,tuple$<bevy_ecs::change_d   (16 个参数)
     1. EventReader<cc::resources::onboarding::TutorialOp>
     2. ResMut<system_onboarding::hint_terrain::HintTerrainConditions>
     3. ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>
     4. ResMut<system_onboarding::hint_terrain_focus::HintFocusTerrainConditions>
     5. ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>
     6. ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>
     7. ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>
     8. ResMut<system_onboarding::hint_undo::UndoHintConditions>
     9. ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>
    10. ResMut<system_onboarding::hint_clutter_focus_edit::FocusedClutterHintConditions>
    11. ResMut<system_onboarding::hint_stairs::StairsHintConditions>
    12. ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>
    13. ResMut<<system_onboarding::hint_camera_controls::CameraHintsState> >
    14. ResMut<cc::game_profile::GameProfile>
    15. EventWriter<cc::game_profile::SaveGameProfileCmd>
    16. ResMut<<cc::resources::onboarding::WelcomeScreenState> >

## systems/onboarding/src/hint_stairs.rs
  fn = void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detecti   (14 个参数)
     1. Res<<cc::systems::mode_manager::AppMode> >
     2. Res<cc::game_profile::GameProfile>
     3. ResMut<system_onboarding::hint_stairs::StairsHintConditions>
     4. EventWriter<system_onboarding::tutorial_completion::TutorialHintComplete>
     5. EventWriter<system_onboarding::hint_orchestrator::DisplayHintRequest>
     6. EventReader<cc::resources::onboarding::NotifyHintSystemStairsPlaced>
     7. EventReader<cc::resources::onboarding::NotifyHintSystemHoveringOverStairs>
     8. EventReader<cc::resources::onboarding::NotifyHintSystemStartedStairsContinue>
     9. EventReader<cc::resources::onboarding::NotifyHintSystemCanContinueStairs>
    10. EventReader<cc::resources::onboarding::NotifyHintSystemStairsDetachedFromWall>
    11. EventReader<cc::resources::onboarding::NotifyHintSystemStairsCanDetachFromWall>
    12. EventReader<cc::systems::ui_edit_focus::ToolSelectionCancelled>
    13. EventReader<cc::resources::events::ModeJustChanged>
    14. Res<cc::resources::time::GameTime>
  fn = void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::onboarding::TutorialOp>,tuple$<bevy_ecs::change_d   (16 个参数)
     1. EventReader<cc::resources::onboarding::TutorialOp>
     2. ResMut<system_onboarding::hint_terrain::HintTerrainConditions>
     3. ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>
     4. ResMut<system_onboarding::hint_terrain_focus::HintFocusTerrainConditions>
     5. ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>
     6. ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>
     7. ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>
     8. ResMut<system_onboarding::hint_undo::UndoHintConditions>
     9. ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>
    10. ResMut<system_onboarding::hint_clutter_focus_edit::FocusedClutterHintConditions>
    11. ResMut<system_onboarding::hint_stairs::StairsHintConditions>
    12. ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>
    13. ResMut<<system_onboarding::hint_camera_controls::CameraHintsState> >
    14. ResMut<cc::game_profile::GameProfile>
    15. EventWriter<cc::game_profile::SaveGameProfileCmd>
    16. ResMut<<cc::resources::onboarding::WelcomeScreenState> >


# B. PDB drop_in_place 符号

## country_core::systems::ui_edit_focus::ui_edit_focus  (17 个参数)
   1. Res<cc::systems::mode_manager::AppMode>
   2. Res<cc::systems::ui::resources::ui_state::UiState>
   3. Res<cc::systems::ui::resources::widget_params::UiNavState>
   4. Res<cc::input::action_set::ActionSetStates>
   5. EventWriter<cc::systems::mode_manager::RequestAppModeChange>
   6. EventWriter<cc::systems::ui_edit_focus::ToolSelectionCancelled>
   7. EventWriter<cc::systems::wall::structure_outline::OutlineWallCmd>
   8. Res<cc::resources::mouse_terrain_raycast::CameraCursorRay>
   9. Res<cc::systems::collision::world::RaycastWorld>
  10. ResMut<cc::resources::walls::public_walls::PublicWalls>
  11. Res<cc::resources::walls::decorator_storage::DecoratorStorage>
  12. EventWriter<cc::systems::ui_clutter::ui_autoclutter::EvRightClickedAutoClutter>
  13. Res<cc::resources::history::History>
  14. Res<cc::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>
  15. Res<cc::resources::cursor_terrain_raycast::CursorTerrainRaycast>
  16. EventWriter<cc::resources::onboarding::NotifyHintSystemTerrain>
  17. Local<cc::systems::ui_edit_focus::TerrainFocusModeOrigin>

## country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height::ui_adjust_wall_height  (11 个参数)
   1. Res<cc::systems::mode_manager::AppMode>
   2. Res<cc::resources::walls::public_walls::PublicWalls>
   3. Query<&cc::resources::roofs::roof_shape::Roof>
   4. Res<cc::systems::ui::resources::ui_state::UiState>
   5. EventWriter<cc::resources::events::FinishAdjWallHeightMode>
   6. EventWriter<cc::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>
   7. EventWriter<cc::resources::events::InitAdjWallHeightMode>
   8. Query<&cc::resources::roofs::roof_animation::RoofAnimation>
   9. Res<cc::systems::ui::signifiers::signifier_create::UiSignifierStream>
  10. EventReader<cc::systems::main_camera::MainCameraMoved>
  11. Res<cc::resources::time::GameTime>

## country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::history_record_adjust_wall_height  (7 个参数)
   1. ResMut<cc::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjWallHeightModeCache>
   2. EventReader<cc::resources::events::FinishAdjWallHeightMode>
   3. Query<(&cc::components::TestCube,&mut utils::components::transform::Transform)>
   4. ResMut<cc::resources::history::History>
   5. Res<cc::resources::walls::public_walls::PublicWalls>
   6. Query<&cc::resources::roofs::roof_shape::Roof>
   7. EventWriter<cc::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>

## country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::ui_adjust_wall_height_mode  (12 个参数)
   1. Res<cc::resources::mouse_terrain_raycast::CursorPositionSS>
   2. Res<cc::systems::camera_rig::MainCamera>
   3. Res<cc::resources::window::WindowSize>
   4. Res<cc::resources::walls::public_walls::PublicWalls>
   5. cc::resources::walls::decorator_archivist::DecoratorArchivalResources
   6. ResMut<cc::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjWallHeightModeCache>
   7. EventReader<cc::resources::window::AudioCursorMoved>
   8. EventReader<cc::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>
   9. EventWriter<cc::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>
  10. Query<(&cc::components::TestCube,&mut utils::components::transform::Transform)>
  11. Res<cc::resources::walls::decorator_storage::DecoratorStorage>
  12. ResMut<cc::resources::walls::decorator_archivist::DecoratorArchivist>

## country_core::systems::wall::shapes::circle::ui_create_circle::ui_create_circle  (21 个参数)
   1. Res<cc::systems::mode_manager::AppMode>
   2. Res<cc::input::action_set::ActionSetStates>
   3. Res<cc::resources::mouse_terrain_raycast::CursorPositionSS>
   4. Res<cc::resources::mouse_terrain_raycast::CameraCursorRay>
   5. Res<cc::resources::time::GameTime>
   6. Res<cc::systems::wall::shapes::shape_placement::ShapePlacement>
   7. ResMut<cc::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>
   8. Res<cc::systems::ui::resources::ui_state::UiState>
   9. Res<cc::systems::ui::signifiers::signifier_create::UiSignifierStream>
  10. ResMut<cc::systems::wall::shapes::rectangle::ui_create_rectangle::ShapesDragMouseMoved>
  11. EventReader<cc::resources::window::AudioCursorMoved>
  12. EventWriter<cc::resources::events::ConstructTimberframeCmd>
  13. EventWriter<cc::resources::tree_storage::HideTreesinAreaCmd>
  14. EventWriter<cc::systems::audio::timberframe_audio::PlayTimberframeSound>
  15. EventWriter<cc::systems::wall::shapes::circle::create_circle::CreateCircleCmd>
  16. EventWriter<cc::resources::mouse_capture::OsCursorCaptureCmd>
  17. EventWriter<cc::systems::wall::shapes::shape_placement::DontRecalculateShapePlacement>
  18. Res<cc::resources::glade_border::GladeBorder>
  19. EventWriter<cc::resources::glade_border::DisplayOutOfBorder>
  20. EventWriter<cc::resources::glade_border::FeedbackInputOutOfBorder>
  21. EventWriter<cc::resources::glade_border::MaxDemoElevationReached>

## country_core::systems::wall::shapes::rectangle::edit_rectangle::edit_rectangle  (5 个参数)
   1. EventReader<cc::systems::wall::shapes::rectangle::EditRectangleCmd>
   2. ResMut<cc::resources::walls::inner_walls::InnerWalls>
   3. EventWriter<cc::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
   4. EventWriter<cc::systems::audio::building_audio::PlayBuildingAudio>
   5. Res<bevy_time::time::Time>

## country_core::systems::wall::shapes::rectangle::finish_edit_rectangle::finish_edit_rectangle  (9 个参数)
   1. Res<cc::resources::walls::inner_walls::InnerWalls>
   2. ResMut<cc::systems::wall::shapes::rectangle::WipRectangle>
   3. ResMut<cc::resources::history::History>
   4. ResMut<cc::resources::walls::decorator_storage::DecoratorStorage>
   5. ResMut<cc::resources::walls::decorator_archivist::DecoratorArchivist>
   6. EventReader<cc::systems::wall::shapes::rectangle::FinishEditRectangleCmd>
   7. EventWriter<cc::resources::events::OnRectangleEdited>
   8. EventWriter<cc::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>
   9. EventWriter<cc::resources::critters::sheep::CmdCheckForSheepInsideBuilding>

## country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::signifier_edit_dims  (4 个参数)
   1. Res<cc::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::UiEditRectangleActiveWidgetResource>
   2. Res<cc::systems::ui::signifiers::signifier_create::UiSignifierStream>
   3. Query<(&cc::systems::wall::shapes::rectangle::signifier_edit_dims_line::EditRectangleLinePreview,&cc::asset_libraries::Handle<cc::render::mesh::Mesh>,&cc::systems::ui::signifiers::SignifierAnimation,&mut utils::components::transform::Transform)>
   4. ResMut<cc::asset_libraries::mesh_library::AssetMeshLibrary>

## country_core::systems::wall::shapes::rectangle::ui_create_rectangle::ui_create_rectangle  (22 个参数)
   1. Res<cc::systems::mode_manager::AppMode>
   2. Res<cc::systems::ui::resources::ui_state::UiState>
   3. Res<cc::systems::wall::shapes::shape_placement::ShapePlacement>
   4. ResMut<cc::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>
   5. Res<cc::systems::camera_rig::MainCamera>
   6. EventWriter<cc::systems::wall::shapes::rectangle::CreateRectangleCmd>
   7. Res<cc::systems::ui::signifiers::signifier_create::UiSignifierStream>
   8. ResMut<cc::systems::wall::shapes::rectangle::ui_create_rectangle::ShapesDragMouseMoved>
   9. Res<cc::input::action_set::ActionSetStates>
  10. EventWriter<cc::resources::mouse_capture::OsCursorCaptureCmd>
  11. Res<cc::resources::mouse_terrain_raycast::CursorPositionSS>
  12. EventWriter<cc::systems::wall::shapes::shape_placement::DontRecalculateShapePlacement>
  13. EventReader<cc::resources::window::AudioCursorMoved>
  14. Res<cc::resources::mouse_terrain_raycast::CameraCursorRay>
  15. EventWriter<cc::resources::events::ConstructTimberframeCmd>
  16. EventWriter<cc::resources::tree_storage::HideTreesinAreaCmd>
  17. EventWriter<cc::systems::audio::timberframe_audio::PlayTimberframeSound>
  18. Res<cc::resources::time::GameTime>
  19. Res<cc::resources::glade_border::GladeBorder>
  20. EventWriter<cc::resources::glade_border::DisplayOutOfBorder>
  21. EventWriter<cc::resources::glade_border::FeedbackInputOutOfBorder>
  22. EventWriter<cc::resources::glade_border::MaxDemoElevationReached>
