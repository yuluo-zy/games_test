// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/construct_elevation_supports/circle_categorize.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: ca3e5ac3251d048e7dfae1a05efa75f92d114b7510f3c5e98e98c9f17993bb5c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: circle_categorize.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1217
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::circle_categorize::circle_categorize
// 记录形式: compiled_procedure; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1217,"record_offset":260,"rva":13230128,"va":5381939248}
// 编译实例 RVAs: 0xc9e030
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b09b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_categorize.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6215993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381939248,"raw_hex":"e8c726a1ff","size":5,"soft_fail":false,"target_va":5375723952,"va":5381939940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle_categorize.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21654487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381939248,"raw_hex":"e8d76b4a01","size":5,"soft_fail":false,"target_va":5403593968,"va":5381939476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21038779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381939248,"raw_hex":"e8bb064101","size":5,"soft_fail":false,"target_va":5402978880,"va":5381940096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381939248,"raw_hex":"e816e4ffff","size":5,"soft_fail":false,"target_va":5381932784,"va":5381939925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6504933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381939248,"raw_hex":"e81bbe9cff","size":5,"soft_fail":false,"target_va":5375434976,"va":5381939904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::is_point_inside
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3970974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381939248,"raw_hex":"e86268c3ff","size":5,"soft_fail":false,"target_va":5377968880,"va":5381939849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1217,"record_offset":460,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/01217.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1217,"record_offset":692,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7622,"source_line":1248,"archive":"_pdb/A/modules/01217.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1217,"record_offset":1096,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01217.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1217,"record_offset":972,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01217.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1217,"record_offset":552,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01217.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1217,"record_offset":664,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/01217.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1217,"record_offset":488,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21527,"source_line":230,"archive":"_pdb/A/modules/01217.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1217,"record_offset":796,"parent_record_offset":720,"ancestor_record_offset":260,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01217.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1217,"record_offset":516,"parent_record_offset":488,"ancestor_record_offset":260,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01217.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1217,"record_offset":720,"parent_record_offset":692,"ancestor_record_offset":260,"inlinee":7624,"source_line":1153,"archive":"_pdb/A/modules/01217.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1217,"record_offset":604,"parent_record_offset":552,"ancestor_record_offset":260,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01217.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1217,"record_offset":576,"parent_record_offset":552,"ancestor_record_offset":260,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01217.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1217,"record_offset":764,"parent_record_offset":720,"ancestor_record_offset":260,"inlinee":7625,"source_line":545,"archive":"_pdb/A/modules/01217.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1217,"record_offset":1060,"parent_record_offset":1036,"ancestor_record_offset":260,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01217.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1217,"record_offset":1008,"parent_record_offset":972,"ancestor_record_offset":260,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1217,"record_offset":1036,"parent_record_offset":972,"ancestor_record_offset":260,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01217.jsonl.gz"}
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
// 完整结构及原始类型记录: circle_categorize.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: circle_categorize.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: circle_categorize.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::circle_categorize::circle_categorize
// 信息 ID: 52eaa88bf8ed9b6ad207227d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::construct_circle_supports @ 0x4d56bb -> 0xc9e030 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
