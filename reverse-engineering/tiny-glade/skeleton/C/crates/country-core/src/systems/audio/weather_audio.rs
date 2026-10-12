// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/audio/weather_audio.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: ba10d6beb1e4390ac583b8019c6e19009ea7f4501208359e2e1c63144f89c994
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: weather_audio.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 571, 613, 1698
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
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8D8D
// 首条位置: {"module":613,"inlinee":36237}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_parameter
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":613,"record_offset":37208,"parent_record_offset":37124,"ancestor_record_offset":37124,"inlinee":29638,"source_line":314,"archive":"_pdb/C/modules/00613.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_winter
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":613,"record_offset":37152,"parent_record_offset":37124,"ancestor_record_offset":37124,"inlinee":16149,"source_line":296,"archive":"_pdb/C/modules/00613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":613,"record_offset":37264,"parent_record_offset":37208,"ancestor_record_offset":37124,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00613.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":613,"record_offset":37236,"parent_record_offset":37208,"ancestor_record_offset":37124,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00613.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_winter
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":613,"record_offset":37176,"parent_record_offset":37152,"ancestor_record_offset":37124,"inlinee":27994,"source_line":117,"archive":"_pdb/C/modules/00613.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 282; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >", "bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound> >", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>", "bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayGroupBuildingMoveAudio>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayBuildingAudio>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayGroupBuildingMoveAudio>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayGroupBuildingMoveAudio>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::startup::install_mod_clutter::ModClutterInstalls>,bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::InstallModClutterCmd>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::UninstallModClutterCmd>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::startup::install_mod_clutter::ModClutterInstalls>", "bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::InstallModClutterCmd>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::UninstallModClutterCmd>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::install_mod_clutter::ModClutterInstalls>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::InstallModClutterCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::UninstallModClutterCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::weather_audio::weather_audio_system
// 信息 ID: b3045bd3a341c19916118ff2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 83ab698cc2cf9c901b10e63c: void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)>; module=613, inline_record=37124; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> >; module=1698, inline_record=3227344; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
