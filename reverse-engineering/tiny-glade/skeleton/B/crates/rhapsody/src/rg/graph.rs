// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/rg/graph.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: bb452077b7e21c1853c21301f85d8579be0cfa09cb412fc31642904b2d70e8a5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: graph.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 93, 105, 676, 677, 679, 692, 697, 699
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::graph::ExportableResource::raw
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2EB7
// 首条位置: {"module":105,"inlinee":11959}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::graph::ExportableResource::index
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2EB6
// 首条位置: {"module":105,"inlinee":11958}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::graph::ExportableResource::raw
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":105,"record_offset":10272,"parent_record_offset":10248,"ancestor_record_offset":10248,"inlinee":11959,"source_line":25,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::graph::impl$12::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x99D4
// 首条位置: {"module":692,"inlinee":39380}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::graph::RenderGraph::on_retire
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9A32
// 首条位置: {"module":697,"record_offset":828,"rva":12489760,"va":5381198880}
// 编译实例 RVAs: 0xbe9420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdfc90
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: graph.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381198880,"raw_hex":"e82568ffff","size":5,"soft_fail":false,"target_va":5381160080,"va":5381198950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":697,"record_offset":956,"parent_record_offset":828,"ancestor_record_offset":828,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00697.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":697,"record_offset":1036,"parent_record_offset":1012,"ancestor_record_offset":828,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00697.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":697,"record_offset":1084,"parent_record_offset":1060,"ancestor_record_offset":828,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00697.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":697,"record_offset":1060,"parent_record_offset":1036,"ancestor_record_offset":828,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00697.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":697,"record_offset":1012,"parent_record_offset":988,"ancestor_record_offset":828,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00697.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":697,"record_offset":988,"parent_record_offset":956,"ancestor_record_offset":828,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00697.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":697,"record_offset":1128,"parent_record_offset":956,"ancestor_record_offset":828,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00697.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::graph::RenderGraph::create_resource_handle
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x984B
// 首条位置: {"module":677,"inlinee":38987}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":677,"record_offset":2932,"parent_record_offset":2896,"ancestor_record_offset":2896,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":677,"record_offset":3004,"parent_record_offset":2972,"ancestor_record_offset":2896,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":677,"record_offset":3052,"parent_record_offset":3028,"ancestor_record_offset":2896,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":677,"record_offset":3028,"parent_record_offset":3004,"ancestor_record_offset":2896,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":677,"record_offset":2972,"parent_record_offset":2932,"ancestor_record_offset":2896,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":677,"record_offset":3092,"parent_record_offset":2932,"ancestor_record_offset":2896,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::graph::RenderGraph::create_raw_resource
// 记录形式: compiled_procedure; instances=1
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":697,"record_offset":1220,"rva":12489936,"va":5381199056}
// 编译实例 RVAs: 0xbe94d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdfc90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: graph.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-39226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381199056,"raw_hex":"e8c666ffff","size":5,"soft_fail":false,"target_va":5381160080,"va":5381199301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-564015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381199056,"raw_hex":"e8d164f7ff","size":5,"soft_fail":false,"target_va":5380635120,"va":5381199130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::graph::RenderGraph::create_resource_handle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":697,"record_offset":1360,"parent_record_offset":1220,"ancestor_record_offset":1220,"inlinee":38987,"source_line":68,"archive":"_pdb/B/modules/00697.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":697,"record_offset":1468,"parent_record_offset":1436,"ancestor_record_offset":1220,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00697.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":697,"record_offset":1516,"parent_record_offset":1492,"ancestor_record_offset":1220,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00697.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":697,"record_offset":1492,"parent_record_offset":1468,"ancestor_record_offset":1220,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00697.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":697,"record_offset":1436,"parent_record_offset":1396,"ancestor_record_offset":1220,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00697.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":697,"record_offset":1396,"parent_record_offset":1360,"ancestor_record_offset":1220,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00697.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":697,"record_offset":1556,"parent_record_offset":1396,"ancestor_record_offset":1220,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00697.jsonl.gz"}
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
// 完整结构及原始类型记录: graph.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::graph::ExportableResource
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::rg::graph::ExportableResource>", "constructor": "enum2$", "arguments": ["rhapsody::rg::graph::ExportableResource"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<enum2$<rhapsody::rg::graph::ExportableResource>,vk_sync_fork::AccessType>", "constructor": "tuple$", "arguments": ["enum2$<rhapsody::rg::graph::ExportableResource>", "vk_sync_fork::AccessType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<tuple$<enum2$<rhapsody::rg::graph::ExportableResource>,vk_sync_fork::AccessType>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["tuple$<enum2$<rhapsody::rg::graph::ExportableResource>,vk_sync_fork::AccessType>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::graph::RenderGraph
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::graph::RenderGraph>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::graph::RenderGraph"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: graph.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: graph.rs.enriched.json.gz
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
// 函数: rhapsody::rg::graph::ExportableResource::index
// 信息 ID: 9d36cf941d652277b9095d96
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>; module=105, inline_record=10248; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>; module=105, inline_record=10496; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::resource_info::ResourceInfo::extend_usage_flags_for_exports; module=676, inline_record=2036; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::graph::ExportableResource::raw
// 信息 ID: b5b5533f0881a93ef1a400bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>; module=105, inline_record=10272; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>; module=105, inline_record=10520; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::resource_info::ResourceInfo::extend_usage_flags_for_exports; module=676, inline_record=2060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::graph::RenderGraph::create_raw_resource
// 信息 ID: b673445f9b92fa0f78cb6425
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 75 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x1623a2 -> 0xbe94d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x162900 -> 0xbe94d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x1629b7 -> 0xbe94d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x162a66 -> 0xbe94d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0> @ 0x1651a3 -> 0xbe94d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x31d5b4 -> 0xbe94d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec461 -> 0xbe94d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec4f2 -> 0xbe94d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ecaef -> 0xbe94d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ecd91 -> 0xbe94d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ecf47 -> 0xbe94d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::ssgi::calculate_ssgi @ 0x87444f -> 0xbe94d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::ssgi::calculate_ssgi @ 0x8747b3 -> 0xbe94d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::ssgi::calculate_ssgi @ 0x874bf4 -> 0xbe94d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::painterly::painterly_pass @ 0x8a68a9 -> 0xbe94d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::vermeer::vermeer_render_from_target @ 0x8a8f27 -> 0xbe94d0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::graph::RenderGraph::create_resource_handle
// 信息 ID: 74d03bd2e53f193bfe4f38fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::import_export::impl$1::import_unowned; module=677, inline_record=2896; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::import_export::impl$0::import_unowned; module=679, inline_record=7676; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::import_export::impl$0::import_owned; module=679, inline_record=8240; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::import_export::impl$2::into_sampled_rg_texture; module=679, inline_record=9488; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::graph::RenderGraph::create_raw_resource; module=697, inline_record=1360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::graph::RenderGraph::on_retire
// 信息 ID: 0170ee9c5b0ef90432e19d3c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x163b16 -> 0xbe9420 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0> @ 0x165849 -> 0xbe9420 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0> @ 0x166fe8 -> 0xbe9420 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::rg::executor::ExecutedRenderGraph::retire; module=699, inline_record=6268; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::graph::impl$12::default
// 信息 ID: b97f75eb274f3a693f963834
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::state::impl$0::default; module=692, inline_record=384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
