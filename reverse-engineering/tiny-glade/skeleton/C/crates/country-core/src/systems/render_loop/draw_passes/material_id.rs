// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/render_loop/draw_passes/material_id.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: af869ed5aba35af8ded4da07ca43684b546c1f165b38fa38ca441dd76d6ddd80
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: material_id.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1689, 1721
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// 当前没有可证实的 crate 引用信息。
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::material_id::impl$7::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 3
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14429
// 首条位置: {"module":1689,"inlinee":82985}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: material_id.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::draw_passes::material_id::MaterialId
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<country_core::systems::render_loop::draw_passes::material_id::MaterialId>", "constructor": "core::fmt::impl$80::fmt", "arguments": ["country_core::systems::render_loop::draw_passes::material_id::MaterialId"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: material_id.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/material_id.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: material_id.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// 父函数直接调用位置 = unknown/当前扫描未找到；PDB 内联及间接调用不能据此排除。
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: material_id.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 9; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::animate_terrain_delete::TerrainDeleteAnimation>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::systems::terrain::animate_terrain_delete::TerrainDeleteAnimation>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::terrain::animate_terrain_delete::TerrainDeleteAnimation>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::InitAdjWallHeightMode>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::main_camera::MainCameraMoved>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::FinishAdjWallHeightMode>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::InitAdjWallHeightMode>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::main_camera::MainCameraMoved>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::FinishAdjWallHeightMode>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::InitAdjWallHeightMode>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::main_camera::MainCameraMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::ui_edit_terrain::TerrainStrokeParamsCache>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::AddTerrainHighlightCmd>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemTerrainSteepness>,bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_edit_terrain::TerrainSteepnessEditAmount> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::main_camera::MainCameraMoved>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast> >,bevy_ecs::change_detection::Res<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::systems::terrain::ui_edit_terrain::TerrainStrokeParamsCache>", "bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::mouse_capture::CursorCaptureCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::AddTerrainHighlightCmd>", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemTerrainSteepness>,bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_edit_terrain::TerrainSteepnessEditAmount> >", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::main_camera::MainCameraMoved>", "tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast> >", "bevy_ecs::change_detection::Res<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::terrain::ui_edit_terrain::TerrainStrokeParamsCache>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::mouse_capture::CursorCaptureCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::AddTerrainHighlightCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemTerrainSteepness>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_edit_terrain::TerrainSteepnessEditAmount>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::main_camera::MainCameraMoved>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::CursorMovedScreenSpace>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::system::commands::Commands", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::prefab::SpawnPrefabCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::prefab::SpawnPrefabCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<country_core::resources::prefab::PrefabsModifiedCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::prefab::PrefabsModifiedCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::prefab::PrefabsModifiedCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::SpawnPrefabCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::prefab::PrefabsToDelete>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::SpawnPrefabCmd>", "bevy_ecs::system::commands::Commands", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::DespawnPrefabCmd>", "bevy_ecs::change_detection::ResMut<country_core::systems::prefab::PrefabsToDelete>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::SpawnPrefabCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::DespawnPrefabCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::prefab::PrefabsToDelete>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<ref_mut$<country_core::resources::prefab::PrefabInstance>,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<ref_mut$<country_core::resources::prefab::PrefabInstance>,tuple$<> >", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::DespawnPrefabCmd>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<country_core::resources::prefab::PrefabInstance>,tuple$<> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::DespawnPrefabCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::material_id::impl$7::fmt
// 信息 ID: 2340e954fa41d1aba6b0e26a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$80::fmt<country_core::systems::render_loop::draw_passes::material_id::MaterialId>; module=1689, inline_record=158424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
