// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/utils/src/wall_space.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 6f8a070b8cf5a1841db988a46de6f5bbba36f28812cad72e5eaf2cb95c557df5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_space.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 60, 383, 385, 457, 476, 1248, 1249, 1253, 1257, 1261, 1284, 1286, 1308, 1309, 1321, 1367, 1370, 1399, 1440, 2269, 2745, 2756, 2767, 2773, 2775, 2776, 2805, 3828, 3841, 3842, 3854, 3858, 3859, 3863, 3865, 4374, 4379
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::wall_space::impl$7::fmt
// 记录形式: inlinee_declaration; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B43B
// 首条位置: {"module":3828,"inlinee":111675}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::impl$9::eq
// 记录形式: inlinee_declaration; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEA7F
// 首条位置: {"module":1261,"inlinee":60031}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::impl$0::distance
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1261,"record_offset":244,"rva":13546640,"va":5382255760}
// 编译实例 RVAs: 0xceb490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::comparative_space::impl$2::distance
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1261,"record_offset":480,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":60012,"source_line":91,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_direct] utils::comparative_space::impl$3::distance
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1261,"record_offset":368,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":36066,"source_line":127,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1261,"record_offset":448,"parent_record_offset":368,"ancestor_record_offset":244,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1261,"record_offset":396,"parent_record_offset":368,"ancestor_record_offset":244,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::impl$0::project
// 记录形式: compiled_procedure, inlinee_declaration; instances=3
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEA76
// 首条位置: {"module":1261,"inlinee":60022}
// 编译实例 RVAs: 0xceb4e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144da0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_space.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21338274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382255840,"raw_hex":"e8a2984501","size":5,"soft_fail":false,"target_va":5403594144,"va":5382255865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::comparative_space::impl$2::project
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1261,"record_offset":724,"parent_record_offset":540,"ancestor_record_offset":540,"inlinee":60014,"source_line":96,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_direct] utils::comparative_space::impl$3::project
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1261,"record_offset":664,"parent_record_offset":540,"ancestor_record_offset":540,"inlinee":60013,"source_line":144,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1261,"record_offset":688,"parent_record_offset":664,"ancestor_record_offset":540,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::impl$0::project_clamp
// 记录形式: compiled_procedure, inlinee_declaration; instances=3
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEA78
// 首条位置: {"module":1261,"inlinee":60024}
// 编译实例 RVAs: 0xceb560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144da0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_space.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21338140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382255968,"raw_hex":"e81c984501","size":5,"soft_fail":false,"target_va":5403594144,"va":5382255999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20722738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382255968,"raw_hex":"e832343c01","size":5,"soft_fail":false,"target_va":5402978944,"va":5382256201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::comparative_space::impl$2::project_clamp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1261,"record_offset":944,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":60016,"source_line":105,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_direct] utils::comparative_space::impl$3::project_clamp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1261,"record_offset":884,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":60015,"source_line":149,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1261,"record_offset":972,"parent_record_offset":944,"ancestor_record_offset":756,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1261,"record_offset":1020,"parent_record_offset":972,"ancestor_record_offset":756,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1261,"record_offset":908,"parent_record_offset":884,"ancestor_record_offset":756,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::impl$0::project_unbounded
// 记录形式: compiled_procedure, inlinee_declaration; instances=3
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEA7B
// 首条位置: {"module":1261,"inlinee":60027}
// 编译实例 RVAs: 0xceb650
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144da0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_space.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21337906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382256208,"raw_hex":"e832974501","size":5,"soft_fail":false,"target_va":5403594144,"va":5382256233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1261,"record_offset":1188,"parent_record_offset":1060,"ancestor_record_offset":1060,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::impl$0::subtract
// 记录形式: compiled_procedure, inlinee_declaration; instances=3
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEA80
// 首条位置: {"module":1261,"inlinee":60032}
// 编译实例 RVAs: 0xceb6a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::comparative_space::impl$2::subtract
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1261,"record_offset":1388,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":60017,"source_line":115,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_direct] utils::comparative_space::impl$3::subtract
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1261,"record_offset":1348,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":23181,"source_line":165,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::WallSpaceRegion::intersects
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F4D
// 首条位置: {"module":2773,"inlinee":89933}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::comparative_space::impl$0::regions_intersect
//     owner=private_source_ownership_candidate; count=16
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":2773,"record_offset":12404,"parent_record_offset":12368,"ancestor_record_offset":12368,"inlinee":89935,"source_line":39,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2773,"record_offset":12452,"parent_record_offset":12428,"ancestor_record_offset":12368,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$1::distance
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":2773,"record_offset":12428,"parent_record_offset":12404,"ancestor_record_offset":12368,"inlinee":89937,"source_line":58,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$2::distance
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":2775,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":1500,"inlinee":60012,"source_line":91,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF797
// 首条位置: {"module":1440,"inlinee":63383}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::__private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":213768,"parent_record_offset":213744,"ancestor_record_offset":213744,"inlinee":6856,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1440,"record_offset":213792,"parent_record_offset":213768,"ancestor_record_offset":213744,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF796
// 首条位置: {"module":1440,"inlinee":63382}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1440,"record_offset":142844,"parent_record_offset":142820,"ancestor_record_offset":142820,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1440,"record_offset":142868,"parent_record_offset":142844,"ancestor_record_offset":142820,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":142892,"parent_record_offset":142868,"ancestor_record_offset":142820,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":142916,"parent_record_offset":142892,"ancestor_record_offset":142820,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10427
// 首条位置: {"module":1440,"inlinee":66599}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":142720,"parent_record_offset":142696,"ancestor_record_offset":142696,"inlinee":66600,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":143116,"parent_record_offset":143092,"ancestor_record_offset":142696,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":143092,"parent_record_offset":143036,"ancestor_record_offset":142696,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":143064,"parent_record_offset":143036,"ancestor_record_offset":142696,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":143036,"parent_record_offset":143012,"ancestor_record_offset":142696,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1440,"record_offset":142868,"parent_record_offset":142844,"ancestor_record_offset":142696,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":142964,"parent_record_offset":142792,"ancestor_record_offset":142696,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":142892,"parent_record_offset":142868,"ancestor_record_offset":142696,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":142916,"parent_record_offset":142892,"ancestor_record_offset":142696,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1440,"record_offset":142844,"parent_record_offset":142820,"ancestor_record_offset":142696,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":142792,"parent_record_offset":142768,"ancestor_record_offset":142696,"inlinee":9763,"source_line":1529,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":142744,"parent_record_offset":142720,"ancestor_record_offset":142696,"inlinee":50427,"source_line":1148,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":142768,"parent_record_offset":142744,"ancestor_record_offset":142696,"inlinee":50429,"source_line":1301,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1440,"record_offset":142820,"parent_record_offset":142792,"ancestor_record_offset":142696,"inlinee":63382,"source_line":97,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::_::impl$0::deserialize::impl$2::expecting
// 记录形式: compiled_procedure; instances=1
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1261,"record_offset":5764,"rva":13548512,"va":5382257632}
// 编译实例 RVAs: 0xcebbe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_space.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-4673733","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5382257632,"raw_hex":"e93bafb8ff","size":5,"soft_fail":false,"target_va":5377583920,"va":5382257648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10425
// 首条位置: {"module":1440,"inlinee":66597}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":143984,"parent_record_offset":141208,"ancestor_record_offset":141208,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":143952,"parent_record_offset":141208,"ancestor_record_offset":141208,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1440,"record_offset":144320,"parent_record_offset":141208,"ancestor_record_offset":141208,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":141236,"parent_record_offset":141208,"ancestor_record_offset":141208,"inlinee":9789,"source_line":1936,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":143400,"parent_record_offset":141208,"ancestor_record_offset":141208,"inlinee":9791,"source_line":1944,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":141912,"parent_record_offset":141888,"ancestor_record_offset":141208,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":141888,"parent_record_offset":141832,"ancestor_record_offset":141208,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":142532,"parent_record_offset":142504,"ancestor_record_offset":141208,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":142592,"parent_record_offset":142532,"ancestor_record_offset":141208,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":141392,"parent_record_offset":141368,"ancestor_record_offset":141208,"inlinee":7109,"source_line":1719,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":141444,"parent_record_offset":141416,"ancestor_record_offset":141208,"inlinee":7033,"source_line":1721,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1440,"record_offset":141468,"parent_record_offset":141444,"ancestor_record_offset":141208,"inlinee":7034,"source_line":1161,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":141860,"parent_record_offset":141832,"ancestor_record_offset":141208,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":142044,"parent_record_offset":142020,"ancestor_record_offset":141208,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":141832,"parent_record_offset":141804,"ancestor_record_offset":141208,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1440,"record_offset":142020,"parent_record_offset":141992,"ancestor_record_offset":141208,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1440,"record_offset":141992,"parent_record_offset":141960,"ancestor_record_offset":141208,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1440,"record_offset":142868,"parent_record_offset":142844,"ancestor_record_offset":141208,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1440,"record_offset":142624,"parent_record_offset":142532,"ancestor_record_offset":141208,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1440,"record_offset":143516,"parent_record_offset":143492,"ancestor_record_offset":141208,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1440,"record_offset":142192,"parent_record_offset":142164,"ancestor_record_offset":141208,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":141416,"parent_record_offset":141392,"ancestor_record_offset":141208,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":143492,"parent_record_offset":143456,"ancestor_record_offset":141208,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":141684,"parent_record_offset":141316,"ancestor_record_offset":141208,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1440,"record_offset":141544,"parent_record_offset":141520,"ancestor_record_offset":141208,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":142560,"parent_record_offset":142532,"ancestor_record_offset":141208,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":143572,"parent_record_offset":143516,"ancestor_record_offset":141208,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":142892,"parent_record_offset":142868,"ancestor_record_offset":141208,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":142916,"parent_record_offset":142892,"ancestor_record_offset":141208,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1440,"record_offset":141496,"parent_record_offset":141468,"ancestor_record_offset":141208,"inlinee":7035,"source_line":721,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1440,"record_offset":141520,"parent_record_offset":141496,"ancestor_record_offset":141208,"inlinee":7036,"source_line":251,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1440,"record_offset":142844,"parent_record_offset":142820,"ancestor_record_offset":141208,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1440,"record_offset":144392,"parent_record_offset":144368,"ancestor_record_offset":141208,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":144368,"parent_record_offset":144344,"ancestor_record_offset":141208,"inlinee":6871,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":141276,"parent_record_offset":141236,"ancestor_record_offset":141208,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":143428,"parent_record_offset":143400,"ancestor_record_offset":141208,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":142792,"parent_record_offset":142768,"ancestor_record_offset":141208,"inlinee":9763,"source_line":1529,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1440,"record_offset":143632,"parent_record_offset":143608,"ancestor_record_offset":141208,"inlinee":7237,"source_line":232,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":142672,"parent_record_offset":141316,"ancestor_record_offset":141208,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":143908,"parent_record_offset":143884,"ancestor_record_offset":141208,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1440,"record_offset":141368,"parent_record_offset":141316,"ancestor_record_offset":141208,"inlinee":25385,"source_line":458,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":142744,"parent_record_offset":142720,"ancestor_record_offset":141208,"inlinee":50427,"source_line":1148,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":142720,"parent_record_offset":142696,"ancestor_record_offset":141208,"inlinee":66600,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":142768,"parent_record_offset":142744,"ancestor_record_offset":141208,"inlinee":50429,"source_line":1301,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":141316,"parent_record_offset":141276,"ancestor_record_offset":141208,"inlinee":50437,"source_line":610,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":143456,"parent_record_offset":143428,"ancestor_record_offset":141208,"inlinee":50438,"source_line":626,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1440,"record_offset":142820,"parent_record_offset":142792,"ancestor_record_offset":141208,"inlinee":63382,"source_line":97,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1440,"record_offset":142696,"parent_record_offset":142672,"ancestor_record_offset":141208,"inlinee":66599,"source_line":97,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10423
// 首条位置: {"module":1440,"inlinee":66595}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::impl$6::next_element
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":121148,"parent_record_offset":121120,"ancestor_record_offset":121120,"inlinee":9782,"source_line":1753,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1440,"record_offset":121232,"parent_record_offset":121204,"ancestor_record_offset":121120,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1440,"record_offset":121292,"parent_record_offset":121232,"ancestor_record_offset":121120,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1440,"record_offset":121260,"parent_record_offset":121232,"ancestor_record_offset":121120,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":121380,"parent_record_offset":121204,"ancestor_record_offset":121120,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":121176,"parent_record_offset":121148,"ancestor_record_offset":121120,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":121324,"parent_record_offset":121204,"ancestor_record_offset":121120,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":121348,"parent_record_offset":121324,"ancestor_record_offset":121120,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$7::next_element_seed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":121204,"parent_record_offset":121176,"ancestor_record_offset":121120,"inlinee":24451,"source_line":575,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=2
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":60,"record_offset":1988,"rva":527888,"va":5369237008}
// 编译实例 RVAs: 0x80e10, 0xf24fe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140017e80
//     owner=unresolved_namespace; count=1
//     target_aliases=13 candidates; complete list: wall_space.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-430396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369237008,"raw_hex":"e8c46ef9ff","size":5,"soft_fail":false,"target_va":5368807040,"va":5369237431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401565c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_space.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t874324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369237008,"raw_hex":"e854570d00","size":5,"soft_fail":false,"target_va":5370111424,"va":5369237095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140361670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_space.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3016577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369237008,"raw_hex":"e881072e00","size":5,"soft_fail":false,"target_va":5372253808,"va":5369237226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: wall_space.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3512452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369237008,"raw_hex":"e884983500","size":5,"soft_fail":false,"target_va":5372749712,"va":5369237255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405cac50
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: wall_space.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9807164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384589280,"raw_hex":"e8c45a6aff","size":5,"soft_fail":false,"target_va":5374782544,"va":5384589703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_space.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8697631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384589280,"raw_hex":"e8e1487bff","size":5,"soft_fail":false,"target_va":5375891872,"va":5384589498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406fa580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_space.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6788682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369237008,"raw_hex":"e84a966700","size":5,"soft_fail":false,"target_va":5376025984,"va":5369237297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141096f40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_space.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1515268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384589280,"raw_hex":"e8041f1700","size":5,"soft_fail":false,"target_va":5386104640,"va":5384589367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":60,"record_offset":2180,"parent_record_offset":1988,"ancestor_record_offset":1988,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":60,"record_offset":2152,"parent_record_offset":1988,"ancestor_record_offset":1988,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":60,"record_offset":2564,"parent_record_offset":2540,"ancestor_record_offset":1988,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":60,"record_offset":2540,"parent_record_offset":2484,"ancestor_record_offset":1988,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":60,"record_offset":2256,"parent_record_offset":2232,"ancestor_record_offset":1988,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":60,"record_offset":2312,"parent_record_offset":2256,"ancestor_record_offset":1988,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":60,"record_offset":2512,"parent_record_offset":2484,"ancestor_record_offset":1988,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":60,"record_offset":2484,"parent_record_offset":2456,"ancestor_record_offset":1988,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":60,"record_offset":2344,"parent_record_offset":2256,"ancestor_record_offset":1988,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":60,"record_offset":2208,"parent_record_offset":2180,"ancestor_record_offset":1988,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":60,"record_offset":2280,"parent_record_offset":2256,"ancestor_record_offset":1988,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::impl$12::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1399,"record_offset":7288,"rva":16990608,"va":5385699728}
// 编译实例 RVAs: 0x1034190, 0x17f0d00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field2_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8115633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385699728,"raw_hex":"e84f2a84ff","size":5,"soft_fail":false,"target_va":5377584208,"va":5385699836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::impl$14::eq
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF09D
// 首条位置: {"module":1309,"inlinee":61597}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::WallSpaceCoord::new
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x54F5
// 首条位置: {"module":383,"inlinee":21749}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::WallSpaceCoord::from_world_space
// 记录形式: compiled_procedure; instances=1
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1261,"record_offset":1420,"rva":13547248,"va":5382256368}
// 编译实例 RVAs: 0xceb6f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1412c5db0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_space.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6137500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382256368,"raw_hex":"e89ca65d00","size":5,"soft_fail":false,"target_va":5388393904,"va":5382256399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1261,"record_offset":1556,"parent_record_offset":1420,"ancestor_record_offset":1420,"inlinee":21347,"source_line":205,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1261,"record_offset":1580,"parent_record_offset":1556,"ancestor_record_offset":1420,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::WallSpaceCoord::from_curve_internal_coord
// 记录形式: compiled_procedure; instances=1
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1261,"record_offset":1616,"rva":13547328,"va":5382256448}
// 编译实例 RVAs: 0xceb740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20722567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382256448,"raw_hex":"e887333c01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382256580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20722798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382256448,"raw_hex":"e86e343c01","size":5,"soft_fail":false,"target_va":5402979344,"va":5382256541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::curve::Curve::get_u_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1261,"record_offset":1760,"parent_record_offset":1616,"ancestor_record_offset":1616,"inlinee":21780,"source_line":664,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1261,"record_offset":1808,"parent_record_offset":1760,"ancestor_record_offset":1616,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1261,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":1616,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1261,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":1616,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1261,"record_offset":2068,"parent_record_offset":2044,"ancestor_record_offset":1616,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1261,"record_offset":2044,"parent_record_offset":2012,"ancestor_record_offset":1616,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1261,"record_offset":1832,"parent_record_offset":1808,"ancestor_record_offset":1616,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1261,"record_offset":1856,"parent_record_offset":1832,"ancestor_record_offset":1616,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1261,"record_offset":1784,"parent_record_offset":1760,"ancestor_record_offset":1616,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1261,"record_offset":1892,"parent_record_offset":1760,"ancestor_record_offset":1616,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1261,"record_offset":2012,"parent_record_offset":1760,"ancestor_record_offset":1616,"inlinee":21781,"source_line":134,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] utils::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1261,"record_offset":2152,"parent_record_offset":2012,"ancestor_record_offset":1616,"inlinee":36057,"source_line":263,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::WallSpaceCoord::x
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 141
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9EE
// 首条位置: {"module":1248,"inlinee":59886}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::WallSpaceCoord::as_vec2
// 记录形式: inline_instance, inlinee_declaration; instances=23
// 源码行: 146
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x666E
// 首条位置: {"module":457,"inlinee":26222}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::WallSpaceCoord::to_world_space
// 记录形式: compiled_procedure; instances=1
// 源码行: 150
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1261,"record_offset":2192,"rva":13547472,"va":5382256592}
// 编译实例 RVAs: 0xceb7d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1412c5ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_space.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6136903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382256592,"raw_hex":"e847a45d00","size":5,"soft_fail":false,"target_va":5388393632,"va":5382256724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t20721937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382256592,"raw_hex":"e811313c01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382257210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20722168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382256592,"raw_hex":"e8f8313c01","size":5,"soft_fail":false,"target_va":5402979344,"va":5382257171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20721641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382256592,"raw_hex":"e8e92f3c01","size":5,"soft_fail":false,"target_va":5402978944,"va":5382257298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1261,"record_offset":4116,"parent_record_offset":2192,"ancestor_record_offset":2192,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1261,"record_offset":4432,"parent_record_offset":2192,"ancestor_record_offset":2192,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1261,"record_offset":4344,"parent_record_offset":2192,"ancestor_record_offset":2192,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1261,"record_offset":3308,"parent_record_offset":2192,"ancestor_record_offset":2192,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1261,"record_offset":2600,"parent_record_offset":2192,"ancestor_record_offset":2192,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1261,"record_offset":3816,"parent_record_offset":2192,"ancestor_record_offset":2192,"inlinee":21578,"source_line":637,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1261,"record_offset":2364,"parent_record_offset":2192,"ancestor_record_offset":2192,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1261,"record_offset":2392,"parent_record_offset":2192,"ancestor_record_offset":2192,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1261,"record_offset":3248,"parent_record_offset":2192,"ancestor_record_offset":2192,"inlinee":21347,"source_line":205,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1261,"record_offset":3148,"parent_record_offset":2192,"ancestor_record_offset":2192,"inlinee":21345,"source_line":189,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1261,"record_offset":2676,"parent_record_offset":2624,"ancestor_record_offset":2192,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1261,"record_offset":2480,"parent_record_offset":2456,"ancestor_record_offset":2192,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1261,"record_offset":2504,"parent_record_offset":2480,"ancestor_record_offset":2192,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1261,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":2192,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_infinite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1261,"record_offset":3216,"parent_record_offset":3148,"ancestor_record_offset":2192,"inlinee":21346,"source_line":535,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1261,"record_offset":2540,"parent_record_offset":2392,"ancestor_record_offset":2192,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1261,"record_offset":2568,"parent_record_offset":2392,"ancestor_record_offset":2192,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1261,"record_offset":3016,"parent_record_offset":2920,"ancestor_record_offset":2192,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1261,"record_offset":3072,"parent_record_offset":2920,"ancestor_record_offset":2192,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1261,"record_offset":2960,"parent_record_offset":2920,"ancestor_record_offset":2192,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1261,"record_offset":2456,"parent_record_offset":2432,"ancestor_record_offset":2192,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1261,"record_offset":2432,"parent_record_offset":2392,"ancestor_record_offset":2192,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1261,"record_offset":2700,"parent_record_offset":2676,"ancestor_record_offset":2192,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1261,"record_offset":2724,"parent_record_offset":2700,"ancestor_record_offset":2192,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1261,"record_offset":4004,"parent_record_offset":3972,"ancestor_record_offset":2192,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1261,"record_offset":3972,"parent_record_offset":3900,"ancestor_record_offset":2192,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1261,"record_offset":2896,"parent_record_offset":2872,"ancestor_record_offset":2192,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1261,"record_offset":3872,"parent_record_offset":3844,"ancestor_record_offset":2192,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1261,"record_offset":2920,"parent_record_offset":2896,"ancestor_record_offset":2192,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1261,"record_offset":3900,"parent_record_offset":3872,"ancestor_record_offset":2192,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1261,"record_offset":4240,"parent_record_offset":4208,"ancestor_record_offset":2192,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1261,"record_offset":4208,"parent_record_offset":4140,"ancestor_record_offset":2192,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1261,"record_offset":4140,"parent_record_offset":4116,"ancestor_record_offset":2192,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1261,"record_offset":4036,"parent_record_offset":4004,"ancestor_record_offset":2192,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1261,"record_offset":2624,"parent_record_offset":2600,"ancestor_record_offset":2192,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1261,"record_offset":2648,"parent_record_offset":2624,"ancestor_record_offset":2192,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1261,"record_offset":2760,"parent_record_offset":2624,"ancestor_record_offset":2192,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1261,"record_offset":2872,"parent_record_offset":2624,"ancestor_record_offset":2192,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::normalize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1261,"record_offset":3844,"parent_record_offset":3816,"ancestor_record_offset":2192,"inlinee":22391,"source_line":65,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::WallSpaceCoord::get_tangent
// 记录形式: compiled_procedure, inlinee_declaration; instances=2
// 源码行: 176
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEA73
// 首条位置: {"module":1261,"inlinee":60019}
// 编译实例 RVAs: 0xcebad0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1412c5ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_space.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6136188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382257360,"raw_hex":"e87ca15d00","size":5,"soft_fail":false,"target_va":5388393632,"va":5382257439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20721591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382257360,"raw_hex":"e8b72f3c01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382257556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20721807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382257360,"raw_hex":"e88f303c01","size":5,"soft_fail":false,"target_va":5402979344,"va":5382257532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1261,"record_offset":4908,"parent_record_offset":4612,"ancestor_record_offset":4612,"inlinee":21578,"source_line":637,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1261,"record_offset":4848,"parent_record_offset":4612,"ancestor_record_offset":4612,"inlinee":21347,"source_line":205,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1261,"record_offset":4752,"parent_record_offset":4612,"ancestor_record_offset":4612,"inlinee":21345,"source_line":189,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1261,"record_offset":4996,"parent_record_offset":4936,"ancestor_record_offset":4612,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1261,"record_offset":4872,"parent_record_offset":4848,"ancestor_record_offset":4612,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_infinite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1261,"record_offset":4816,"parent_record_offset":4752,"ancestor_record_offset":4612,"inlinee":21346,"source_line":535,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1261,"record_offset":4788,"parent_record_offset":4752,"ancestor_record_offset":4612,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1261,"record_offset":5332,"parent_record_offset":5200,"ancestor_record_offset":4612,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1261,"record_offset":5116,"parent_record_offset":5092,"ancestor_record_offset":4612,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1261,"record_offset":5024,"parent_record_offset":4996,"ancestor_record_offset":4612,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1261,"record_offset":5052,"parent_record_offset":5024,"ancestor_record_offset":4612,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1261,"record_offset":5264,"parent_record_offset":5232,"ancestor_record_offset":4612,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1261,"record_offset":5232,"parent_record_offset":5200,"ancestor_record_offset":4612,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1261,"record_offset":5200,"parent_record_offset":5176,"ancestor_record_offset":4612,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1261,"record_offset":5176,"parent_record_offset":5152,"ancestor_record_offset":4612,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1261,"record_offset":5092,"parent_record_offset":4936,"ancestor_record_offset":4612,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1261,"record_offset":5296,"parent_record_offset":5264,"ancestor_record_offset":4612,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_unnormalized_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1261,"record_offset":4936,"parent_record_offset":4908,"ancestor_record_offset":4612,"inlinee":21585,"source_line":633,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1261,"record_offset":4968,"parent_record_offset":4936,"ancestor_record_offset":4612,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::normalize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1261,"record_offset":5152,"parent_record_offset":4908,"ancestor_record_offset":4612,"inlinee":22391,"source_line":65,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::WallSpaceCoord::to_affine3
// 记录形式: inlinee_declaration; instances=1
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEA74
// 首条位置: {"module":1261,"inlinee":60020}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::WallSpaceCoord::with_offset
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x54EF
// 首条位置: {"module":383,"inlinee":21743}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$40::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":25348,"parent_record_offset":25320,"ancestor_record_offset":25320,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::WallSpaceCoord::offset_x_clamped
// 记录形式: inline_instance, inlinee_declaration; instances=13
// 源码行: 205
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEA77
// 首条位置: {"module":1261,"inlinee":60023}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::WallSpaceCoord::with_y_offset
// 记录形式: compiled_procedure, inlinee_declaration; instances=2
// 源码行: 216, 219
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEA79
// 首条位置: {"module":1261,"inlinee":60025}
// 编译实例 RVAs: 0xcebbc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::WallSpaceCoord::with_y
// 记录形式: compiled_procedure; instances=1
// 源码行: 224
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1261,"record_offset":5536,"rva":13548496,"va":5382257616}
// 编译实例 RVAs: 0xcebbd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::WallSpaceCoord::with_offset_unbounded
// 记录形式: inline_instance, inlinee_declaration; instances=17
// 源码行: 231
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEA7A
// 首条位置: {"module":1261,"inlinee":60026}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::WallSpaceCoord::with_offset_clamped
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 241
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x54EC
// 首条位置: {"module":383,"inlinee":21740}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::WallSpaceCoord::is_bounded
// 记录形式: inlinee_declaration; instances=1
// 源码行: 249
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEA7C
// 首条位置: {"module":1261,"inlinee":60028}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::WallSpaceCoord::bound
// 记录形式: inlinee_declaration; instances=1
// 源码行: 256
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEA7D
// 首条位置: {"module":1261,"inlinee":60029}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::WallSpaceCoord::subtract
// 记录形式: inline_instance, inlinee_declaration; instances=13
// 源码行: 268
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x54F7
// 首条位置: {"module":383,"inlinee":21751}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::impl$3::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 286
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1261,"record_offset":5656,"rva":11552,"va":5368720672}
// 编译实例 RVAs: 0x2d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::wall_space::impl$25::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 394
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15E9C
// 首条位置: {"module":2745,"inlinee":89756}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$46::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2745,"record_offset":22284,"parent_record_offset":22228,"ancestor_record_offset":22228,"inlinee":5676,"source_line":18,"archive":"_pdb/A/modules/02745.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::impl$14::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":2745,"record_offset":22256,"parent_record_offset":22228,"ancestor_record_offset":22228,"inlinee":61597,"source_line":97,"archive":"_pdb/A/modules/02745.jsonl.gz"}
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
// 完整结构及原始类型记录: wall_space.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::wall_space::WallBoundCoord
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 14; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::wall_space::WallSpaceCoord
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 20; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<utils::wall_space::WallSpaceCoord>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["utils::wall_space::WallSpaceCoord"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::wall_space::WallSpaceRegion
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "tuple$<utils::wall_space::WallSpaceRegion,country_core::resources::walls::wall_holes::HoleType>", "constructor": "tuple$", "arguments": ["utils::wall_space::WallSpaceRegion", "country_core::resources::walls::wall_holes::HoleType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<tuple$<utils::wall_space::WallSpaceRegion,country_core::resources::walls::wall_holes::HoleType>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["tuple$<utils::wall_space::WallSpaceRegion,country_core::resources::walls::wall_holes::HoleType>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<tuple$<utils::wall_space::WallSpaceRegion,country_core::resources::walls::wall_holes::HoleType>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["tuple$<utils::wall_space::WallSpaceRegion,country_core::resources::walls::wall_holes::HoleType>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<utils::wall_space::WallSpaceRegion,country_core::resources::walls::wall_holes::HoleType>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<utils::wall_space::WallSpaceRegion,country_core::resources::walls::wall_holes::HoleType>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::walls::wall_holes::InProgressHoleStorage::extend<alloc::vec::Vec<tuple$<utils::wall_space::WallSpaceRegion,country_core::resources::walls::wall_holes::HoleType>,alloc::alloc::Global> >", "constructor": "country_core::resources::walls::wall_holes::InProgressHoleStorage::extend", "arguments": ["alloc::vec::Vec<tuple$<utils::wall_space::WallSpaceRegion,country_core::resources::walls::wall_holes::HoleType>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<tuple$<utils::wall_space::WallSpaceRegion,country_core::resources::walls::wall_holes::HoleType> >", "constructor": "core::slice::iter::Iter", "arguments": ["tuple$<utils::wall_space::WallSpaceRegion,country_core::resources::walls::wall_holes::HoleType>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::Copied<core::slice::iter::Iter<tuple$<utils::wall_space::WallSpaceRegion,country_core::resources::walls::wall_holes::HoleType> > >", "constructor": "core::iter::adapters::copied::Copied", "arguments": ["core::slice::iter::Iter<tuple$<utils::wall_space::WallSpaceRegion,country_core::resources::walls::wall_holes::HoleType> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::walls::wall_holes::WallHoles::extend<core::iter::adapters::copied::Copied<core::slice::iter::Iter<tuple$<utils::wall_space::WallSpaceRegion,country_core::resources::walls::wall_holes::HoleType> > > >", "constructor": "country_core::resources::walls::wall_holes::WallHoles::extend", "arguments": ["core::iter::adapters::copied::Copied<core::slice::iter::Iter<tuple$<utils::wall_space::WallSpaceRegion,country_core::resources::walls::wall_holes::HoleType> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_space.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_space.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 139; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec2::Vec2)
// 候选参数类型: ["glam::f32::vec2::Vec2"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: utils::wall_space::WallSpaceCoord::as_vec2
// 信息 ID: 72fa560d01bf20d09d3ab4ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 14 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::wall_holes::WallHoleStorage::finalize; module=457, inline_record=32128; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_holes::WallHoleStorage::finalize; module=457, inline_record=32820; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback<country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$0,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$1,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$2>; module=476, inline_record=37908; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback<country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$0,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$1,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$2>; module=476, inline_record=37936; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback<country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$0,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$1,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$2>; module=476, inline_record=37964; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback<country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$0,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$1,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$2>; module=476, inline_record=37992; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback<country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$0,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$1,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$2>; module=476, inline_record=38720; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback<country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$0,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$1,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$2>; module=476, inline_record=38748; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::WallSpaceCoord::bound
// 信息 ID: bad9eb8a3c36cf9665133450
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::WallSpaceCoord::from_curve_internal_coord
// 信息 ID: d219f9b4e748a65670e436d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_on_freehand_wall_edited::decorator_on_freehand_wall_edited @ 0x4fc6e7 -> 0xceb740 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::WallSpaceCoord::from_world_space
// 信息 ID: 66abafc1d4a028f7e84d8110
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4f1d48 -> 0xceb6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4f1da1 -> 0xceb6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::recompute_quads @ 0xcdfc04 -> 0xceb6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::recompute_quads @ 0xcdfc30 -> 0xceb6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::add_hole_at_shape_intersection @ 0x14dd1e3 -> 0xceb6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::add_hole_at_shape_intersection @ 0x14dd20c -> 0xceb6f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::WallSpaceCoord::get_tangent
// 信息 ID: ff4f9665134d288c9be75272
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x4fd507 -> 0xcebad0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$3 @ 0x1e6abfa -> 0xcebad0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::WallSpaceCoord::is_bounded
// 信息 ID: 87c047b85bcd9c0c22966d82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::WallSpaceCoord::new
// 信息 ID: 62e517949d7397685f81b972
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=26724; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=27028; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=27592; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=27620; 不是独立调用边。
// [PDB 内联宿主] system_decorator::move_decorators_following_anchors::move_decorators_following_anchors; module=385, inline_record=5996; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1286, inline_record=4004; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1286, inline_record=4132; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1286, inline_record=4264; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::WallSpaceCoord::offset_x_clamped
// 信息 ID: 17e76cab57f21e3ebb9ef496
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::get_gothic_hat_bones; module=3842, inline_record=23900; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::get_gothic_hat_bones; module=3842, inline_record=23928; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors; module=3854, inline_record=11168; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors; module=3854, inline_record=11196; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows; module=3858, inline_record=8848; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows; module=3858, inline_record=8876; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows; module=3863, inline_record=15040; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows; module=3863, inline_record=15096; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::WallSpaceCoord::subtract
// 信息 ID: d913d6e622ed62ca24a980a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=26868; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=26940; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=26968; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=26996; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::should_snap_to_corner; module=383, inline_record=34468; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1286, inline_record=4072; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1286, inline_record=4204; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1286, inline_record=4336; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::WallSpaceCoord::to_affine3
// 信息 ID: 6b64d001ddcbe978cb13e6b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::WallSpaceCoord::to_world_space
// 信息 ID: 9de53e3877c07aacefa85db7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 35 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x4fd520 -> 0xceb7d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x500e6d -> 0xceb7d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccbe36 -> 0xceb7d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd09b8 -> 0xceb7d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd0a1a -> 0xceb7d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd006f -> 0xceb7d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd293d -> 0xceb7d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd29a1 -> 0xceb7d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_skirts::add_skirts @ 0xcd860f -> 0xceb7d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> > @ 0xce46ad -> 0xceb7d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::add_underwindow_pattern<system_wall_constructor::utils::half_timber::half_timbering::closure_env$0> @ 0x17756d1 -> 0xceb7d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::add_underwindow_pattern<system_wall_constructor::utils::half_timber::half_timbering::closure_env$0> @ 0x17756fa -> 0xceb7d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$1> > @ 0x17b03af -> 0xceb7d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$1> > @ 0x17b05be -> 0xceb7d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,core::iter::sources::once::Once<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0> > > > @ 0x17d448b -> 0xceb7d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,core::iter::sources::once::Once<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0> > > > @ 0x17d43f2 -> 0xceb7d0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::WallSpaceCoord::with_offset
// 信息 ID: ad178d16398bc873bda52abc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=25320; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=25380; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1286, inline_record=5620; 不是独立调用边。
// [PDB 内联宿主] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner; module=2776, inline_record=3732; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$3; module=4379, inline_record=15468; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::WallSpaceCoord::with_offset_clamped
// 信息 ID: 461a8524bc6d9c038fef05d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23260; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23568; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23876; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners; module=1248, inline_record=5332; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners; module=1248, inline_record=6972; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners; module=1248, inline_record=7028; 不是独立调用边。
// [PDB 内联宿主] system_clutter::inter_shape_stitches::detect_intra_shape_corners::add_hole_at_shape_intersection; module=2269, inline_record=20460; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::WallSpaceCoord::with_offset_unbounded
// 信息 ID: c4402209082bd9b53a8f86d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$1> >; module=2756, inline_record=28788; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$1> >; module=2756, inline_record=30512; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,core::iter::sources::once::Once<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0> > > >; module=2767, inline_record=7800; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,core::iter::sources::once::Once<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0> > > >; module=2767, inline_record=8908; 不是独立调用边。
// [PDB 内联宿主] system_decorator::derive_decorator_subtype::derive_decorator_subtypes; module=2773, inline_record=11336; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows; module=3863, inline_record=16096; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows; module=3863, inline_record=16484; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=3865, inline_record=10744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::WallSpaceCoord::with_y
// 信息 ID: b986031e29e15d8a7158f72b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb26e -> 0xcebbd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::WallSpaceCoord::with_y_offset
// 信息 ID: 981d0fab7686908b2ab8f533
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> > @ 0xce3c1c -> 0xcebbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::add_lintels @ 0x1c6385c -> 0xcebbc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::WallSpaceCoord::x
// 信息 ID: 6b8b4fa5636f6efa47e69967
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners; module=1248, inline_record=7908; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners; module=1248, inline_record=7936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::WallSpaceRegion::intersects
// 信息 ID: b98c40fa0eb9f0d5b4d23e0f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::derive_decorator_subtype::derive_decorator_subtypes; module=2773, inline_record=12368; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; module=2775, inline_record=1500; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; module=2775, inline_record=3068; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; module=2775, inline_record=4328; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; module=2775, inline_record=5592; 不是独立调用边。
// [PDB 内联宿主] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner; module=2776, inline_record=3348; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::flag::generate_flags; module=3859, inline_record=16960; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=3865, inline_record=12720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::_::impl$0::deserialize
// 信息 ID: 2fa7d6b63bdbe0f7d7c1addb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::walls::decorator::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=134076; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::walls::decorator::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=213528; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::walls::decorator::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=213744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: dbe07918ecee8aeb697b3f80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<utils::wall_space::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=142820; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 58f3859b666ec5d5cbc37bfb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<utils::wall_space::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=142696; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 778a37169109059ac3374ab2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impl$1::fmt<utils::wall_space::_::impl$0::deserialize::__Visitor> @ 0xf24f90 -> 0xcebbe0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 282762ea4ebddaa9077f98d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<utils::wall_space::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=141208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 2430548293361b93a045a450
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<utils::wall_space::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=121120; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: 6afefa18feecbb5b35f9b6cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::wall_space::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<utils::wall_space::WallSpaceCoord> @ 0x1528fb -> 0x80e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<utils::wall_space::WallSpaceCoord> @ 0x109b39b -> 0xf24fe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::impl$0::distance
// 信息 ID: 656d41f957dda23353b1f4b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::derive_decorator_subtype::derive_decorator_subtypes @ 0x17d6daf -> 0xceb490 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner @ 0x17d9546 -> 0xceb490 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::flag::generate_flags @ 0x1c57d36 -> 0xceb490 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys @ 0x1c668c7 -> 0xceb490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::impl$0::project
// 信息 ID: 7445bc74173ee505d15c418d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x500f71 -> 0xceb4e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x500f93 -> 0xceb4e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0ae6a -> 0xceb4e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner @ 0x17d9605 -> 0xceb4e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$3 @ 0x1e6ab46 -> 0xceb4e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::impl$0::project_clamp
// 信息 ID: 7e06a15bd64a664c481e9c0a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 28 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x500bea -> 0xceb560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x500c9e -> 0xceb560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x500d3c -> 0xceb560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x501c84 -> 0xceb560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x501d55 -> 0xceb560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x50200e -> 0xceb560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x502155 -> 0xceb560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::move_decorators_following_anchors::move_decorators_following_anchors @ 0x503714 -> 0xceb560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments @ 0xcb85b3 -> 0xceb560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments @ 0xcb86e2 -> 0xceb560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> > @ 0xcbaa01 -> 0xceb560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_arches::construct_arches::closure_env$3> @ 0xcbb4d0 -> 0xceb560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb240 -> 0xceb560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccbaa1 -> 0xceb560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccbadf -> 0xceb560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0a7bc -> 0xceb560 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::impl$0::project_unbounded
// 信息 ID: 2812b968efae86ac87488c79
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$1> > @ 0x17b038a -> 0xceb650 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$1> > @ 0x17b0596 -> 0xceb650 (direct_call); 同地址别名未裁定。
// [静态直接引用] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,core::iter::sources::once::Once<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0> > > > @ 0x17d446c -> 0xceb650 (direct_call); 同地址别名未裁定。
// [静态直接引用] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,core::iter::sources::once::Once<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0> > > > @ 0x17d43d3 -> 0xceb650 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::derive_decorator_subtype::derive_decorator_subtypes @ 0x17d6ba5 -> 0xceb650 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x1c61857 -> 0xceb650 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x1c618d8 -> 0xceb650 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys @ 0x1c66640 -> 0xceb650 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::impl$0::subtract
// 信息 ID: ade10c19da9e16bb1e87a23b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 14 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x501cf9 -> 0xceb6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x501fb2 -> 0xceb6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x5020f9 -> 0xceb6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x5021e8 -> 0xceb6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::should_snap_to_corner @ 0x502a5f -> 0xceb6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments @ 0xcb858c -> 0xceb6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments @ 0xcb86bb -> 0xceb6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> > @ 0xcba9ed -> 0xceb6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_arches::construct_arches::closure_env$3> @ 0xcbb4bb -> 0xceb6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0a81b -> 0xceb6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0a8fa -> 0xceb6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0abe4 -> 0xceb6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::add_hole_at_shape_intersection @ 0x14dd237 -> 0xceb6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner @ 0x17d95be -> 0xceb6a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::impl$12::fmt
// 信息 ID: a12f77a15320edcc25c66cbf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::impl$14::eq
// 信息 ID: 873e24565472ceb8d6887c3a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::convert_before_after_states_to_changes; module=1309, inline_record=109192; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::impl$94::extend<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0> >; module=1370, inline_record=12108; 不是独立调用边。
// [PDB 内联宿主] system_decorator::blueprint::instantiate_blueprints; module=2745, inline_record=22256; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::impl$25::eq
// 信息 ID: 89f07592fc282e77dc48d27e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::blueprint::instantiate_blueprints; module=2745, inline_record=22228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::impl$3::from
// 信息 ID: 316766fe0e66436ae2c4a76f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 03d77369acb10f9e8a8c49c5: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>)
// [泛型宿主/闭包/直接引用方参数候选] 076821a8e8d882e6d45dbb01: void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::WindowClutterCandidates>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>)
// [泛型宿主/闭包/直接引用方参数候选] 0c4383e19e8e41ddd102d7ce: void (*)(bevy_ecs::event::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// [泛型宿主/闭包/直接引用方参数候选] 0f7c39de2bbbbcade72fdc5b: void (*)(bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::rt::RtBvhBuilderGlueResource>,bevy_ecs::change_detection::Res<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::With<country_core::resources::critters::Critter> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,tuple$<bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 121a48a843a3661860cc0c60: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// [泛型宿主/闭包/直接引用方参数候选] 188ee5116e6a2f635b4a8b5b: void (*)(bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::BirdHaven>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::system::system_param::Local<system_critters::wren::WrenGen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Wren> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::BirdSpots>,tuple$<> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 1ac3b09f94008d55466c958a: void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// [泛型宿主/闭包/直接引用方参数候选] 1bdb6576ac14098c1ed7fdf1: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 其余候选接口见完整附档，共 115 条。
// 引用方: 原程序直接转移 514 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x12d50 -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x12dc0 -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x5d93c -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64cf3 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64d55 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0x6525e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x6535e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo> @ 0x6545e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo> @ 0x6555e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x664ec -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x6655c -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >),void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >)> @ 0x8b872 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)> @ 0x8bb13 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x8bd82 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands)> @ 0x8bee1 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x8c110 -> 0x2d20 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::impl$7::fmt
// 信息 ID: 33c8f4a547fa3d70b1e66c17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::wall_space::impl$9::eq
// 信息 ID: be65aa88d8cc5669d4a3d7b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
