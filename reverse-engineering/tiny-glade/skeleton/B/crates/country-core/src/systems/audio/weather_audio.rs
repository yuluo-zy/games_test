// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/audio/weather_audio.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: fc2730034b62c12a24d542b5224644fea5fa7e0cdfaf1f51156aeb3de1607de0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: weather_audio.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1596, 1606, 1611, 1617
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::weather_audio::weather_audio_system
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11B52
// 首条位置: {"module":1596,"inlinee":72530}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_parameter
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":1611,"record_offset":2894952,"parent_record_offset":2894928,"ancestor_record_offset":2894928,"inlinee":27248,"source_line":314,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1611,"record_offset":2895008,"parent_record_offset":2894952,"ancestor_record_offset":2894928,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1611,"record_offset":2894980,"parent_record_offset":2894952,"ancestor_record_offset":2894928,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// === 完整可用 TPI 类型引用图（包含参数表、返回类型、字段等实际记录） ===
// TPI 0x1000:
// ---
// Kind:            LF_ARGLIST
// ArgList:
//   ArgIndices:      [  ]
// ...
// TPI 0x1001:
// ---
// Kind:            LF_PROCEDURE
// Procedure:
//   ReturnType:      3
//   CallConv:        NearC
//   Options:         [ None ]
//   ParameterCount:  0
//   ArgumentList:    4096
// ...

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: weather_audio.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: weather_audio.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: weather_audio.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 230; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::shapes::rectangle::CreateRectangleCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::ShapesDragMouseMoved>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorCaptureCmd> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::shapes::shape_placement::DontRecalculateShapePlacement>,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::timberframe_audio::PlayTimberframeSound> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::MaxDemoElevationReached>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle> >", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::event::writer::EventWriter<country_core::systems::wall::shapes::rectangle::CreateRectangleCmd>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::ShapesDragMouseMoved>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorCaptureCmd> >", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::event::writer::EventWriter<country_core::systems::wall::shapes::shape_placement::DontRecalculateShapePlacement>", "tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >", "tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::timberframe_audio::PlayTimberframeSound> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::MaxDemoElevationReached>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::shapes::rectangle::CreateRectangleCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::ShapesDragMouseMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorCaptureCmd> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::shapes::shape_placement::DontRecalculateShapePlacement>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::window::AudioCursorMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::events::ConstructTimberframeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::timberframe_audio::PlayTimberframeSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::MaxDemoElevationReached>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_draw_terrain::CacheMouseRaycastHeight>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>", "bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>", "bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>", "bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_draw_terrain::CacheMouseRaycastHeight>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_draw_terrain::CacheMouseRaycastHeight>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::weather_audio::weather_audio_system
// 信息 ID: 19d49c6ef6dacf9912e3d1fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ccfbb4cc9578ff42e5314e3d: void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>)> > >; module=1611, inline_record=2894928; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>)> >; module=1617, inline_record=13136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
