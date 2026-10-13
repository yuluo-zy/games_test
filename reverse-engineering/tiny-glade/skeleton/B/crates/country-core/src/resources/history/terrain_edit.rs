// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/history/terrain_edit.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 5a41d018a5a636ccfa3b3dc3649ddb43ea6bf1b6e978b822458df40b322a7fa3
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: terrain_edit.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 199, 479, 524, 1585, 1605, 1610, 1630, 1729
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 20, 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12239
// 首条位置: {"module":1630,"inlinee":74297}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":258392,"parent_record_offset":258360,"ancestor_record_offset":258360,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":258416,"parent_record_offset":258392,"ancestor_record_offset":258360,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":258440,"parent_record_offset":258416,"ancestor_record_offset":258360,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":258464,"parent_record_offset":258440,"ancestor_record_offset":258360,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 20, 28, 36, 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12238
// 首条位置: {"module":1630,"inlinee":74296}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::value::impl$26::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":504204,"parent_record_offset":504176,"ancestor_record_offset":504176,"inlinee":74331,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":258216,"parent_record_offset":258188,"ancestor_record_offset":258188,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":258984,"parent_record_offset":258960,"ancestor_record_offset":258188,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":258960,"parent_record_offset":258880,"ancestor_record_offset":258188,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":258920,"parent_record_offset":258880,"ancestor_record_offset":258188,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":258880,"parent_record_offset":258844,"ancestor_record_offset":258188,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":258844,"parent_record_offset":258808,"ancestor_record_offset":258188,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":258416,"parent_record_offset":258392,"ancestor_record_offset":258188,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":258736,"parent_record_offset":258312,"ancestor_record_offset":258188,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":258440,"parent_record_offset":258416,"ancestor_record_offset":258188,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":258464,"parent_record_offset":258440,"ancestor_record_offset":258188,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":258392,"parent_record_offset":258360,"ancestor_record_offset":258188,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":1630,"record_offset":258360,"parent_record_offset":258312,"ancestor_record_offset":258188,"inlinee":74297,"source_line":20,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":258312,"parent_record_offset":258280,"ancestor_record_offset":258188,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$26::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1630,"record_offset":504232,"parent_record_offset":504204,"ancestor_record_offset":504176,"inlinee":74332,"source_line":615,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":258248,"parent_record_offset":258216,"ancestor_record_offset":258188,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":258280,"parent_record_offset":258248,"ancestor_record_offset":258188,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13016
// 首条位置: {"module":1729,"inlinee":77846}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1729,"record_offset":57044,"parent_record_offset":56516,"ancestor_record_offset":56516,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1729,"record_offset":56596,"parent_record_offset":56516,"ancestor_record_offset":56516,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1729,"record_offset":56632,"parent_record_offset":56516,"ancestor_record_offset":56516,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1729,"record_offset":56568,"parent_record_offset":56516,"ancestor_record_offset":56516,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] serde::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1729,"record_offset":56660,"parent_record_offset":56516,"ancestor_record_offset":56516,"inlinee":5585,"source_line":2117,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":1729,"record_offset":56748,"parent_record_offset":56724,"ancestor_record_offset":56516,"inlinee":74544,"source_line":28,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1729,"record_offset":56724,"parent_record_offset":56688,"ancestor_record_offset":56516,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$5::newtype_variant_seed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1729,"record_offset":56688,"parent_record_offset":56660,"ancestor_record_offset":56516,"inlinee":31380,"source_line":502,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1729,"record_offset":55636,"rva":24910528,"va":5393619648}
// 编译实例 RVAs: 0x17c1ac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t17857879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393619648,"raw_hex":"e8577d1001","size":5,"soft_fail":false,"target_va":5411478016,"va":5393620132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18539560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393619648,"raw_hex":"e8d81be5fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5393621059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14085da90
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16140294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393619648,"raw_hex":"e8fab709ff","size":5,"soft_fail":false,"target_va":5377481360,"va":5393621649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140887df0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15965695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393619648,"raw_hex":"e801620cff","size":5,"soft_fail":false,"target_va":5377654256,"va":5393619946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888d70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15961978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393619648,"raw_hex":"e886700cff","size":5,"soft_fail":false,"target_va":5377658224,"va":5393620197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140889390
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15961347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393619648,"raw_hex":"e8fd720cff","size":5,"soft_fail":false,"target_va":5377659792,"va":5393621134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140898f40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15896441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393619648,"raw_hex":"e887700dff","size":5,"soft_fail":false,"target_va":5377724224,"va":5393620660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14143c450
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3692301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393619648,"raw_hex":"e8f3a8c7ff","size":5,"soft_fail":false,"target_va":5389927504,"va":5393619800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14159f330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2239666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393619648,"raw_hex":"e84ed3ddff","size":5,"soft_fail":false,"target_va":5391381296,"va":5393620957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415a1130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2231768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393619648,"raw_hex":"e828f2ddff","size":5,"soft_fail":false,"target_va":5391388976,"va":5393620739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415a1930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2229822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393619648,"raw_hex":"e8c2f9ddff","size":5,"soft_fail":false,"target_va":5391391024,"va":5393620841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415d4280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2021824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393619648,"raw_hex":"e84026e1ff","size":5,"soft_fail":false,"target_va":5391598208,"va":5393620027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f9b40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18055026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393619648,"raw_hex":"e8727f1301","size":5,"soft_fail":false,"target_va":5411674944,"va":5393619913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::de::_$LT$impl$u20$serde_json..value..Value$GT$::unexpected
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t17858430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393619648,"raw_hex":"e87e7f1001","size":5,"soft_fail":false,"target_va":5411478544,"va":5393620109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::value::de::impl$2::deserialize_enum
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1729,"record_offset":55828,"parent_record_offset":55636,"ancestor_record_offset":55636,"inlinee":26604,"source_line":255,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1729,"record_offset":57532,"parent_record_offset":57508,"ancestor_record_offset":55636,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1729,"record_offset":57508,"parent_record_offset":57420,"ancestor_record_offset":55636,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1729,"record_offset":56040,"parent_record_offset":56016,"ancestor_record_offset":55636,"inlinee":6033,"source_line":1680,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1729,"record_offset":56096,"parent_record_offset":56072,"ancestor_record_offset":55636,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1729,"record_offset":56148,"parent_record_offset":56120,"ancestor_record_offset":55636,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1729,"record_offset":56172,"parent_record_offset":56148,"ancestor_record_offset":55636,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1729,"record_offset":57280,"parent_record_offset":57256,"ancestor_record_offset":55636,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1729,"record_offset":57448,"parent_record_offset":57420,"ancestor_record_offset":55636,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1729,"record_offset":57420,"parent_record_offset":57396,"ancestor_record_offset":55636,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1729,"record_offset":57328,"parent_record_offset":57304,"ancestor_record_offset":55636,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1729,"record_offset":57304,"parent_record_offset":57280,"ancestor_record_offset":55636,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1729,"record_offset":57396,"parent_record_offset":57372,"ancestor_record_offset":55636,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1729,"record_offset":57256,"parent_record_offset":57232,"ancestor_record_offset":55636,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1729,"record_offset":57232,"parent_record_offset":57204,"ancestor_record_offset":55636,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1729,"record_offset":57476,"parent_record_offset":57448,"ancestor_record_offset":55636,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1729,"record_offset":56120,"parent_record_offset":56096,"ancestor_record_offset":55636,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1729,"record_offset":56288,"parent_record_offset":56200,"ancestor_record_offset":55636,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1729,"record_offset":57172,"parent_record_offset":55828,"ancestor_record_offset":55636,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1729,"record_offset":56312,"parent_record_offset":56288,"ancestor_record_offset":55636,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1729,"record_offset":57044,"parent_record_offset":56516,"ancestor_record_offset":55636,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1729,"record_offset":56596,"parent_record_offset":56516,"ancestor_record_offset":55636,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1729,"record_offset":56632,"parent_record_offset":56516,"ancestor_record_offset":55636,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1729,"record_offset":56200,"parent_record_offset":56172,"ancestor_record_offset":55636,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1729,"record_offset":56256,"parent_record_offset":56232,"ancestor_record_offset":55636,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1729,"record_offset":56232,"parent_record_offset":56200,"ancestor_record_offset":55636,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":1729,"record_offset":56748,"parent_record_offset":56724,"ancestor_record_offset":55636,"inlinee":74544,"source_line":28,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":1729,"record_offset":56516,"parent_record_offset":55828,"ancestor_record_offset":55636,"inlinee":77846,"source_line":20,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1729,"record_offset":56568,"parent_record_offset":56516,"ancestor_record_offset":55636,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1729,"record_offset":56660,"parent_record_offset":56516,"ancestor_record_offset":55636,"inlinee":5585,"source_line":2117,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1729,"record_offset":56724,"parent_record_offset":56688,"ancestor_record_offset":55636,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$17::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1729,"record_offset":56016,"parent_record_offset":55828,"ancestor_record_offset":55636,"inlinee":26606,"source_line":877,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1729,"record_offset":56072,"parent_record_offset":55828,"ancestor_record_offset":55636,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$5::newtype_variant_seed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1729,"record_offset":56688,"parent_record_offset":56660,"ancestor_record_offset":55636,"inlinee":31380,"source_line":502,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":199,"record_offset":8016,"rva":5194160,"va":5373903280}
// 编译实例 RVAs: 0x4f41b0, 0x17c1590
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b5870
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2354364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903280,"raw_hex":"e84413dcff","size":5,"soft_fail":false,"target_va":5371549808,"va":5373904167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b6100
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-254244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903280,"raw_hex":"e8dc1efcff","size":5,"soft_fail":false,"target_va":5373649152,"va":5373903391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404baab0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-235488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903280,"raw_hex":"e82068fcff","size":5,"soft_fail":false,"target_va":5373668016,"va":5373903499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bbdb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-230964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903280,"raw_hex":"e8cc79fcff","size":5,"soft_fail":false,"target_va":5373672880,"va":5373903839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bd170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-226191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903280,"raw_hex":"e8718cfcff","size":5,"soft_fail":false,"target_va":5373677936,"va":5373904122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bdc80
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-222772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903280,"raw_hex":"e8cc99fcff","size":5,"soft_fail":false,"target_va":5373680768,"va":5373903535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f4520
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5034988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393618320,"raw_hex":"e8142cb3ff","size":5,"soft_fail":false,"target_va":5388584224,"va":5393619207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141580d20
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2361572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393618320,"raw_hex":"e81cf7dbff","size":5,"soft_fail":false,"target_va":5391256864,"va":5393618431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141583a40
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2350128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393618320,"raw_hex":"e8d023dcff","size":5,"soft_fail":false,"target_va":5391268416,"va":5393618539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141586090
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2340356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393618320,"raw_hex":"e8fc49dcff","size":5,"soft_fail":false,"target_va":5391278224,"va":5393618575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14158c570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2314836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393618320,"raw_hex":"e8acaddcff","size":5,"soft_fail":false,"target_va":5391304048,"va":5393618879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14158ce20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2312895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393618320,"raw_hex":"e841b5dcff","size":5,"soft_fail":false,"target_va":5391306272,"va":5393619162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=18
//     evidence={"assembly":"\tcallq\t37413976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903280,"raw_hex":"e858e43a02","size":5,"soft_fail":false,"target_va":5411318240,"va":5373904259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] serde_json::error::Error::io
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t37573435","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5373903280,"raw_hex":"e93b533d02","size":5,"soft_fail":false,"target_va":5411477664,"va":5373904224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_newtype_variant
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":8296,"parent_record_offset":8016,"ancestor_record_offset":8016,"inlinee":5418,"source_line":268,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":8700,"parent_record_offset":8668,"ancestor_record_offset":8016,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":8568,"parent_record_offset":8540,"ancestor_record_offset":8016,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":8628,"parent_record_offset":8596,"ancestor_record_offset":8016,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":8748,"parent_record_offset":8724,"ancestor_record_offset":8016,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":8724,"parent_record_offset":8700,"ancestor_record_offset":8016,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":8596,"parent_record_offset":8568,"ancestor_record_offset":8016,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":8500,"parent_record_offset":8472,"ancestor_record_offset":8016,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":8668,"parent_record_offset":8500,"ancestor_record_offset":8016,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":8444,"parent_record_offset":8416,"ancestor_record_offset":8016,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":8788,"parent_record_offset":8500,"ancestor_record_offset":8016,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":8540,"parent_record_offset":8500,"ancestor_record_offset":8016,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":199,"record_offset":8472,"parent_record_offset":8444,"ancestor_record_offset":8016,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":199,"record_offset":8864,"parent_record_offset":8840,"ancestor_record_offset":8016,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::_::impl$0::serialize
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":199,"record_offset":9376,"parent_record_offset":8296,"ancestor_record_offset":8016,"inlinee":14660,"source_line":28,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":8360,"parent_record_offset":8296,"ancestor_record_offset":8016,"inlinee":5420,"source_line":1849,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object_value
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":8896,"parent_record_offset":8296,"ancestor_record_offset":8016,"inlinee":5429,"source_line":1894,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":9436,"parent_record_offset":9408,"ancestor_record_offset":8016,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":8840,"parent_record_offset":8296,"ancestor_record_offset":8016,"inlinee":5182,"source_line":219,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":9408,"parent_record_offset":9376,"ancestor_record_offset":8016,"inlinee":5727,"source_line":409,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":9952,"parent_record_offset":9376,"ancestor_record_offset":8016,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":199,"record_offset":8416,"parent_record_offset":8388,"ancestor_record_offset":8016,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":199,"record_offset":8388,"parent_record_offset":8360,"ancestor_record_offset":8016,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 28, 36, 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12330
// 首条位置: {"module":1630,"inlinee":74544}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 28, 36, 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67CC
// 首条位置: {"module":1729,"inlinee":26572}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 28, 36, 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12236
// 首条位置: {"module":1630,"inlinee":74294}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":259576,"parent_record_offset":256640,"ancestor_record_offset":256640,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":260872,"parent_record_offset":256640,"ancestor_record_offset":256640,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":259884,"parent_record_offset":256640,"ancestor_record_offset":256640,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":260620,"parent_record_offset":256640,"ancestor_record_offset":256640,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":256684,"parent_record_offset":256640,"ancestor_record_offset":256640,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":259300,"parent_record_offset":256640,"ancestor_record_offset":256640,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":257492,"parent_record_offset":257468,"ancestor_record_offset":256640,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":257468,"parent_record_offset":257412,"ancestor_record_offset":256640,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":256836,"parent_record_offset":256812,"ancestor_record_offset":256640,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":256888,"parent_record_offset":256860,"ancestor_record_offset":256640,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":256912,"parent_record_offset":256888,"ancestor_record_offset":256640,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":257848,"parent_record_offset":257824,"ancestor_record_offset":256640,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":257440,"parent_record_offset":257412,"ancestor_record_offset":256640,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":257412,"parent_record_offset":257388,"ancestor_record_offset":256640,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":257896,"parent_record_offset":257872,"ancestor_record_offset":256640,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":257872,"parent_record_offset":257848,"ancestor_record_offset":256640,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":257388,"parent_record_offset":257364,"ancestor_record_offset":256640,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":257824,"parent_record_offset":257800,"ancestor_record_offset":256640,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":257800,"parent_record_offset":257544,"ancestor_record_offset":256640,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":258416,"parent_record_offset":258392,"ancestor_record_offset":256640,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":259416,"parent_record_offset":259392,"ancestor_record_offset":256640,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":257756,"parent_record_offset":257728,"ancestor_record_offset":256640,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":256860,"parent_record_offset":256836,"ancestor_record_offset":256640,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":259392,"parent_record_offset":259356,"ancestor_record_offset":256640,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":257028,"parent_record_offset":256940,"ancestor_record_offset":256640,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=58
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":257256,"parent_record_offset":256756,"ancestor_record_offset":256640,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":257052,"parent_record_offset":257028,"ancestor_record_offset":256640,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":259472,"parent_record_offset":259416,"ancestor_record_offset":256640,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":260232,"parent_record_offset":260180,"ancestor_record_offset":256640,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":258440,"parent_record_offset":258416,"ancestor_record_offset":256640,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":258464,"parent_record_offset":258440,"ancestor_record_offset":256640,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":256940,"parent_record_offset":256912,"ancestor_record_offset":256640,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":256996,"parent_record_offset":256972,"ancestor_record_offset":256640,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":256972,"parent_record_offset":256940,"ancestor_record_offset":256640,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":258392,"parent_record_offset":258360,"ancestor_record_offset":256640,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":1630,"record_offset":258360,"parent_record_offset":258312,"ancestor_record_offset":256640,"inlinee":74297,"source_line":20,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":1630,"record_offset":258188,"parent_record_offset":258156,"ancestor_record_offset":256640,"inlinee":74296,"source_line":20,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":1630,"record_offset":260180,"parent_record_offset":260156,"ancestor_record_offset":256640,"inlinee":28090,"source_line":38,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/features/impl_serde.rs
//     evidence={"module":1630,"record_offset":259840,"parent_record_offset":259816,"ancestor_record_offset":256640,"inlinee":29596,"source_line":18,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":260692,"parent_record_offset":260668,"ancestor_record_offset":256640,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":331548,"parent_record_offset":331524,"ancestor_record_offset":327100,"inlinee":6671,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":260668,"parent_record_offset":260644,"ancestor_record_offset":256640,"inlinee":6672,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_tuple_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":261316,"parent_record_offset":261292,"ancestor_record_offset":256640,"inlinee":6648,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":260804,"parent_record_offset":260780,"ancestor_record_offset":256640,"inlinee":28015,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":256720,"parent_record_offset":256684,"ancestor_record_offset":256640,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":259328,"parent_record_offset":259300,"ancestor_record_offset":256640,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":258312,"parent_record_offset":258280,"ancestor_record_offset":256640,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":259532,"parent_record_offset":259508,"ancestor_record_offset":256640,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":258156,"parent_record_offset":256756,"ancestor_record_offset":256640,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":260544,"parent_record_offset":260520,"ancestor_record_offset":256640,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":260204,"parent_record_offset":260180,"ancestor_record_offset":256640,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":330072,"parent_record_offset":330048,"ancestor_record_offset":327100,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":256812,"parent_record_offset":256756,"ancestor_record_offset":256640,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":258248,"parent_record_offset":258216,"ancestor_record_offset":256640,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":258216,"parent_record_offset":258188,"ancestor_record_offset":256640,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":258280,"parent_record_offset":258248,"ancestor_record_offset":256640,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":256756,"parent_record_offset":256720,"ancestor_record_offset":256640,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":259356,"parent_record_offset":259328,"ancestor_record_offset":256640,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 28, 36, 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1220F
// 首条位置: {"module":1630,"inlinee":74255}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":75920,"parent_record_offset":74816,"ancestor_record_offset":74816,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$6::next_element
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":74856,"parent_record_offset":74816,"ancestor_record_offset":74816,"inlinee":11489,"source_line":1753,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":76132,"parent_record_offset":76108,"ancestor_record_offset":74816,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":76108,"parent_record_offset":76004,"ancestor_record_offset":74816,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":76040,"parent_record_offset":76004,"ancestor_record_offset":74816,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":76004,"parent_record_offset":75976,"ancestor_record_offset":74816,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":75976,"parent_record_offset":75948,"ancestor_record_offset":74816,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1630,"record_offset":74932,"parent_record_offset":74904,"ancestor_record_offset":74816,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":76076,"parent_record_offset":76040,"ancestor_record_offset":74816,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":75948,"parent_record_offset":75920,"ancestor_record_offset":74816,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1630,"record_offset":75004,"parent_record_offset":74932,"ancestor_record_offset":74816,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1630,"record_offset":75032,"parent_record_offset":74932,"ancestor_record_offset":74816,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1630,"record_offset":74972,"parent_record_offset":74932,"ancestor_record_offset":74816,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":75056,"parent_record_offset":75032,"ancestor_record_offset":74816,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":75164,"parent_record_offset":75116,"ancestor_record_offset":74816,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":1630,"record_offset":75116,"parent_record_offset":75092,"ancestor_record_offset":74816,"inlinee":28090,"source_line":38,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/features/impl_serde.rs
//     evidence={"module":1630,"record_offset":75840,"parent_record_offset":75816,"ancestor_record_offset":74816,"inlinee":29596,"source_line":18,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":74880,"parent_record_offset":74856,"ancestor_record_offset":74816,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":75092,"parent_record_offset":74904,"ancestor_record_offset":74816,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":75484,"parent_record_offset":75460,"ancestor_record_offset":74816,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":75136,"parent_record_offset":75116,"ancestor_record_offset":74816,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":98912,"parent_record_offset":98888,"ancestor_record_offset":98256,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$7::next_element_seed
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":74904,"parent_record_offset":74880,"ancestor_record_offset":74816,"inlinee":29591,"source_line":575,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 28, 36, 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3944
// 首条位置: {"module":199,"inlinee":14660}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":9408,"parent_record_offset":9376,"ancestor_record_offset":9376,"inlinee":5727,"source_line":409,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":9952,"parent_record_offset":9376,"ancestor_record_offset":9376,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":9804,"parent_record_offset":9772,"ancestor_record_offset":9376,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":9672,"parent_record_offset":9644,"ancestor_record_offset":9376,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":9732,"parent_record_offset":9700,"ancestor_record_offset":9376,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":9852,"parent_record_offset":9828,"ancestor_record_offset":9376,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":9828,"parent_record_offset":9804,"ancestor_record_offset":9376,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":9700,"parent_record_offset":9672,"ancestor_record_offset":9376,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":9604,"parent_record_offset":9576,"ancestor_record_offset":9376,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":9772,"parent_record_offset":9604,"ancestor_record_offset":9376,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":9548,"parent_record_offset":9520,"ancestor_record_offset":9376,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":9892,"parent_record_offset":9604,"ancestor_record_offset":9376,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":9644,"parent_record_offset":9604,"ancestor_record_offset":9376,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":199,"record_offset":9576,"parent_record_offset":9548,"ancestor_record_offset":9376,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":9464,"parent_record_offset":9436,"ancestor_record_offset":9376,"inlinee":5420,"source_line":1849,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":9436,"parent_record_offset":9408,"ancestor_record_offset":9376,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":199,"record_offset":9520,"parent_record_offset":9492,"ancestor_record_offset":9376,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":199,"record_offset":9492,"parent_record_offset":9464,"ancestor_record_offset":9376,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=2
// 源码行: 36, 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1729,"record_offset":60276,"rva":24912864,"va":5393621984}
// 编译实例 RVAs: 0x17c23e0, 0x17c24b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18224711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393622192,"raw_hex":"e847161601","size":5,"soft_fail":false,"target_va":5411847152,"va":5393622436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1729,"record_offset":60476,"parent_record_offset":60276,"ancestor_record_offset":60276,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1729,"record_offset":60496,"parent_record_offset":60476,"ancestor_record_offset":60276,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1729,"record_offset":60516,"parent_record_offset":60496,"ancestor_record_offset":60276,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1729,"record_offset":60536,"parent_record_offset":60516,"ancestor_record_offset":60276,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$0::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":479,"record_offset":11672,"rva":8752256,"va":5377461376}
// 编译实例 RVAs: 0x858c80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5430975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377461376,"raw_hex":"e8bfde5200","size":5,"soft_fail":false,"target_va":5382892464,"va":5377461484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::builders::DebugStruct::field
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5416475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377461376,"raw_hex":"e81ba65200","size":5,"soft_fail":false,"target_va":5382878000,"va":5377461520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::builders::DebugStruct::finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5417075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377461376,"raw_hex":"e873a85200","size":5,"soft_fail":false,"target_va":5382878608,"va":5377461528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::TerrainEdit::duration
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6793
// 首条位置: {"module":524,"inlinee":26515}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":524,"record_offset":21464,"parent_record_offset":21200,"ancestor_record_offset":21200,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":524,"record_offset":21436,"parent_record_offset":21200,"ancestor_record_offset":21200,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":524,"record_offset":21324,"parent_record_offset":21200,"ancestor_record_offset":21200,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":524,"record_offset":21380,"parent_record_offset":21200,"ancestor_record_offset":21200,"inlinee":9237,"source_line":3521,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":524,"record_offset":21240,"parent_record_offset":21200,"ancestor_record_offset":21200,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":524,"record_offset":21348,"parent_record_offset":21324,"ancestor_record_offset":21200,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":524,"record_offset":21404,"parent_record_offset":21380,"ancestor_record_offset":21200,"inlinee":17977,"source_line":105,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":524,"record_offset":21288,"parent_record_offset":21264,"ancestor_record_offset":21200,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":524,"record_offset":21264,"parent_record_offset":21240,"ancestor_record_offset":21200,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::duration::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11FC5
// 首条位置: {"module":1605,"inlinee":73669}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1610,"record_offset":51628,"parent_record_offset":51548,"ancestor_record_offset":51548,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1610,"record_offset":51572,"parent_record_offset":51548,"ancestor_record_offset":51548,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1610,"record_offset":51700,"parent_record_offset":51672,"ancestor_record_offset":51548,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1610,"record_offset":51596,"parent_record_offset":51572,"ancestor_record_offset":51548,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::impl$1::duration::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":1610,"record_offset":51652,"parent_record_offset":51628,"ancestor_record_offset":51548,"inlinee":73671,"source_line":88,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1610,"record_offset":51780,"parent_record_offset":51752,"ancestor_record_offset":51548,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1610,"record_offset":51752,"parent_record_offset":51652,"ancestor_record_offset":51548,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1610,"record_offset":51672,"parent_record_offset":51652,"ancestor_record_offset":51548,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1610,"record_offset":51816,"parent_record_offset":51780,"ancestor_record_offset":51548,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::duration::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11FC7
// 首条位置: {"module":1605,"inlinee":73671}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1610,"record_offset":51752,"parent_record_offset":51652,"ancestor_record_offset":51652,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1610,"record_offset":51672,"parent_record_offset":51652,"ancestor_record_offset":51652,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1610,"record_offset":51700,"parent_record_offset":51672,"ancestor_record_offset":51652,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1610,"record_offset":51780,"parent_record_offset":51752,"ancestor_record_offset":51652,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1610,"record_offset":51816,"parent_record_offset":51780,"ancestor_record_offset":51652,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::TerrainEdit::revert
// 记录形式: compiled_procedure; instances=1
// 源码行: 102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":479,"record_offset":11808,"rva":8752432,"va":5377461552}
// 编译实例 RVAs: 0x858d30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2785589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377461552,"raw_hex":"e8cb7ed5ff","size":5,"soft_fail":false,"target_va":5374676688,"va":5377462272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2381350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377461552,"raw_hex":"e8daa9dbff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377462849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4472085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377461552,"raw_hex":"e8153d4400","size":5,"soft_fail":false,"target_va":5381934560,"va":5377462470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14130edd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11230899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377461552,"raw_hex":"e8b35eab00","size":5,"soft_fail":false,"target_va":5388692944,"va":5377462040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-3105634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377461552,"raw_hex":"e89e9cd0ff","size":5,"soft_fail":false,"target_va":5374356144,"va":5377461773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34079523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377461552,"raw_hex":"e823030802","size":5,"soft_fail":false,"target_va":5411542296,"va":5377462768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34079145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377461552,"raw_hex":"e8a9010802","size":5,"soft_fail":false,"target_va":5411541904,"va":5377462754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377461552,"raw_hex":"e8a9080000","size":5,"soft_fail":false,"target_va":5377464128,"va":5377461906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4444176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377461552,"raw_hex":"e810d04300","size":5,"soft_fail":false,"target_va":5381906160,"va":5377461979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":479,"record_offset":12760,"parent_record_offset":11808,"ancestor_record_offset":11808,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":479,"record_offset":12112,"parent_record_offset":11808,"ancestor_record_offset":11808,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":479,"record_offset":12000,"parent_record_offset":11808,"ancestor_record_offset":11808,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":479,"record_offset":12676,"parent_record_offset":11808,"ancestor_record_offset":11808,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$183::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":479,"record_offset":12440,"parent_record_offset":11808,"ancestor_record_offset":11808,"inlinee":24913,"source_line":35,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":479,"record_offset":12524,"parent_record_offset":11808,"ancestor_record_offset":11808,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":479,"record_offset":15236,"parent_record_offset":11808,"ancestor_record_offset":11808,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":479,"record_offset":15320,"parent_record_offset":11808,"ancestor_record_offset":11808,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":479,"record_offset":12592,"parent_record_offset":11808,"ancestor_record_offset":11808,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":479,"record_offset":12496,"parent_record_offset":11808,"ancestor_record_offset":11808,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_editing::TerrainEditingData::get_mut_stroke
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":479,"record_offset":13524,"parent_record_offset":11808,"ancestor_record_offset":11808,"inlinee":26446,"source_line":75,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":479,"record_offset":13452,"parent_record_offset":13428,"ancestor_record_offset":11808,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":479,"record_offset":13428,"parent_record_offset":13340,"ancestor_record_offset":11808,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":12308,"parent_record_offset":12276,"ancestor_record_offset":11808,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":13368,"parent_record_offset":13340,"ancestor_record_offset":11808,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":13340,"parent_record_offset":13316,"ancestor_record_offset":11808,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":12356,"parent_record_offset":12332,"ancestor_record_offset":11808,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":12332,"parent_record_offset":12308,"ancestor_record_offset":11808,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":13316,"parent_record_offset":13292,"ancestor_record_offset":11808,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":479,"record_offset":12276,"parent_record_offset":12236,"ancestor_record_offset":11808,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":479,"record_offset":12236,"parent_record_offset":12200,"ancestor_record_offset":11808,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":479,"record_offset":12700,"parent_record_offset":12676,"ancestor_record_offset":11808,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":479,"record_offset":12172,"parent_record_offset":12136,"ancestor_record_offset":11808,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":479,"record_offset":12136,"parent_record_offset":12112,"ancestor_record_offset":11808,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":479,"record_offset":12200,"parent_record_offset":12136,"ancestor_record_offset":11808,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":479,"record_offset":12024,"parent_record_offset":12000,"ancestor_record_offset":11808,"inlinee":8507,"source_line":1740,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":479,"record_offset":14332,"parent_record_offset":14308,"ancestor_record_offset":11808,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":479,"record_offset":13840,"parent_record_offset":13816,"ancestor_record_offset":11808,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":479,"record_offset":13924,"parent_record_offset":13900,"ancestor_record_offset":11808,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":479,"record_offset":15284,"parent_record_offset":15260,"ancestor_record_offset":11808,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":479,"record_offset":13864,"parent_record_offset":13840,"ancestor_record_offset":11808,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":479,"record_offset":14400,"parent_record_offset":13704,"ancestor_record_offset":11808,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":479,"record_offset":12724,"parent_record_offset":12700,"ancestor_record_offset":11808,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":479,"record_offset":13396,"parent_record_offset":13368,"ancestor_record_offset":11808,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":479,"record_offset":14072,"parent_record_offset":14044,"ancestor_record_offset":11808,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":479,"record_offset":14044,"parent_record_offset":14016,"ancestor_record_offset":11808,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":479,"record_offset":12464,"parent_record_offset":12440,"ancestor_record_offset":11808,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":479,"record_offset":13220,"parent_record_offset":13020,"ancestor_record_offset":11808,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":479,"record_offset":12640,"parent_record_offset":12616,"ancestor_record_offset":11808,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":479,"record_offset":14192,"parent_record_offset":14168,"ancestor_record_offset":11808,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":479,"record_offset":12396,"parent_record_offset":12236,"ancestor_record_offset":11808,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":479,"record_offset":12796,"parent_record_offset":12760,"ancestor_record_offset":11808,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":479,"record_offset":12832,"parent_record_offset":12796,"ancestor_record_offset":11808,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":479,"record_offset":12616,"parent_record_offset":12592,"ancestor_record_offset":11808,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::impl$1::revert::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":479,"record_offset":12560,"parent_record_offset":12524,"ancestor_record_offset":11808,"inlinee":26450,"source_line":125,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::impl$1::revert::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":479,"record_offset":15260,"parent_record_offset":15236,"ancestor_record_offset":11808,"inlinee":26451,"source_line":131,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":479,"record_offset":14356,"parent_record_offset":14332,"ancestor_record_offset":11808,"inlinee":26448,"source_line":38,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":479,"record_offset":13984,"parent_record_offset":13956,"ancestor_record_offset":11808,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":479,"record_offset":14016,"parent_record_offset":13984,"ancestor_record_offset":11808,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":479,"record_offset":13956,"parent_record_offset":13704,"ancestor_record_offset":11808,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":479,"record_offset":13816,"parent_record_offset":13704,"ancestor_record_offset":11808,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":479,"record_offset":14428,"parent_record_offset":13704,"ancestor_record_offset":11808,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":479,"record_offset":13900,"parent_record_offset":13704,"ancestor_record_offset":11808,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":479,"record_offset":13788,"parent_record_offset":13704,"ancestor_record_offset":11808,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":479,"record_offset":14308,"parent_record_offset":14284,"ancestor_record_offset":11808,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":479,"record_offset":13596,"parent_record_offset":13572,"ancestor_record_offset":11808,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":479,"record_offset":13572,"parent_record_offset":13548,"ancestor_record_offset":11808,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":479,"record_offset":14284,"parent_record_offset":14116,"ancestor_record_offset":11808,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":479,"record_offset":13628,"parent_record_offset":13596,"ancestor_record_offset":11808,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":479,"record_offset":14224,"parent_record_offset":14140,"ancestor_record_offset":11808,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":479,"record_offset":14168,"parent_record_offset":14140,"ancestor_record_offset":11808,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":479,"record_offset":14520,"parent_record_offset":13704,"ancestor_record_offset":11808,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":479,"record_offset":14140,"parent_record_offset":14116,"ancestor_record_offset":11808,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":479,"record_offset":13680,"parent_record_offset":13656,"ancestor_record_offset":11808,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":479,"record_offset":13656,"parent_record_offset":13596,"ancestor_record_offset":11808,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":479,"record_offset":13704,"parent_record_offset":13680,"ancestor_record_offset":11808,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":479,"record_offset":13760,"parent_record_offset":13704,"ancestor_record_offset":11808,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":479,"record_offset":14116,"parent_record_offset":13704,"ancestor_record_offset":11808,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":479,"record_offset":13548,"parent_record_offset":13524,"ancestor_record_offset":11808,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::revert::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6752
// 首条位置: {"module":479,"inlinee":26450}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::revert::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6753
// 首条位置: {"module":479,"inlinee":26451}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":479,"record_offset":15284,"parent_record_offset":15260,"ancestor_record_offset":15260,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::revert::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 169
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11AE8
// 首条位置: {"module":1585,"inlinee":72424}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1585,"record_offset":485648,"parent_record_offset":485532,"ancestor_record_offset":485532,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1585,"record_offset":485560,"parent_record_offset":485532,"ancestor_record_offset":485532,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1585,"record_offset":485588,"parent_record_offset":485560,"ancestor_record_offset":485532,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1585,"record_offset":485704,"parent_record_offset":485672,"ancestor_record_offset":485532,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1585,"record_offset":485672,"parent_record_offset":485648,"ancestor_record_offset":485532,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1585,"record_offset":485820,"parent_record_offset":485648,"ancestor_record_offset":485532,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::TerrainEdit::reapply
// 记录形式: compiled_procedure; instances=1
// 源码行: 184
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":479,"record_offset":15560,"rva":8753872,"va":5377462992}
// 编译实例 RVAs: 0x8592d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2382486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377462992,"raw_hex":"e86aa5dbff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377463985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4470872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377462992,"raw_hex":"e858384400","size":5,"soft_fail":false,"target_va":5381934560,"va":5377463683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141302730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11178634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377462992,"raw_hex":"e88a92aa00","size":5,"soft_fail":false,"target_va":5388642096,"va":5377463457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-3107077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377462992,"raw_hex":"e8fb96d0ff","size":5,"soft_fail":false,"target_va":5374356144,"va":5377463216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34078384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377462992,"raw_hex":"e8b0fe0702","size":5,"soft_fail":false,"target_va":5411542296,"va":5377463907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377462992,"raw_hex":"e820030000","size":5,"soft_fail":false,"target_va":5377464128,"va":5377463323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4442759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377462992,"raw_hex":"e887ca4300","size":5,"soft_fail":false,"target_va":5381906160,"va":5377463396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":479,"record_offset":16500,"parent_record_offset":15560,"ancestor_record_offset":15560,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":479,"record_offset":15852,"parent_record_offset":15560,"ancestor_record_offset":15560,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":479,"record_offset":15744,"parent_record_offset":15560,"ancestor_record_offset":15560,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":479,"record_offset":16416,"parent_record_offset":15560,"ancestor_record_offset":15560,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$183::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":479,"record_offset":16180,"parent_record_offset":15560,"ancestor_record_offset":15560,"inlinee":24913,"source_line":35,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":479,"record_offset":16264,"parent_record_offset":15560,"ancestor_record_offset":15560,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":479,"record_offset":17920,"parent_record_offset":15560,"ancestor_record_offset":15560,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":479,"record_offset":16332,"parent_record_offset":15560,"ancestor_record_offset":15560,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":479,"record_offset":16236,"parent_record_offset":15560,"ancestor_record_offset":15560,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":479,"record_offset":17192,"parent_record_offset":17168,"ancestor_record_offset":15560,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":479,"record_offset":17168,"parent_record_offset":17080,"ancestor_record_offset":15560,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":16048,"parent_record_offset":16016,"ancestor_record_offset":15560,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":17108,"parent_record_offset":17080,"ancestor_record_offset":15560,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":17080,"parent_record_offset":17056,"ancestor_record_offset":15560,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":16096,"parent_record_offset":16072,"ancestor_record_offset":15560,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":16072,"parent_record_offset":16048,"ancestor_record_offset":15560,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":17056,"parent_record_offset":17032,"ancestor_record_offset":15560,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":479,"record_offset":16016,"parent_record_offset":15976,"ancestor_record_offset":15560,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":479,"record_offset":15976,"parent_record_offset":15940,"ancestor_record_offset":15560,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":479,"record_offset":16440,"parent_record_offset":16416,"ancestor_record_offset":15560,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":479,"record_offset":15912,"parent_record_offset":15876,"ancestor_record_offset":15560,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":479,"record_offset":15876,"parent_record_offset":15852,"ancestor_record_offset":15560,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":479,"record_offset":15940,"parent_record_offset":15876,"ancestor_record_offset":15560,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":479,"record_offset":15768,"parent_record_offset":15744,"ancestor_record_offset":15560,"inlinee":8507,"source_line":1740,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":479,"record_offset":16464,"parent_record_offset":16440,"ancestor_record_offset":15560,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":479,"record_offset":17136,"parent_record_offset":17108,"ancestor_record_offset":15560,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":479,"record_offset":16204,"parent_record_offset":16180,"ancestor_record_offset":15560,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":479,"record_offset":16960,"parent_record_offset":16760,"ancestor_record_offset":15560,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":479,"record_offset":16380,"parent_record_offset":16356,"ancestor_record_offset":15560,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":479,"record_offset":16136,"parent_record_offset":15976,"ancestor_record_offset":15560,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":479,"record_offset":16536,"parent_record_offset":16500,"ancestor_record_offset":15560,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":479,"record_offset":16572,"parent_record_offset":16536,"ancestor_record_offset":15560,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":479,"record_offset":16356,"parent_record_offset":16332,"ancestor_record_offset":15560,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::impl$1::reapply::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":479,"record_offset":16300,"parent_record_offset":16264,"ancestor_record_offset":15560,"inlinee":26453,"source_line":206,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::reapply::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 206
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6755
// 首条位置: {"module":479,"inlinee":26453}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::reapply::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 241
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11ABB
// 首条位置: {"module":1585,"inlinee":72379}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1585,"record_offset":293048,"parent_record_offset":292932,"ancestor_record_offset":292932,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1585,"record_offset":292960,"parent_record_offset":292932,"ancestor_record_offset":292932,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1585,"record_offset":292988,"parent_record_offset":292960,"ancestor_record_offset":292932,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1585,"record_offset":293104,"parent_record_offset":293072,"ancestor_record_offset":292932,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1585,"record_offset":293072,"parent_record_offset":293048,"ancestor_record_offset":292932,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1585,"record_offset":293220,"parent_record_offset":293048,"ancestor_record_offset":292932,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve
// 记录形式: compiled_procedure; instances=1
// 源码行: 257
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":479,"record_offset":18160,"rva":8755008,"va":5377464128}
// 编译实例 RVAs: 0x859740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14014ba20
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7398510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377464128,"raw_hex":"e8921b8fff","size":5,"soft_fail":false,"target_va":5370067488,"va":5377465993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2787543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377464128,"raw_hex":"e82977d5ff","size":5,"soft_fail":false,"target_va":5374676688,"va":5377464226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2382991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377464128,"raw_hex":"e871a3dbff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377464474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2383721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377464128,"raw_hex":"e897a0dbff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377465220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f6880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11128535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377464128,"raw_hex":"e8d7cea900","size":5,"soft_fail":false,"target_va":5388593280,"va":5377464740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141466300
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12634063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377464128,"raw_hex":"e8cfc7c000","size":5,"soft_fail":false,"target_va":5390099200,"va":5377465132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141469230
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12646497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377464128,"raw_hex":"e861f8c000","size":5,"soft_fail":false,"target_va":5390111280,"va":5377464778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34382464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377464128,"raw_hex":"e880a20c02","size":5,"soft_fail":false,"target_va":5411847168,"va":5377464699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34075317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377464128,"raw_hex":"e8b5f20702","size":5,"soft_fail":false,"target_va":5411540963,"va":5377465641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t34076599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377464128,"raw_hex":"e8b7f70702","size":5,"soft_fail":false,"target_va":5411542064,"va":5377465460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34077325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377464128,"raw_hex":"e88dfa0702","size":5,"soft_fail":false,"target_va":5411541904,"va":5377464574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34077139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377464128,"raw_hex":"e8d3f90702","size":5,"soft_fail":false,"target_va":5411542880,"va":5377465736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4441505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377464128,"raw_hex":"e8a1c54300","size":5,"soft_fail":false,"target_va":5381906160,"va":5377464650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":479,"record_offset":19428,"parent_record_offset":18160,"ancestor_record_offset":18160,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":479,"record_offset":20136,"parent_record_offset":18160,"ancestor_record_offset":18160,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":479,"record_offset":22072,"parent_record_offset":18160,"ancestor_record_offset":18160,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":479,"record_offset":20052,"parent_record_offset":18160,"ancestor_record_offset":18160,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":479,"record_offset":22488,"parent_record_offset":18160,"ancestor_record_offset":18160,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_editing::TerrainEditingData::get_mut_stroke
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":479,"record_offset":18372,"parent_record_offset":18160,"ancestor_record_offset":18160,"inlinee":26446,"source_line":75,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":479,"record_offset":21748,"parent_record_offset":18160,"ancestor_record_offset":18160,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_slice
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":479,"record_offset":22104,"parent_record_offset":18160,"ancestor_record_offset":18160,"inlinee":24574,"source_line":851,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::try_new_from_points_remove_duplicates
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":479,"record_offset":20692,"parent_record_offset":18160,"ancestor_record_offset":18160,"inlinee":24695,"source_line":249,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":479,"record_offset":21860,"parent_record_offset":18160,"ancestor_record_offset":18160,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":479,"record_offset":21688,"parent_record_offset":18160,"ancestor_record_offset":18160,"inlinee":24561,"source_line":207,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":479,"record_offset":21592,"parent_record_offset":18160,"ancestor_record_offset":18160,"inlinee":24559,"source_line":191,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":479,"record_offset":19900,"parent_record_offset":19876,"ancestor_record_offset":18160,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":479,"record_offset":19924,"parent_record_offset":19900,"ancestor_record_offset":18160,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":479,"record_offset":20920,"parent_record_offset":20896,"ancestor_record_offset":18160,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":479,"record_offset":19876,"parent_record_offset":19700,"ancestor_record_offset":18160,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":479,"record_offset":20896,"parent_record_offset":20804,"ancestor_record_offset":18160,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":20248,"parent_record_offset":20224,"ancestor_record_offset":18160,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":19640,"parent_record_offset":19612,"ancestor_record_offset":18160,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":20832,"parent_record_offset":20804,"ancestor_record_offset":18160,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":20804,"parent_record_offset":20780,"ancestor_record_offset":18160,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":20296,"parent_record_offset":20272,"ancestor_record_offset":18160,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":20272,"parent_record_offset":20248,"ancestor_record_offset":18160,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":19700,"parent_record_offset":19668,"ancestor_record_offset":18160,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":19668,"parent_record_offset":19640,"ancestor_record_offset":18160,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":20780,"parent_record_offset":20756,"ancestor_record_offset":18160,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":19736,"parent_record_offset":19700,"ancestor_record_offset":18160,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":479,"record_offset":19572,"parent_record_offset":19536,"ancestor_record_offset":18160,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":479,"record_offset":19536,"parent_record_offset":19500,"ancestor_record_offset":18160,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":479,"record_offset":19464,"parent_record_offset":19428,"ancestor_record_offset":18160,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":479,"record_offset":19500,"parent_record_offset":19464,"ancestor_record_offset":18160,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":479,"record_offset":20224,"parent_record_offset":20200,"ancestor_record_offset":18160,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":479,"record_offset":20200,"parent_record_offset":20164,"ancestor_record_offset":18160,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":479,"record_offset":22148,"parent_record_offset":22104,"ancestor_record_offset":18160,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":479,"record_offset":19612,"parent_record_offset":19572,"ancestor_record_offset":18160,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":479,"record_offset":20164,"parent_record_offset":20136,"ancestor_record_offset":18160,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":479,"record_offset":19760,"parent_record_offset":19736,"ancestor_record_offset":18160,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":479,"record_offset":19784,"parent_record_offset":19760,"ancestor_record_offset":18160,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":479,"record_offset":21948,"parent_record_offset":21924,"ancestor_record_offset":18160,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":479,"record_offset":21972,"parent_record_offset":21948,"ancestor_record_offset":18160,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":479,"record_offset":19180,"parent_record_offset":19156,"ancestor_record_offset":18160,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":479,"record_offset":18688,"parent_record_offset":18664,"ancestor_record_offset":18160,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":479,"record_offset":18772,"parent_record_offset":18748,"ancestor_record_offset":18160,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":479,"record_offset":21712,"parent_record_offset":21688,"ancestor_record_offset":18160,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_infinite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":479,"record_offset":21656,"parent_record_offset":21592,"ancestor_record_offset":18160,"inlinee":24560,"source_line":547,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":479,"record_offset":21628,"parent_record_offset":21592,"ancestor_record_offset":18160,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":479,"record_offset":20100,"parent_record_offset":20076,"ancestor_record_offset":18160,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":479,"record_offset":18712,"parent_record_offset":18688,"ancestor_record_offset":18160,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":479,"record_offset":19248,"parent_record_offset":18556,"ancestor_record_offset":18160,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":479,"record_offset":19808,"parent_record_offset":19784,"ancestor_record_offset":18160,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":479,"record_offset":19832,"parent_record_offset":19808,"ancestor_record_offset":18160,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":479,"record_offset":20864,"parent_record_offset":20832,"ancestor_record_offset":18160,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":479,"record_offset":18920,"parent_record_offset":18892,"ancestor_record_offset":18160,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":479,"record_offset":18892,"parent_record_offset":18864,"ancestor_record_offset":18160,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":479,"record_offset":21828,"parent_record_offset":21748,"ancestor_record_offset":18160,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":479,"record_offset":21800,"parent_record_offset":21748,"ancestor_record_offset":18160,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":479,"record_offset":21772,"parent_record_offset":21748,"ancestor_record_offset":18160,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":479,"record_offset":21924,"parent_record_offset":21900,"ancestor_record_offset":18160,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":479,"record_offset":21900,"parent_record_offset":21860,"ancestor_record_offset":18160,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":479,"record_offset":19976,"parent_record_offset":19572,"ancestor_record_offset":18160,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":479,"record_offset":20732,"parent_record_offset":20692,"ancestor_record_offset":18160,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":479,"record_offset":19040,"parent_record_offset":19016,"ancestor_record_offset":18160,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":479,"record_offset":22204,"parent_record_offset":22176,"ancestor_record_offset":18160,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::impl$1::update_terrain_curve::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":479,"record_offset":20076,"parent_record_offset":20052,"ancestor_record_offset":18160,"inlinee":26455,"source_line":265,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":479,"record_offset":19204,"parent_record_offset":19180,"ancestor_record_offset":18160,"inlinee":26448,"source_line":38,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":479,"record_offset":18832,"parent_record_offset":18804,"ancestor_record_offset":18160,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":479,"record_offset":18864,"parent_record_offset":18832,"ancestor_record_offset":18160,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":479,"record_offset":18804,"parent_record_offset":18556,"ancestor_record_offset":18160,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":479,"record_offset":18664,"parent_record_offset":18556,"ancestor_record_offset":18160,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":479,"record_offset":19276,"parent_record_offset":18556,"ancestor_record_offset":18160,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":479,"record_offset":18748,"parent_record_offset":18556,"ancestor_record_offset":18160,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":479,"record_offset":18636,"parent_record_offset":18556,"ancestor_record_offset":18160,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":479,"record_offset":19156,"parent_record_offset":19132,"ancestor_record_offset":18160,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":479,"record_offset":18444,"parent_record_offset":18420,"ancestor_record_offset":18160,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":479,"record_offset":18420,"parent_record_offset":18396,"ancestor_record_offset":18160,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":479,"record_offset":19132,"parent_record_offset":18964,"ancestor_record_offset":18160,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":479,"record_offset":18476,"parent_record_offset":18444,"ancestor_record_offset":18160,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":479,"record_offset":19072,"parent_record_offset":18988,"ancestor_record_offset":18160,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":479,"record_offset":19016,"parent_record_offset":18988,"ancestor_record_offset":18160,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":479,"record_offset":19368,"parent_record_offset":18556,"ancestor_record_offset":18160,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":479,"record_offset":18988,"parent_record_offset":18964,"ancestor_record_offset":18160,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":479,"record_offset":18528,"parent_record_offset":18504,"ancestor_record_offset":18160,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":479,"record_offset":18504,"parent_record_offset":18444,"ancestor_record_offset":18160,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":479,"record_offset":18556,"parent_record_offset":18528,"ancestor_record_offset":18160,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":479,"record_offset":18608,"parent_record_offset":18556,"ancestor_record_offset":18160,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":479,"record_offset":18964,"parent_record_offset":18556,"ancestor_record_offset":18160,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":479,"record_offset":18396,"parent_record_offset":18372,"ancestor_record_offset":18160,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":479,"record_offset":22176,"parent_record_offset":22104,"ancestor_record_offset":18160,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::update_terrain_curve::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 265
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6757
// 首条位置: {"module":479,"inlinee":26455}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":479,"record_offset":20100,"parent_record_offset":20076,"ancestor_record_offset":20076,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00479.jsonl.gz"}
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
// 完整结构及原始类型记录: terrain_edit.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::terrain_edit::TerrainEdit
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 14
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::resources::history::terrain_edit::TerrainEdit>", "constructor": "enum2$", "arguments": ["country_core::resources::history::terrain_edit::TerrainEdit"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> >", "constructor": "alloc::sync::ArcInner", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::ArcInner<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::ArcInner<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> >", "constructor": "core::option::Option", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> > >", "constructor": "enum2$", "arguments": ["core::option::Option<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::Arc<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::Arc<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$216::deserialize<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,enum2$<serde_json::value::Value> >", "constructor": "serde::de::impls::impl$216::deserialize", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$54::fmt<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::impl$54::fmt", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<alloc::sync::Arc<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global> >", "constructor": "core::marker::PhantomData", "arguments": ["alloc::sync::Arc<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$5::newtype_variant_seed<core::marker::PhantomData<alloc::sync::Arc<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global> > >", "constructor": "serde_json::value::de::impl$5::newtype_variant_seed", "arguments": ["core::marker::PhantomData<alloc::sync::Arc<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [已有逆向证据 B / native_observed_partial_layout; not full original struct declaration] TerrainEdit native variants (partial)
// name: "TerrainEdit native variants (partial)"
// baseline: "B"
// status: "native_observed_partial_layout; not full original struct declaration"
// alignment: "unknown"
// complete_size: "unknown"
// unobserved_fields: "unknown"
// original_field_names: "unknown unless explicitly established"
// evidence: [{"path": "evidence/terrain-editing/B-history-schema.json", "sha256": "974291dee7e5f2ca82ee85a57a0bcd1a66bf36e898853fc5497d6245f9eb32d6"}]
// native_variant_schema: {"source_function": "0x1417c1590", "offset_evidence": "B-second/1417c1590.asm.txt", "discriminant_policy": "Native niche encoding, not a normal u32 tag", "variants": [{"name": "NewTerrainStroke", "first_qword": "0x8000000000000000", "fields": {"index": "0x20 u32", "stroke_pts": "0x08 Vec<Vec3> (cap=+8, ptr=+16, len=+24)", "height_ws": "0x24 f32"}}, {"name": "EditTerrainStroke", "first_qword": "0x8000000000000001", "fields": {"index": "0x08 u32", "prev_height_ws": "0x0c f32", "new_height_ws": "0x10 f32", "prev_falloff": "0x14 f32", "new_falloff": "0x18 f32"}}, {"name": "MoveTerrainStroke", "first_qword": "Vec allocation capacity; niche fallback branch", "fields": {"id": "0x20 u32", "original_stroke_pts": "0x00 Vec<Vec3> (cap=+0, ptr=+8, len=+16)", "move_vec": "0x18 Vec2"}}], "limitations": ["Native layouts specific to B", "index/id not generally interchangeable", "Vec/serde implementation reused; no parser reconstruction", "No snapshot format acceptance"]}
// enum_discriminant: "native niche; not Rust repr u32"
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: terrain_edit.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [资源/历史 JSON schema，不是内存布局] Terrain history serialized observations
// 全部实际键和数据形态: {"/commands/Terrain": {"observed_types": {"object": 102}, "object_instances": 102, "field_occurrences": {"NewTerrainStroke": 42, "EditTerrainStroke": 56, "MoveTerrainStroke": 4}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/15/edits/0/Terrain"}, {"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/16/edits/0/Terrain"}]}, "/commands/Terrain/EditTerrainStroke": {"observed_types": {"object": 56}, "object_instances": 56, "field_occurrences": {"index": 56, "new_falloff": 56, "new_height_ws": 56, "prev_falloff": 56, "prev_height_ws": 56}, "present_in_every_observed_object": ["index", "new_falloff", "new_height_ws", "prev_falloff", "prev_height_ws"], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/16/edits/0/Terrain/EditTerrainStroke"}, {"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/18/edits/0/Terrain/EditTerrainStroke"}]}, "/commands/Terrain/EditTerrainStroke/index": {"observed_types": {"integer": 56}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0, 12], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/16/edits/0/Terrain/EditTerrainStroke/index"}, {"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/18/edits/0/Terrain/EditTerrainStroke/index"}]}, "/commands/Terrain/EditTerrainStroke/new_falloff": {"observed_types": {"number": 56}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.0, 1.0], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/16/edits/0/Terrain/EditTerrainStroke/new_falloff"}, {"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/18/edits/0/Terrain/EditTerrainStroke/new_falloff"}]}, "/commands/Terrain/EditTerrainStroke/new_height_ws": {"observed_types": {"number": 56}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [2.6363797187805176, 20.0], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/16/edits/0/Terrain/EditTerrainStroke/new_height_ws"}, {"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/18/edits/0/Terrain/EditTerrainStroke/new_height_ws"}]}, "/commands/Terrain/EditTerrainStroke/prev_falloff": {"observed_types": {"number": 56}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.0, 1.0], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/16/edits/0/Terrain/EditTerrainStroke/prev_falloff"}, {"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/18/edits/0/Terrain/EditTerrainStroke/prev_falloff"}]}, "/commands/Terrain/EditTerrainStroke/prev_height_ws": {"observed_types": {"number": 56}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [1.6912766, 20.0], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/16/edits/0/Terrain/EditTerrainStroke/prev_height_ws"}, {"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/18/edits/0/Terrain/EditTerrainStroke/prev_height_ws"}]}, "/commands/Terrain/MoveTerrainStroke": {"observed_types": {"object": 4}, "object_instances": 4, "field_occurrences": {"id": 4, "original_stroke_pts": 4, "move_vec": 4}, "present_in_every_observed_object": ["id", "original_stroke_pts", "move_vec"], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/28-wizard-tower/history.json", "pointer": "/History/edit_chain/3/edits/0/Terrain/MoveTerrainStroke"}, {"file": "assets/starting-builds/full/28-wizard-tower/history.json", "pointer": "/History/edit_chain/5/edits/0/Terrain/MoveTerrainStroke"}]}, "/commands/Terrain/MoveTerrainStroke/id": {"observed_types": {"integer": 4}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [1, 2], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/28-wizard-tower/history.json", "pointer": "/History/edit_chain/3/edits/0/Terrain/MoveTerrainStroke/id"}, {"file": "assets/starting-builds/full/28-wizard-tower/history.json", "pointer": "/History/edit_chain/5/edits/0/Terrain/MoveTerrainStroke/id"}]}, "/commands/Terrain/MoveTerrainStroke/move_vec": {"observed_types": {"array": 4}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {"2": 4}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/28-wizard-tower/history.json", "pointer": "/History/edit_chain/3/edits/0/Terrain/MoveTerrainStroke/move_vec"}, {"file": "assets/starting-builds/full/28-wizard-tower/history.json", "pointer": "/History/edit_chain/5/edits/0/Terrain/MoveTerrainStroke/move_vec"}]}, "/commands/Terrain/MoveTerrainStroke/move_vec/[]": {"observed_types": {"number": 8}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [-2.4561691, 1.2595444], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/28-wizard-tower/history.json", "pointer": "/History/edit_chain/3/edits/0/Terrain/MoveTerrainStroke/move_vec/0"}, {"file": "assets/starting-builds/full/28-wizard-tower/history.json", "pointer": "/History/edit_chain/3/edits/0/Terrain/MoveTerrainStroke/move_vec/1"}]}, "/commands/Terrain/MoveTerrainStroke/original_stroke_pts": {"observed_types": {"array": 4}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {"1": 4}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/28-wizard-tower/history.json", "pointer": "/History/edit_chain/3/edits/0/Terrain/MoveTerrainStroke/original_stroke_pts"}, {"file": "assets/starting-builds/full/28-wizard-tower/history.json", "pointer": "/History/edit_chain/5/edits/0/Terrain/MoveTerrainStroke/original_stroke_pts"}]}, "/commands/Terrain/MoveTerrainStroke/original_stroke_pts/[]": {"observed_types": {"array": 4}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {"3": 4}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/28-wizard-tower/history.json", "pointer": "/History/edit_chain/3/edits/0/Terrain/MoveTerrainStroke/original_stroke_pts/0"}, {"file": "assets/starting-builds/full/28-wizard-tower/history.json", "pointer": "/History/edit_chain/5/edits/0/Terrain/MoveTerrainStroke/original_stroke_pts/0"}]}, "/commands/Terrain/MoveTerrainStroke/original_stroke_pts/[]/[]": {"observed_types": {"number": 12}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [-5.1859055, 14.085384], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/28-wizard-tower/history.json", "pointer": "/History/edit_chain/3/edits/0/Terrain/MoveTerrainStroke/original_stroke_pts/0/0"}, {"file": "assets/starting-builds/full/28-wizard-tower/history.json", "pointer": "/History/edit_chain/3/edits/0/Terrain/MoveTerrainStroke/original_stroke_pts/0/1"}]}, "/commands/Terrain/NewTerrainStroke": {"observed_types": {"object": 42}, "object_instances": 42, "field_occurrences": {"height_ws": 42, "index": 42, "stroke_pts": 42}, "present_in_every_observed_object": ["height_ws", "index", "stroke_pts"], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/15/edits/0/Terrain/NewTerrainStroke"}, {"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/17/edits/0/Terrain/NewTerrainStroke"}]}, "/commands/Terrain/NewTerrainStroke/height_ws": {"observed_types": {"number": 42}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [1.6912766, 14.085384], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/15/edits/0/Terrain/NewTerrainStroke/height_ws"}, {"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/17/edits/0/Terrain/NewTerrainStroke/height_ws"}]}, "/commands/Terrain/NewTerrainStroke/index": {"observed_types": {"integer": 42}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0, 12], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/15/edits/0/Terrain/NewTerrainStroke/index"}, {"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/17/edits/0/Terrain/NewTerrainStroke/index"}]}, "/commands/Terrain/NewTerrainStroke/stroke_pts": {"observed_types": {"array": 42}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {"1": 4, "2": 1, "3": 2, "4": 1, "5": 1, "6": 1, "8": 1, "9": 4, "11": 1, "12": 1, "13": 1, "16": 1, "18": 3, "20": 2, "21": 1, "22": 3, "23": 1, "24": 1, "25": 1, "27": 1, "29": 1, "34": 1, "35": 1, "38": 1, "42": 1, "45": 1, "55": 1, "60": 1, "85": 2}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/15/edits/0/Terrain/NewTerrainStroke/stroke_pts"}, {"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/17/edits/0/Terrain/NewTerrainStroke/stroke_pts"}]}, "/commands/Terrain/NewTerrainStroke/stroke_pts/[]": {"observed_types": {"array": 911}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {"3": 911}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/15/edits/0/Terrain/NewTerrainStroke/stroke_pts/0"}, {"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/15/edits/0/Terrain/NewTerrainStroke/stroke_pts/1"}]}, "/commands/Terrain/NewTerrainStroke/stroke_pts/[]/[]": {"observed_types": {"number": 2733}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [-24.717159271240234, 21.50347137451172], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/15/edits/0/Terrain/NewTerrainStroke/stroke_pts/0/0"}, {"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/15/edits/0/Terrain/NewTerrainStroke/stroke_pts/0/1"}]}}
// offsets / Rust numeric types / missing defaults = unknown。
// [已有模块证据关联；不自动成为字段事实] evidence/pdb-structure/type-layout-coverage.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::history::terrain_edit::TerrainEdit", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/terrain-editing/B-core/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::history::terrain_edit::TerrainEdit", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/terrain-editing/B-second-helper/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::history::terrain_edit::TerrainEdit", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/terrain-editing/source-ownership.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/history/terrain_edit.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: terrain_edit.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 9; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::TerrainEdit::duration
// 信息 ID: a7696bad06d00ec1817e7c42
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_terrain::TerrainReplayCmd::forward; module=524, inline_record=21200; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_terrain::TerrainReplayCmd::backward; module=524, inline_record=21848; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::TerrainEdit::reapply
// 信息 ID: 1fe585b73f6d8ba08967cd05
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_terrain::replay_terrain @ 0x99539a -> 0x8592d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::TerrainEdit::revert
// 信息 ID: ea0e19912f1d91a2fa3c1b6f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_terrain::replay_terrain @ 0x99536a -> 0x858d30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve
// 信息 ID: c32de69f0ef0937203c50815
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::history::terrain_edit::TerrainEdit::revert @ 0x858e92 -> 0x859740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::terrain_edit::TerrainEdit::reapply @ 0x85941b -> 0x859740 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize
// 信息 ID: f01fecb73a392e303b134be9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1729, inline_record=56748; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1729, inline_record=56876; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1729, inline_record=57004; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 3b5e4ecf400fc891c55d7621
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=258360; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Field> >; module=1630, inline_record=504304; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: 5ebbfd4178402fc75aa9dd37
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::visit_object<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor> @ 0x15c2433 -> 0x17c23e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::visit_object<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor> @ 0x15cb8a5 -> 0x17c24b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 7955f02ae0126c747e311029
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=258188; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=328636; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=408676; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Field> >; module=1630, inline_record=504176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 0161e547202819507ac8170d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1729, inline_record=672; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1729, inline_record=1104; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1729, inline_record=1984; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: cf980868254bbbf189a41390
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1729, inline_record=56516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: b505dfccb5329f83b19c29a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=256640; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=327100; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=407136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: cb304a8da444c36ee85504cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=74816; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=98256; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=175972; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 信息 ID: 2e20532185d24575a15ba7c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::resources::history::terrain_edit::_::impl$0::deserialize -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impls::impl$216::deserialize<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,enum2$<serde_json::value::Value> > @ 0xb5e042 -> 0x17c1ac0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::serialize
// 信息 ID: 594449639a4d249d924c93d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=9376; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=11136; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=12928; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1729, inline_record=51416; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1729, inline_record=53176; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1729, inline_record=54968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: c4be3252749ef3ce7cba636a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::history::terrain_edit::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::history::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x3166ef -> 0x4f41b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0xb39d5d -> 0x17c1590 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$0::fmt
// 信息 ID: 42191ab95bd88dcde4c944c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::sync::impl$54::fmt<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global> @ 0xb5e377 -> 0x858c80 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::duration::closure$0
// 信息 ID: 4b1ef012d726f5d5a1543672
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::duration", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0 <- core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0,f32,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0 <- core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0,f32,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0,f32,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0> > >; module=1610, inline_record=51548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::duration::closure$0::closure$0
// 信息 ID: e415fafe920fc498379e5cc4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::duration", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0,f32,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0> > >; module=1610, inline_record=51652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::reapply::closure$0
// 信息 ID: 19b9a24506a1bc6faca8d46c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::reapply", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::TerrainEdit::reapply; module=479, inline_record=16300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::reapply::closure$1
// 信息 ID: bfbbcb6177ff2388452e624f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::reapply", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::history::terrain_edit::impl$1::reapply::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::history::terrain_edit::impl$1::reapply::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::history::terrain_edit::impl$1::reapply::closure_env$1> >; module=1585, inline_record=292932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::revert::closure$0
// 信息 ID: 89a363814ec851ac42ecaa0a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::revert", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::TerrainEdit::revert; module=479, inline_record=12560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::revert::closure$1
// 信息 ID: fdb4336252ed8caf253ad731
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::revert", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::TerrainEdit::revert; module=479, inline_record=15260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::revert::closure$2
// 信息 ID: 932957046cd262c4b27fb30f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::revert", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::history::terrain_edit::impl$1::revert::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::history::terrain_edit::impl$1::revert::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::history::terrain_edit::impl$1::revert::closure_env$2> >; module=1585, inline_record=485532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::update_terrain_curve::closure$0
// 信息 ID: d8bb16d3ef936422cb119087
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::update_terrain_curve", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve; module=479, inline_record=20076; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
