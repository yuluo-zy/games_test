// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/startup/startup_deferred_raster.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 2f69bb5c75e5f01fb4478bea5663842f32155f8644dcf356f6833626d4557fdd
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: startup_deferred_raster.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 494, 559, 1596
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_deferred_raster::startup_deferred_raster
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B2D
// 首条位置: {"module":494,"inlinee":27437}
// 编译实例 RVAs: 0xa97a30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140b3fe40
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: startup_deferred_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t689093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379815984,"raw_hex":"e8c5830a00","size":5,"soft_fail":false,"target_va":5380505152,"va":5379816054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":494,"record_offset":33296,"parent_record_offset":33196,"ancestor_record_offset":33196,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00494.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":494,"record_offset":33240,"parent_record_offset":33196,"ancestor_record_offset":33196,"inlinee":25911,"source_line":53,"archive":"_pdb/B/modules/00494.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":494,"record_offset":33264,"parent_record_offset":33240,"ancestor_record_offset":33196,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00494.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":494,"record_offset":33320,"parent_record_offset":33296,"ancestor_record_offset":33196,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00494.jsonl.gz"}
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
// 完整结构及原始类型记录: startup_deferred_raster.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: startup_deferred_raster.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: startup_deferred_raster.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 230; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::color_id::ColorId>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::BalconyDoorInstanceData> > >,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::color_id::ColorId>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::BalconyDoorInstanceData> > >", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::BalconyDoorInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::color_id::ColorId>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::BalconyDoorInstanceData> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::VelocityOnlyInstanceData> > >,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::VelocityOnlyInstanceData> > >", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::VelocityOnlyInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::VelocityOnlyInstanceData> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::stairs::visual::StairsFloorMeshChunk> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StairsFloorInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::stairs::visual::StairsFloorMeshChunk>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::stairs::visual::StairsFloorMeshChunk> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StairsFloorInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::stairs::visual::StairsFloorMeshChunk>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StairsFloorInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::stairs::visual::StairsFloorMeshChunk> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StairsFloorInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::stairs::visual::StairsFloorMeshChunk>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::reader::EventReader<country_core::systems::ui_raise_shape::HandleRaiseShapeUiInteraction>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History> >,bevy_ecs::system::system_param::Local<country_core::systems::ui_raise_shape::RaiseShape>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::MaxDemoElevationReached>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::event::reader::EventReader<country_core::systems::ui_raise_shape::HandleRaiseShapeUiInteraction>", "tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History> >", "bevy_ecs::system::system_param::Local<country_core::systems::ui_raise_shape::RaiseShape>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>", "bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::MaxDemoElevationReached>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::ui_raise_shape::HandleRaiseShapeUiInteraction>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::ui_raise_shape::RaiseShape>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::MaxDemoElevationReached>
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
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::ui_edit_terrain::TerrainStrokeParamsCache>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::AddTerrainHighlightCmd>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemTerrainSteepness>,bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_edit_terrain::TerrainSteepnessEditAmount> >,bevy_ecs::event::reader::EventReader<country_core::systems::main_camera::MainCameraMoved>,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast> >,bevy_ecs::change_detection::Res<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::systems::terrain::ui_edit_terrain::TerrainStrokeParamsCache>", "bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>", "bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::AddTerrainHighlightCmd>", "tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemTerrainSteepness>,bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_edit_terrain::TerrainSteepnessEditAmount> >", "bevy_ecs::event::reader::EventReader<country_core::systems::main_camera::MainCameraMoved>", "tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast> >", "bevy_ecs::change_detection::Res<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::terrain::ui_edit_terrain::TerrainStrokeParamsCache>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CursorCaptureCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::AddTerrainHighlightCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemTerrainSteepness>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_edit_terrain::TerrainSteepnessEditAmount>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::main_camera::MainCameraMoved>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_deferred_raster::startup_deferred_raster
// 信息 ID: eb2bb1fe4383a09924b7bbae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0a0bc4d76faec5be35e32390: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>)
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>)> > > @ 0x14d19b5 -> 0xa97a30 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>)> >; module=494, inline_record=33196; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
