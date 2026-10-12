// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/color_id_roof.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: c58feadcb5f1e210a6ff49d49f120d603acd3f8e35e4a38798de26382bd5504f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: color_id_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1367
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// 当前没有可证实的 crate 引用信息。
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::color_id_roof::impl$0::default_value
// 记录形式: compiled_procedure; instances=1
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1367,"record_offset":70492,"rva":6725584,"va":5375434704}
// 编译实例 RVAs: 0x669fd0
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
// 完整结构及原始类型记录: color_id_roof.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::color_id_roof::RoofColorIdSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::color_id_roof::RoofColorIdSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::resources::color_id_roof::RoofColorIdSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::color_id_roof::RoofColorIdSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::resources::color_id_roof::RoofColorIdSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::resources::color_id_roof::RoofColorIdSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::resources::color_id_roof::RoofColorIdSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::resources::color_id_roof::RoofColorIdSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::resources::color_id_roof::RoofColorIdSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::color_id_roof::RoofColorIdSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::resources::color_id_roof::RoofColorIdSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::resources::color_id_roof::RoofColorIdSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::resources::color_id_roof::RoofColorIdSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::resources::color_id_roof::RoofColorIdSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::resources::color_id_roof::RoofColorIdSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::buffer<country_core::resources::color_id_roof::RoofColorIdSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::buffer", "arguments": ["country_core::resources::color_id_roof::RoofColorIdSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::resources::color_id_roof::RoofColorIdSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::resources::color_id_roof::RoofColorIdSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::color_id_roof::RoofColorIdSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::resources::color_id_roof::RoofColorIdSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: color_id_roof.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: color_id_roof.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::TimelapseReplayFinished>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >", "bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::TimelapseReplayFinished>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::TimelapseReplayFinished>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::InterruptTimelapse>,bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::TimelapseReplayFinished>,bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::InterruptTimelapse>", "bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::TimelapseReplayFinished>", "bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::InterruptTimelapse>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::TimelapseReplayFinished>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>

// ------------------------------------------------------------------
// 函数: country_core::resources::color_id_roof::impl$0::default_value
// 信息 ID: 403fae24dcb4b7335cbd8e7d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::app_systems::walls_on_glade_unload @ 0x3bdd4b -> 0x669fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_systems::walls_on_glade_unload @ 0x3bdf4a -> 0x669fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_systems::walls_on_glade_unload @ 0x3be131 -> 0x669fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::color_id::GateColorIdSsbo> @ 0x1ebe978 -> 0x669fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::color_id::HalftimberColorIdSsbo> @ 0x1ebed38 -> 0x669fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::color_id::WallColorIdSsbo> @ 0x1ebf0f8 -> 0x669fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::color_id_roof::RoofColorIdSsbo> @ 0x1ebf4b8 -> 0x669fd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
