// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/geforcenow/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 7f482b8183523c8f9361d6c481f46fa5f3e6161a36aef93a849aad76703638a4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 93, 262, 871
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use geforcenow; // namespace candidate
//   package identities: [{"name":"geforcenow","source":"local_or_path","version":"0.1.0"}]
// use libloading; // namespace candidate
//   package identities: [{"name":"libloading","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.7.3"},{"name":"libloading","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.3"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: geforcenow::is_running_in_cloud
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":262,"record_offset":256,"rva":6415968,"va":5375125088}
// 编译实例 RVAs: 0x61e660
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$libloading..os..windows..Library$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6755986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375125088,"raw_hex":"e892166700","size":5,"soft_fail":false,"target_va":5381881392,"va":5375125401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7971810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375125088,"raw_hex":"e8e2a37900","size":5,"soft_fail":false,"target_va":5383097008,"va":5375125193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::load_with_flags
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7971435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375125088,"raw_hex":"e86ba27900","size":5,"soft_fail":false,"target_va":5383096576,"va":5375125136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*%rbx","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5375125088,"raw_hex":"ffd3","size":2,"soft_fail":false,"va":5375125291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":262,"record_offset":664,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00262.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":262,"record_offset":832,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00262.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":262,"record_offset":720,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00262.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":262,"record_offset":464,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":16268,"source_line":147,"archive":"_pdb/B/modules/00262.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":262,"record_offset":388,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":16265,"source_line":83,"archive":"_pdb/B/modules/00262.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":262,"record_offset":856,"parent_record_offset":832,"ancestor_record_offset":256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00262.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":262,"record_offset":492,"parent_record_offset":464,"ancestor_record_offset":256,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00262.jsonl.gz"}
//   [pdb_inline_transitive] geforcenow::impl$3::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"geforcenow","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/geforcenow/src/lib.rs
//     evidence={"module":262,"record_offset":688,"parent_record_offset":664,"ancestor_record_offset":256,"inlinee":16270,"source_line":3,"archive":"_pdb/B/modules/00262.jsonl.gz"}
//   [pdb_inline_transitive] libloading::os::windows::Library::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/os/windows/mod.rs
//     evidence={"module":262,"record_offset":416,"parent_record_offset":388,"ancestor_record_offset":256,"inlinee":16267,"source_line":69,"archive":"_pdb/B/modules/00262.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: geforcenow::impl$1::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 3
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C01
// 首条位置: {"module":93,"inlinee":11265}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: geforcenow::impl$3::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 3
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F8E
// 首条位置: {"module":262,"inlinee":16270}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: geforcenow::impl$7::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2BFF
// 首条位置: {"module":93,"inlinee":11263}
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] geforcenow::GfnError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "enum2$<geforcenow::GfnError>", "constructor": "enum2$", "arguments": ["geforcenow::GfnError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<enum2$<geforcenow::GfnError> >", "constructor": "core::fmt::impl$74::fmt", "arguments": ["enum2$<geforcenow::GfnError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<bool,enum2$<geforcenow::GfnError> >", "constructor": "core::result::Result", "arguments": ["bool", "enum2$<geforcenow::GfnError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<bool,enum2$<geforcenow::GfnError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<bool,enum2$<geforcenow::GfnError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<bool,enum2$<geforcenow::GfnError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<bool,enum2$<geforcenow::GfnError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::impl$35::fmt<bool,enum2$<geforcenow::GfnError> >", "constructor": "core::result::impl$35::fmt", "arguments": ["bool", "enum2$<geforcenow::GfnError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] geforcenow::GfnInternalError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<geforcenow::GfnInternalError>", "constructor": "core::fmt::impl$74::fmt", "arguments": ["geforcenow::GfnInternalError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::DeletePlasterCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>", "bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::DeletePlasterCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::DeletePlasterCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::CreatePlasterCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::CreatePlasterCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::CreatePlasterCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::RecalculateHalfTimberMeshCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::RecalculateWoodenTurretCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>", "bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::RecalculateHalfTimberMeshCmd>", "bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::RecalculateWoodenTurretCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::RecalculateHalfTimberMeshCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::RecalculateWoodenTurretCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: geforcenow::impl$1::fmt
// 信息 ID: 02404e10048d135e90a0085e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<geforcenow::GfnInternalError>; module=93, inline_record=74072; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: geforcenow::impl$3::eq
// 信息 ID: a3767318511ccfab53b1a6fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] geforcenow::is_running_in_cloud; module=262, inline_record=688; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: geforcenow::impl$7::fmt
// 信息 ID: 4ef69f0121d1fd55749ae8ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<enum2$<geforcenow::GfnError> >; module=93, inline_record=73888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: geforcenow::is_running_in_cloud
// 信息 ID: 0cbb0506770b78c0375d516f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x5949e4 -> 0x61e660 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
