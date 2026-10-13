// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/history/decorator_edit.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 4981f1c7178786d8a3ec51258c940413c9a22ec922ba63e883b757b03af85b7a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: decorator_edit.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 191, 199, 470, 475, 540, 555, 572, 584, 586, 587, 593, 599, 1503, 1574, 1585, 1608, 1610, 1613, 1618, 1620, 1627, 1630, 1655, 1729, 3666, 4443
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// use vectorize; // namespace candidate
//   package identities: [{"name":"vectorize","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=2
// 源码行: 17, 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1729,"record_offset":40776,"rva":24905872,"va":5393614992}
// 编译实例 RVAs: 0x17c0890, 0x17c1170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f9e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18060513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393614992,"raw_hex":"e8e1941301","size":5,"soft_fail":false,"target_va":5411675680,"va":5393615162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18229804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393617264,"raw_hex":"e82c2a1601","size":5,"soft_fail":false,"target_va":5411847152,"va":5393617343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1729,"record_offset":40980,"parent_record_offset":40776,"ancestor_record_offset":40776,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1729,"record_offset":41000,"parent_record_offset":40980,"ancestor_record_offset":40776,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1729,"record_offset":41020,"parent_record_offset":41000,"ancestor_record_offset":40776,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1729,"record_offset":41040,"parent_record_offset":41020,"ancestor_record_offset":40776,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 17, 31, 34, 70, 449, 477
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12224
// 首条位置: {"module":1630,"inlinee":74276}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::value::impl$26::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":471968,"parent_record_offset":471944,"ancestor_record_offset":471944,"inlinee":74331,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":209056,"parent_record_offset":209032,"ancestor_record_offset":209032,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":209476,"parent_record_offset":209452,"ancestor_record_offset":209032,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":209452,"parent_record_offset":209396,"ancestor_record_offset":209032,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":209424,"parent_record_offset":209396,"ancestor_record_offset":209032,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":209396,"parent_record_offset":209372,"ancestor_record_offset":209032,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":209372,"parent_record_offset":209348,"ancestor_record_offset":209032,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":209204,"parent_record_offset":209180,"ancestor_record_offset":209032,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":209300,"parent_record_offset":209128,"ancestor_record_offset":209032,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":209228,"parent_record_offset":209204,"ancestor_record_offset":209032,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":209252,"parent_record_offset":209228,"ancestor_record_offset":209032,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":209180,"parent_record_offset":209156,"ancestor_record_offset":209032,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1630,"record_offset":209156,"parent_record_offset":209128,"ancestor_record_offset":209032,"inlinee":74277,"source_line":477,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":209128,"parent_record_offset":209104,"ancestor_record_offset":209032,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$26::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1630,"record_offset":471996,"parent_record_offset":471968,"ancestor_record_offset":471944,"inlinee":74332,"source_line":615,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":209080,"parent_record_offset":209056,"ancestor_record_offset":209032,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":209104,"parent_record_offset":209080,"ancestor_record_offset":209032,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13014
// 首条位置: {"module":1729,"inlinee":77844}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1729,"record_offset":37384,"parent_record_offset":36860,"ancestor_record_offset":36860,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1729,"record_offset":36936,"parent_record_offset":36860,"ancestor_record_offset":36860,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1729,"record_offset":36972,"parent_record_offset":36860,"ancestor_record_offset":36860,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1729,"record_offset":36908,"parent_record_offset":36860,"ancestor_record_offset":36860,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] serde::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1729,"record_offset":37000,"parent_record_offset":36860,"ancestor_record_offset":36860,"inlinee":5585,"source_line":2117,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1729,"record_offset":37088,"parent_record_offset":37064,"ancestor_record_offset":36860,"inlinee":33596,"source_line":28,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1729,"record_offset":37064,"parent_record_offset":37028,"ancestor_record_offset":36860,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$5::newtype_variant_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1729,"record_offset":37028,"parent_record_offset":37000,"ancestor_record_offset":36860,"inlinee":31380,"source_line":502,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1729,"record_offset":35992,"rva":24903552,"va":5393612672}
// 编译实例 RVAs: 0x17bff80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t17864867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393612672,"raw_hex":"e8a3981001","size":5,"soft_fail":false,"target_va":5411478016,"va":5393613144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18532236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393612672,"raw_hex":"e87438e5fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5393613735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14085da90
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16133363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393612672,"raw_hex":"e80dd309ff","size":5,"soft_fail":false,"target_va":5377481360,"va":5393614718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140887df0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15958713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393612672,"raw_hex":"e8477d0cff","size":5,"soft_fail":false,"target_va":5377654256,"va":5393612964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888d70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15954984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393612672,"raw_hex":"e8d88b0cff","size":5,"soft_fail":false,"target_va":5377658224,"va":5393613203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140889390
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15953985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393612672,"raw_hex":"e8bf8f0cff","size":5,"soft_fail":false,"target_va":5377659792,"va":5393613772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140898f40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15889335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393612672,"raw_hex":"e8498c0dff","size":5,"soft_fail":false,"target_va":5377724224,"va":5393613554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14143c450
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3685325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393612672,"raw_hex":"e833c4c7ff","size":5,"soft_fail":false,"target_va":5389927504,"va":5393612824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14159df30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2237882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393612672,"raw_hex":"e846daddff","size":5,"soft_fail":false,"target_va":5391376176,"va":5393614053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14159ef30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2233862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393612672,"raw_hex":"e8fae9ddff","size":5,"soft_fail":false,"target_va":5391380272,"va":5393614129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14159f530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2232038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393612672,"raw_hex":"e81af1ddff","size":5,"soft_fail":false,"target_va":5391381808,"va":5393613841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415a1330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2224454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393612672,"raw_hex":"e8ba0edeff","size":5,"soft_fail":false,"target_va":5391389488,"va":5393613937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415d18a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2025562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393612672,"raw_hex":"e8a617e1ff","size":5,"soft_fail":false,"target_va":5391587488,"va":5393613045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f9b40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18062002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393612672,"raw_hex":"e8b29a1301","size":5,"soft_fail":false,"target_va":5411674944,"va":5393612937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::de::_$LT$impl$u20$serde_json..value..Value$GT$::unexpected
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t17865418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393612672,"raw_hex":"e8ca9a1001","size":5,"soft_fail":false,"target_va":5411478544,"va":5393613121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::value::de::impl$2::deserialize_enum
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1729,"record_offset":36184,"parent_record_offset":35992,"ancestor_record_offset":35992,"inlinee":26604,"source_line":255,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1729,"record_offset":38004,"parent_record_offset":37980,"ancestor_record_offset":35992,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1729,"record_offset":37980,"parent_record_offset":37892,"ancestor_record_offset":35992,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1729,"record_offset":36384,"parent_record_offset":36360,"ancestor_record_offset":35992,"inlinee":6033,"source_line":1680,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1729,"record_offset":36440,"parent_record_offset":36416,"ancestor_record_offset":35992,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1729,"record_offset":36492,"parent_record_offset":36464,"ancestor_record_offset":35992,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1729,"record_offset":36516,"parent_record_offset":36492,"ancestor_record_offset":35992,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1729,"record_offset":37752,"parent_record_offset":37728,"ancestor_record_offset":35992,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1729,"record_offset":37920,"parent_record_offset":37892,"ancestor_record_offset":35992,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1729,"record_offset":37892,"parent_record_offset":37868,"ancestor_record_offset":35992,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1729,"record_offset":37800,"parent_record_offset":37776,"ancestor_record_offset":35992,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1729,"record_offset":37776,"parent_record_offset":37752,"ancestor_record_offset":35992,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1729,"record_offset":37868,"parent_record_offset":37844,"ancestor_record_offset":35992,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1729,"record_offset":37728,"parent_record_offset":37704,"ancestor_record_offset":35992,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1729,"record_offset":37704,"parent_record_offset":37676,"ancestor_record_offset":35992,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1729,"record_offset":37948,"parent_record_offset":37920,"ancestor_record_offset":35992,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1729,"record_offset":36464,"parent_record_offset":36440,"ancestor_record_offset":35992,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1729,"record_offset":36632,"parent_record_offset":36544,"ancestor_record_offset":35992,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1729,"record_offset":37644,"parent_record_offset":36184,"ancestor_record_offset":35992,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1729,"record_offset":36656,"parent_record_offset":36632,"ancestor_record_offset":35992,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1729,"record_offset":37384,"parent_record_offset":36860,"ancestor_record_offset":35992,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1729,"record_offset":36936,"parent_record_offset":36860,"ancestor_record_offset":35992,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1729,"record_offset":36972,"parent_record_offset":36860,"ancestor_record_offset":35992,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1729,"record_offset":36544,"parent_record_offset":36516,"ancestor_record_offset":35992,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1729,"record_offset":36600,"parent_record_offset":36576,"ancestor_record_offset":35992,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1729,"record_offset":36576,"parent_record_offset":36544,"ancestor_record_offset":35992,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1729,"record_offset":37088,"parent_record_offset":37064,"ancestor_record_offset":35992,"inlinee":33596,"source_line":28,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1729,"record_offset":36860,"parent_record_offset":36184,"ancestor_record_offset":35992,"inlinee":77844,"source_line":17,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1729,"record_offset":36908,"parent_record_offset":36860,"ancestor_record_offset":35992,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1729,"record_offset":37000,"parent_record_offset":36860,"ancestor_record_offset":35992,"inlinee":5585,"source_line":2117,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1729,"record_offset":37064,"parent_record_offset":37028,"ancestor_record_offset":35992,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$17::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1729,"record_offset":36360,"parent_record_offset":36184,"ancestor_record_offset":35992,"inlinee":26606,"source_line":877,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1729,"record_offset":36416,"parent_record_offset":36184,"ancestor_record_offset":35992,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$5::newtype_variant_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1729,"record_offset":37028,"parent_record_offset":37000,"ancestor_record_offset":35992,"inlinee":31380,"source_line":502,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=8
// 源码行: 17, 70, 449, 477
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":199,"record_offset":13596,"rva":5195488,"va":5373904608}
// 编译实例 RVAs: 0x4f46e0, 0x4f47c0, 0x4f4c30, 0x4f5000, 0x17bfb10, 0x17c0960, 0x17c13b0, 0x17c1490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b5870
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2355504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373904832,"raw_hex":"e8d00edcff","size":5,"soft_fail":false,"target_va":5371549808,"va":5373905307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b6100
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-255827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373904832,"raw_hex":"e8ad18fcff","size":5,"soft_fail":false,"target_va":5373649152,"va":5373904974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b7780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-251796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373905968,"raw_hex":"e86c28fcff","size":5,"soft_fail":false,"target_va":5373654912,"va":5373906703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b8720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-247456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373905968,"raw_hex":"e86039fcff","size":5,"soft_fail":false,"target_va":5373658912,"va":5373906363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404ba220
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-240501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373905968,"raw_hex":"e88b54fcff","size":5,"soft_fail":false,"target_va":5373665824,"va":5373906320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404baab0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-238277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373905968,"raw_hex":"e83b5dfcff","size":5,"soft_fail":false,"target_va":5373668016,"va":5373906288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404baf70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-235956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373904832,"raw_hex":"e84c66fcff","size":5,"soft_fail":false,"target_va":5373669232,"va":5373905183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bc390
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-232049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373905968,"raw_hex":"e88f75fcff","size":5,"soft_fail":false,"target_va":5373674384,"va":5373906428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bd3f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-227659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373905968,"raw_hex":"e8b586fcff","size":5,"soft_fail":false,"target_va":5373678576,"va":5373906230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bdc80
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-224456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373904832,"raw_hex":"e83893fcff","size":5,"soft_fail":false,"target_va":5373680768,"va":5373905219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bde40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-223936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373904832,"raw_hex":"e84095fcff","size":5,"soft_fail":false,"target_va":5373681216,"va":5373905147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404be820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-220942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373904608,"raw_hex":"e8f2a0fcff","size":5,"soft_fail":false,"target_va":5373683744,"va":5373904681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404be900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-222783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373905968,"raw_hex":"e8c199fcff","size":5,"soft_fail":false,"target_va":5373683968,"va":5373906746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404c0220
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-215771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373905968,"raw_hex":"e825b5fcff","size":5,"soft_fail":false,"target_va":5373690400,"va":5373906166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404c10c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-212059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373905968,"raw_hex":"e8a5c3fcff","size":5,"soft_fail":false,"target_va":5373694144,"va":5373906198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404c1af0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-210269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373906944,"raw_hex":"e8a3cafcff","size":5,"soft_fail":false,"target_va":5373696752,"va":5373907016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833230
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3401174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373905968,"raw_hex":"e8d6e53300","size":5,"soft_fail":false,"target_va":5377307184,"va":5373906005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408c5680
//     owner=unresolved_namespace; count=4
//     target_aliases=10 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4000277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373905968,"raw_hex":"e8150a3d00","size":5,"soft_fail":false,"target_va":5377906304,"va":5373906022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b3f910
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6597778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373905968,"raw_hex":"e892ac6400","size":5,"soft_fail":false,"target_va":5380503824,"va":5373906041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f4520
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5027792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393611536,"raw_hex":"e83048b3ff","size":5,"soft_fail":false,"target_va":5388584224,"va":5393612011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141580d20
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2354819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393611536,"raw_hex":"e87d11dcff","size":5,"soft_fail":false,"target_va":5391256864,"va":5393611678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141582ea0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2349880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393615200,"raw_hex":"e8c824dcff","size":5,"soft_fail":false,"target_va":5391265440,"va":5393615315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141583160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2349650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393615200,"raw_hex":"e8ae25dcff","size":5,"soft_fail":false,"target_va":5391266144,"va":5393615789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141583a40
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2347027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393615200,"raw_hex":"e8ed2fdcff","size":5,"soft_fail":false,"target_va":5391268416,"va":5393615438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141586090
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2333704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393611536,"raw_hex":"e8f863dcff","size":5,"soft_fail":false,"target_va":5391278224,"va":5393611923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141586a10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2331200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393611536,"raw_hex":"e8c06ddcff","size":5,"soft_fail":false,"target_va":5391280656,"va":5393611851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141586af0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2331012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393611536,"raw_hex":"e87c6edcff","size":5,"soft_fail":false,"target_va":5391280880,"va":5393611887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141586d70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2336621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393618064,"raw_hex":"e89358dcff","size":5,"soft_fail":false,"target_va":5391281520,"va":5393618136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141588070
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2331534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393617840,"raw_hex":"e8726cdcff","size":5,"soft_fail":false,"target_va":5391286384,"va":5393617913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141588150
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2328965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393615200,"raw_hex":"e87b76dcff","size":5,"soft_fail":false,"target_va":5391286608,"va":5393615568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141588c60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2326035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393615200,"raw_hex":"e8ed81dcff","size":5,"soft_fail":false,"target_va":5391289440,"va":5393615470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14158a500
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2319608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393615200,"raw_hex":"e8089bdcff","size":5,"soft_fail":false,"target_va":5391295744,"va":5393615347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14158b8c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2314584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393615200,"raw_hex":"e8a8aedcff","size":5,"soft_fail":false,"target_va":5391300800,"va":5393615379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14158b9a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2314489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393615200,"raw_hex":"e807afdcff","size":5,"soft_fail":false,"target_va":5391301024,"va":5393615508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14158cf00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2309339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393615200,"raw_hex":"e825c3dcff","size":5,"soft_fail":false,"target_va":5391306496,"va":5393615830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=30
//     evidence={"assembly":"\tcallq\t37413464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373904608,"raw_hex":"e858e23a02","size":5,"soft_fail":false,"target_va":5411318240,"va":5373904771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] serde_json::error::Error::io
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t37572112","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5373904832,"raw_hex":"e9104e3d02","size":5,"soft_fail":false,"target_va":5411477664,"va":5373905547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::history::decorator_edit::is_default
//     owner=private_source_ownership_candidate; count=14
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":199,"record_offset":19224,"parent_record_offset":18928,"ancestor_record_offset":18928,"inlinee":14664,"source_line":136,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::decorator_edit::is_default_lantern_brightness
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1729,"record_offset":43168,"parent_record_offset":41420,"ancestor_record_offset":41420,"inlinee":77845,"source_line":152,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::decorator_edit::is_default_stair_width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1729,"record_offset":43224,"parent_record_offset":41420,"ancestor_record_offset":41420,"inlinee":33469,"source_line":168,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::ClutterVersion::is_initial
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1729,"record_offset":43056,"parent_record_offset":41420,"ancestor_record_offset":41420,"inlinee":26024,"source_line":220,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_newtype_variant
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":15252,"parent_record_offset":14968,"ancestor_record_offset":14968,"inlinee":5418,"source_line":268,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":13888,"parent_record_offset":13596,"ancestor_record_offset":13596,"inlinee":5727,"source_line":409,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":14420,"parent_record_offset":13596,"ancestor_record_offset":13596,"inlinee":5728,"source_line":774,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=38
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":14392,"parent_record_offset":13596,"ancestor_record_offset":13596,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":14244,"parent_record_offset":14212,"ancestor_record_offset":13596,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":14124,"parent_record_offset":14100,"ancestor_record_offset":13596,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":14176,"parent_record_offset":14148,"ancestor_record_offset":13596,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":14292,"parent_record_offset":14268,"ancestor_record_offset":13596,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":14268,"parent_record_offset":14244,"ancestor_record_offset":13596,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":14148,"parent_record_offset":14124,"ancestor_record_offset":13596,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":14064,"parent_record_offset":14040,"ancestor_record_offset":13596,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":14212,"parent_record_offset":14064,"ancestor_record_offset":13596,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":14016,"parent_record_offset":13992,"ancestor_record_offset":13596,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":14332,"parent_record_offset":14064,"ancestor_record_offset":13596,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":14100,"parent_record_offset":14064,"ancestor_record_offset":13596,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":199,"record_offset":14040,"parent_record_offset":14016,"ancestor_record_offset":13596,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":199,"record_offset":19372,"parent_record_offset":19344,"ancestor_record_offset":18928,"inlinee":8507,"source_line":1740,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":199,"record_offset":19252,"parent_record_offset":19224,"ancestor_record_offset":18928,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":199,"record_offset":16912,"parent_record_offset":16888,"ancestor_record_offset":14968,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::impl$14::eq
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":199,"record_offset":19280,"parent_record_offset":19252,"ancestor_record_offset":18928,"inlinee":14666,"source_line":26,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::_::impl$0::serialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":199,"record_offset":17424,"parent_record_offset":16348,"ancestor_record_offset":14968,"inlinee":14662,"source_line":34,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$77::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":199,"record_offset":19740,"parent_record_offset":19712,"ancestor_record_offset":18928,"inlinee":14668,"source_line":323,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$84::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":199,"record_offset":19832,"parent_record_offset":19804,"ancestor_record_offset":18928,"inlinee":14670,"source_line":332,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":13940,"parent_record_offset":13916,"ancestor_record_offset":13596,"inlinee":5420,"source_line":1849,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":16944,"parent_record_offset":16348,"ancestor_record_offset":14968,"inlinee":5429,"source_line":1894,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":14484,"parent_record_offset":14452,"ancestor_record_offset":13596,"inlinee":5435,"source_line":1859,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":13916,"parent_record_offset":13888,"ancestor_record_offset":13596,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":15768,"parent_record_offset":15252,"ancestor_record_offset":14968,"inlinee":5182,"source_line":219,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":17452,"parent_record_offset":17424,"ancestor_record_offset":14968,"inlinee":5727,"source_line":409,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":14452,"parent_record_offset":14420,"ancestor_record_offset":13596,"inlinee":5730,"source_line":720,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::end
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":18108,"parent_record_offset":17424,"ancestor_record_offset":14968,"inlinee":5728,"source_line":774,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":17996,"parent_record_offset":17424,"ancestor_record_offset":14968,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":199,"record_offset":13992,"parent_record_offset":13968,"ancestor_record_offset":13596,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":199,"record_offset":13968,"parent_record_offset":13940,"ancestor_record_offset":13596,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$8::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8345
// 首条位置: {"module":593,"inlinee":33605}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$9::clone
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x64D9
// 首条位置: {"module":470,"inlinee":25817}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=20
// 源码行: 25, 28, 31, 34, 70, 449, 477
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x833C
// 首条位置: {"module":593,"inlinee":33596}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 25, 28, 31, 34, 70, 449, 477
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x82CA
// 首条位置: {"module":1729,"inlinee":33482}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$11::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13004
// 首条位置: {"module":1729,"inlinee":77828}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$13::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13000
// 首条位置: {"module":1729,"inlinee":77824}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 31, 34, 70, 449, 477
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12222
// 首条位置: {"module":1630,"inlinee":74274}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":210068,"parent_record_offset":207504,"ancestor_record_offset":207504,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":210588,"parent_record_offset":207504,"ancestor_record_offset":207504,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":230616,"parent_record_offset":227240,"ancestor_record_offset":227240,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":231716,"parent_record_offset":227240,"ancestor_record_offset":227240,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":207540,"parent_record_offset":207504,"ancestor_record_offset":207504,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=28
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":209792,"parent_record_offset":207504,"ancestor_record_offset":207504,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":208340,"parent_record_offset":208316,"ancestor_record_offset":207504,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":208316,"parent_record_offset":208260,"ancestor_record_offset":207504,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":207684,"parent_record_offset":207660,"ancestor_record_offset":207504,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":207736,"parent_record_offset":207708,"ancestor_record_offset":207504,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":207760,"parent_record_offset":207736,"ancestor_record_offset":207504,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":208700,"parent_record_offset":208676,"ancestor_record_offset":207504,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":208288,"parent_record_offset":208260,"ancestor_record_offset":207504,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":208260,"parent_record_offset":208236,"ancestor_record_offset":207504,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":208748,"parent_record_offset":208724,"ancestor_record_offset":207504,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":208724,"parent_record_offset":208700,"ancestor_record_offset":207504,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":208236,"parent_record_offset":208212,"ancestor_record_offset":207504,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":208676,"parent_record_offset":208652,"ancestor_record_offset":207504,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":210444,"parent_record_offset":210360,"ancestor_record_offset":207504,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":208652,"parent_record_offset":208392,"ancestor_record_offset":207504,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":209204,"parent_record_offset":209180,"ancestor_record_offset":207504,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1630,"record_offset":210504,"parent_record_offset":210360,"ancestor_record_offset":207504,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":209908,"parent_record_offset":209884,"ancestor_record_offset":207504,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":396340,"parent_record_offset":396312,"ancestor_record_offset":392548,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":396368,"parent_record_offset":396340,"ancestor_record_offset":392548,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":208608,"parent_record_offset":208580,"ancestor_record_offset":207504,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":207708,"parent_record_offset":207684,"ancestor_record_offset":207504,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":209884,"parent_record_offset":209848,"ancestor_record_offset":207504,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":207876,"parent_record_offset":207788,"ancestor_record_offset":207504,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=86
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":208104,"parent_record_offset":207612,"ancestor_record_offset":207504,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1630,"record_offset":210472,"parent_record_offset":210444,"ancestor_record_offset":207504,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1630,"record_offset":396404,"parent_record_offset":396252,"ancestor_record_offset":392548,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":207900,"parent_record_offset":207876,"ancestor_record_offset":207504,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":209964,"parent_record_offset":209908,"ancestor_record_offset":207504,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":400112,"parent_record_offset":398060,"ancestor_record_offset":397932,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":210416,"parent_record_offset":210360,"ancestor_record_offset":207504,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":209228,"parent_record_offset":209204,"ancestor_record_offset":207504,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":209252,"parent_record_offset":209228,"ancestor_record_offset":207504,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":207788,"parent_record_offset":207760,"ancestor_record_offset":207504,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":207844,"parent_record_offset":207820,"ancestor_record_offset":207504,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":207820,"parent_record_offset":207788,"ancestor_record_offset":207504,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":209180,"parent_record_offset":209156,"ancestor_record_offset":207504,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":402220,"parent_record_offset":402196,"ancestor_record_offset":397932,"inlinee":28073,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":402268,"parent_record_offset":402244,"ancestor_record_offset":397932,"inlinee":28014,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1630,"record_offset":209156,"parent_record_offset":209128,"ancestor_record_offset":207504,"inlinee":74277,"source_line":477,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1630,"record_offset":209032,"parent_record_offset":209008,"ancestor_record_offset":207504,"inlinee":74276,"source_line":17,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1630,"record_offset":210336,"parent_record_offset":210312,"ancestor_record_offset":207504,"inlinee":74279,"source_line":449,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":231556,"parent_record_offset":231532,"ancestor_record_offset":227240,"inlinee":31381,"source_line":591,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":231604,"parent_record_offset":231580,"ancestor_record_offset":227240,"inlinee":31375,"source_line":155,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1630,"record_offset":396252,"parent_record_offset":396220,"ancestor_record_offset":392548,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1630,"record_offset":396284,"parent_record_offset":396252,"ancestor_record_offset":392548,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1630,"record_offset":396184,"parent_record_offset":396156,"ancestor_record_offset":392548,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1630,"record_offset":396220,"parent_record_offset":396184,"ancestor_record_offset":392548,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1630,"record_offset":396644,"parent_record_offset":396616,"ancestor_record_offset":392548,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1630,"record_offset":396312,"parent_record_offset":396252,"ancestor_record_offset":392548,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1630,"record_offset":210684,"parent_record_offset":210660,"ancestor_record_offset":207504,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":231844,"parent_record_offset":231820,"ancestor_record_offset":227240,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":406312,"parent_record_offset":406288,"ancestor_record_offset":397932,"inlinee":6675,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":231932,"parent_record_offset":231908,"ancestor_record_offset":227240,"inlinee":6671,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_map
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":396008,"parent_record_offset":395984,"ancestor_record_offset":392548,"inlinee":73644,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":231820,"parent_record_offset":231796,"ancestor_record_offset":227240,"inlinee":6705,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":406536,"parent_record_offset":406512,"ancestor_record_offset":397932,"inlinee":5746,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":406648,"parent_record_offset":406624,"ancestor_record_offset":397932,"inlinee":28015,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":207576,"parent_record_offset":207540,"ancestor_record_offset":207504,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=28
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":209820,"parent_record_offset":209792,"ancestor_record_offset":207504,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":209128,"parent_record_offset":209104,"ancestor_record_offset":207504,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":210024,"parent_record_offset":210000,"ancestor_record_offset":207504,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=33
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":209008,"parent_record_offset":207612,"ancestor_record_offset":207504,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":210388,"parent_record_offset":210360,"ancestor_record_offset":207504,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$204::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":395548,"parent_record_offset":395524,"ancestor_record_offset":392548,"inlinee":73646,"source_line":1505,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":404136,"parent_record_offset":404112,"ancestor_record_offset":397932,"inlinee":6537,"source_line":72,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$63::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":404776,"parent_record_offset":404752,"ancestor_record_offset":397932,"inlinee":6547,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":231628,"parent_record_offset":231604,"ancestor_record_offset":227240,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":230908,"parent_record_offset":230884,"ancestor_record_offset":227240,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":207660,"parent_record_offset":207612,"ancestor_record_offset":207504,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":209080,"parent_record_offset":209056,"ancestor_record_offset":207504,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":209056,"parent_record_offset":209032,"ancestor_record_offset":207504,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":209104,"parent_record_offset":209080,"ancestor_record_offset":207504,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":231580,"parent_record_offset":231556,"ancestor_record_offset":227240,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":207612,"parent_record_offset":207576,"ancestor_record_offset":207504,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=28
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":209848,"parent_record_offset":209820,"ancestor_record_offset":207504,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] vectorize::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"vectorize","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vectorize-0.2.0/src/lib.rs
//     evidence={"module":1630,"record_offset":210360,"parent_record_offset":210336,"ancestor_record_offset":207504,"inlinee":74033,"source_line":17,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$15::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1300A
// 首条位置: {"module":1729,"inlinee":77834}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 34, 449, 477
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12225
// 首条位置: {"module":1630,"inlinee":74277}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":209180,"parent_record_offset":209156,"ancestor_record_offset":209156,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":209204,"parent_record_offset":209180,"ancestor_record_offset":209156,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":209228,"parent_record_offset":209204,"ancestor_record_offset":209156,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":209252,"parent_record_offset":209228,"ancestor_record_offset":209156,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 34, 70, 449, 477
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1220A
// 首条位置: {"module":1630,"inlinee":74250}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":91656,"parent_record_offset":90908,"ancestor_record_offset":90908,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":68924,"parent_record_offset":66120,"ancestor_record_offset":66120,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$6::next_element
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":23388,"parent_record_offset":23360,"ancestor_record_offset":23360,"inlinee":11489,"source_line":1753,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":92080,"parent_record_offset":92052,"ancestor_record_offset":90908,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":92052,"parent_record_offset":91836,"ancestor_record_offset":90908,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":120116,"parent_record_offset":120032,"ancestor_record_offset":119724,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1630,"record_offset":23464,"parent_record_offset":23436,"ancestor_record_offset":23360,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1630,"record_offset":120176,"parent_record_offset":120032,"ancestor_record_offset":119724,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":91956,"parent_record_offset":91928,"ancestor_record_offset":90908,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":91984,"parent_record_offset":91956,"ancestor_record_offset":90908,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":91684,"parent_record_offset":91656,"ancestor_record_offset":90908,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1630,"record_offset":120144,"parent_record_offset":120116,"ancestor_record_offset":119724,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1630,"record_offset":92020,"parent_record_offset":91868,"ancestor_record_offset":90908,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1630,"record_offset":23536,"parent_record_offset":23464,"ancestor_record_offset":23360,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1630,"record_offset":23564,"parent_record_offset":23464,"ancestor_record_offset":23360,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1630,"record_offset":23504,"parent_record_offset":23464,"ancestor_record_offset":23360,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":23588,"parent_record_offset":23564,"ancestor_record_offset":23360,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":23652,"parent_record_offset":23436,"ancestor_record_offset":23360,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":24064,"parent_record_offset":24012,"ancestor_record_offset":23360,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":67380,"parent_record_offset":67356,"ancestor_record_offset":66120,"inlinee":28073,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":67428,"parent_record_offset":67404,"ancestor_record_offset":66120,"inlinee":28014,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1630,"record_offset":120012,"parent_record_offset":119988,"ancestor_record_offset":119724,"inlinee":74264,"source_line":449,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":23964,"parent_record_offset":23940,"ancestor_record_offset":23360,"inlinee":31381,"source_line":591,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":24012,"parent_record_offset":23988,"ancestor_record_offset":23360,"inlinee":31375,"source_line":155,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1630,"record_offset":91868,"parent_record_offset":91836,"ancestor_record_offset":90908,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1630,"record_offset":91900,"parent_record_offset":91868,"ancestor_record_offset":90908,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1630,"record_offset":91768,"parent_record_offset":91740,"ancestor_record_offset":90908,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1630,"record_offset":91836,"parent_record_offset":91768,"ancestor_record_offset":90908,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1630,"record_offset":91804,"parent_record_offset":91768,"ancestor_record_offset":90908,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1630,"record_offset":91928,"parent_record_offset":91868,"ancestor_record_offset":90908,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1630,"record_offset":91740,"parent_record_offset":91712,"ancestor_record_offset":90908,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":23412,"parent_record_offset":23388,"ancestor_record_offset":23360,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":23624,"parent_record_offset":23436,"ancestor_record_offset":23360,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":120060,"parent_record_offset":120032,"ancestor_record_offset":119724,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$204::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":91216,"parent_record_offset":91192,"ancestor_record_offset":90908,"inlinee":73646,"source_line":1505,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":68848,"parent_record_offset":68824,"ancestor_record_offset":66120,"inlinee":6537,"source_line":72,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$63::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":68172,"parent_record_offset":68148,"ancestor_record_offset":66120,"inlinee":6547,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":24036,"parent_record_offset":24012,"ancestor_record_offset":23360,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":24392,"parent_record_offset":24368,"ancestor_record_offset":23360,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":23988,"parent_record_offset":23964,"ancestor_record_offset":23360,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$7::next_element_seed
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":23436,"parent_record_offset":23412,"ancestor_record_offset":23360,"inlinee":29591,"source_line":575,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] vectorize::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"vectorize","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vectorize-0.2.0/src/lib.rs
//     evidence={"module":1630,"record_offset":120032,"parent_record_offset":120012,"ancestor_record_offset":119724,"inlinee":74033,"source_line":17,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3946
// 首条位置: {"module":199,"inlinee":14662}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":17452,"parent_record_offset":17424,"ancestor_record_offset":17424,"inlinee":5727,"source_line":409,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":18108,"parent_record_offset":17424,"ancestor_record_offset":17424,"inlinee":5728,"source_line":774,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":17996,"parent_record_offset":17424,"ancestor_record_offset":17424,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":17848,"parent_record_offset":17816,"ancestor_record_offset":17424,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":17716,"parent_record_offset":17688,"ancestor_record_offset":17424,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":17776,"parent_record_offset":17744,"ancestor_record_offset":17424,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":17896,"parent_record_offset":17872,"ancestor_record_offset":17424,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":17872,"parent_record_offset":17848,"ancestor_record_offset":17424,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":17744,"parent_record_offset":17716,"ancestor_record_offset":17424,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":17648,"parent_record_offset":17620,"ancestor_record_offset":17424,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":17816,"parent_record_offset":17648,"ancestor_record_offset":17424,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":17592,"parent_record_offset":17564,"ancestor_record_offset":17424,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":17936,"parent_record_offset":17648,"ancestor_record_offset":17424,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":17688,"parent_record_offset":17648,"ancestor_record_offset":17424,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":199,"record_offset":17620,"parent_record_offset":17592,"ancestor_record_offset":17424,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":17508,"parent_record_offset":17480,"ancestor_record_offset":17424,"inlinee":5420,"source_line":1849,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":18164,"parent_record_offset":18136,"ancestor_record_offset":17424,"inlinee":5435,"source_line":1859,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":17480,"parent_record_offset":17452,"ancestor_record_offset":17424,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":18136,"parent_record_offset":18108,"ancestor_record_offset":17424,"inlinee":5730,"source_line":720,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":199,"record_offset":17564,"parent_record_offset":17536,"ancestor_record_offset":17424,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":199,"record_offset":17536,"parent_record_offset":17508,"ancestor_record_offset":17424,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$18::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1300E
// 首条位置: {"module":1729,"inlinee":77838}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=1
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1729,"record_offset":43600,"rva":24906992,"va":5393616112}
// 编译实例 RVAs: 0x17c0cf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18534862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393616112,"raw_hex":"e8322ee5fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5393616361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14085db60
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16135686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393616112,"raw_hex":"e8fac909ff","size":5,"soft_fail":false,"target_va":5377481568,"va":5393617249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140898f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15892113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393616112,"raw_hex":"e86f810dff","size":5,"soft_fail":false,"target_va":5377724224,"va":5393616332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b864c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12823408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393616112,"raw_hex":"e890543cff","size":5,"soft_fail":false,"target_va":5380793536,"va":5393616939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141592dd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2286035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393616112,"raw_hex":"e82d1eddff","size":5,"soft_fail":false,"target_va":5391330768,"va":5393616798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141593290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2284262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393616112,"raw_hex":"e81a25ddff","size":5,"soft_fail":false,"target_va":5391331984,"va":5393616241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141595090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2277183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393616112,"raw_hex":"e8c140ddff","size":5,"soft_fail":false,"target_va":5391339664,"va":5393616842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141595530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2275468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393616112,"raw_hex":"e87447ddff","size":5,"soft_fail":false,"target_va":5391340848,"va":5393616311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141595a20
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2274867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393616112,"raw_hex":"e8cd49ddff","size":5,"soft_fail":false,"target_va":5391342112,"va":5393616974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141597e60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2265115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393616112,"raw_hex":"e8e56fddff","size":5,"soft_fail":false,"target_va":5391351392,"va":5393616502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141598030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2264425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393616112,"raw_hex":"e89772ddff","size":5,"soft_fail":false,"target_va":5391351856,"va":5393616276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141599080
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2260538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393616112,"raw_hex":"e8c681ddff","size":5,"soft_fail":false,"target_va":5391356032,"va":5393616565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141599250
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2259931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393616112,"raw_hex":"e82584ddff","size":5,"soft_fail":false,"target_va":5391356496,"va":5393616422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141599a30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2257950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393616112,"raw_hex":"e8e28bddff","size":5,"soft_fail":false,"target_va":5391358512,"va":5393616457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1729,"record_offset":44484,"parent_record_offset":43600,"ancestor_record_offset":43600,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1729,"record_offset":44456,"parent_record_offset":43600,"ancestor_record_offset":43600,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::decorator_edit::is_default
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1729,"record_offset":43812,"parent_record_offset":43600,"ancestor_record_offset":43600,"inlinee":14664,"source_line":136,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::decorator_edit::is_default_lantern_brightness
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1729,"record_offset":45028,"parent_record_offset":43600,"ancestor_record_offset":43600,"inlinee":77845,"source_line":152,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::decorator_edit::is_default_stair_width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1729,"record_offset":45056,"parent_record_offset":43600,"ancestor_record_offset":43600,"inlinee":33469,"source_line":168,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::ClutterVersion::is_initial
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1729,"record_offset":45000,"parent_record_offset":43600,"ancestor_record_offset":43600,"inlinee":26024,"source_line":220,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1729,"record_offset":44712,"parent_record_offset":44688,"ancestor_record_offset":43600,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1729,"record_offset":44688,"parent_record_offset":44632,"ancestor_record_offset":43600,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1729,"record_offset":44660,"parent_record_offset":44632,"ancestor_record_offset":43600,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1729,"record_offset":44632,"parent_record_offset":44608,"ancestor_record_offset":43600,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1729,"record_offset":44608,"parent_record_offset":44584,"ancestor_record_offset":43600,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1729,"record_offset":43960,"parent_record_offset":43932,"ancestor_record_offset":43600,"inlinee":8507,"source_line":1740,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1729,"record_offset":43840,"parent_record_offset":43812,"ancestor_record_offset":43600,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1729,"record_offset":44512,"parent_record_offset":44484,"ancestor_record_offset":43600,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::impl$14::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1729,"record_offset":43868,"parent_record_offset":43840,"ancestor_record_offset":43600,"inlinee":14666,"source_line":26,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$77::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1729,"record_offset":44236,"parent_record_offset":44208,"ancestor_record_offset":43600,"inlinee":14668,"source_line":323,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$84::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1729,"record_offset":44328,"parent_record_offset":44300,"ancestor_record_offset":43600,"inlinee":14670,"source_line":332,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$19::clone
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x839B
// 首条位置: {"module":599,"inlinee":33691}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$20::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1300C
// 首条位置: {"module":1729,"inlinee":77836}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$22::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120EB
// 首条位置: {"module":1613,"inlinee":73963}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::color_id::impl$14::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1613,"record_offset":28604,"parent_record_offset":28088,"ancestor_record_offset":28088,"inlinee":14666,"source_line":26,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$102::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1613,"record_offset":28168,"parent_record_offset":28088,"ancestor_record_offset":28088,"inlinee":25016,"source_line":518,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$23::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1613,"record_offset":28548,"parent_record_offset":28088,"ancestor_record_offset":28088,"inlinee":24851,"source_line":21,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$38::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1613,"record_offset":28632,"parent_record_offset":28088,"ancestor_record_offset":28088,"inlinee":58545,"source_line":180,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$54::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1613,"record_offset":28576,"parent_record_offset":28088,"ancestor_record_offset":28088,"inlinee":33461,"source_line":242,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$77::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1613,"record_offset":28716,"parent_record_offset":28088,"ancestor_record_offset":28088,"inlinee":14668,"source_line":323,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$84::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1613,"record_offset":28744,"parent_record_offset":28088,"ancestor_record_offset":28088,"inlinee":14670,"source_line":332,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1613,"record_offset":28204,"parent_record_offset":28168,"ancestor_record_offset":28088,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":1613,"record_offset":28480,"parent_record_offset":28448,"ancestor_record_offset":28088,"inlinee":4501,"source_line":38,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$107::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1613,"record_offset":28228,"parent_record_offset":28204,"ancestor_record_offset":28088,"inlinee":24980,"source_line":584,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$122::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1613,"record_offset":28448,"parent_record_offset":28424,"ancestor_record_offset":28088,"inlinee":24988,"source_line":614,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1613,"record_offset":28384,"parent_record_offset":28284,"ancestor_record_offset":28088,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_attachment::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":1613,"record_offset":28356,"parent_record_offset":28284,"ancestor_record_offset":28088,"inlinee":24986,"source_line":44,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_attachment::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":1613,"record_offset":28284,"parent_record_offset":28228,"ancestor_record_offset":28088,"inlinee":24984,"source_line":6,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$102::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1613,"record_offset":28508,"parent_record_offset":28448,"ancestor_record_offset":28088,"inlinee":8499,"source_line":17,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$104::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1613,"record_offset":28320,"parent_record_offset":28284,"ancestor_record_offset":28088,"inlinee":8505,"source_line":17,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::impl$15::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1613,"record_offset":28256,"parent_record_offset":28228,"ancestor_record_offset":28088,"inlinee":24982,"source_line":106,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::is_default
// 记录形式: inline_instance, inlinee_declaration; instances=29
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3948
// 首条位置: {"module":199,"inlinee":14664}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":199,"record_offset":19252,"parent_record_offset":19224,"ancestor_record_offset":19224,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":199,"record_offset":19372,"parent_record_offset":19344,"ancestor_record_offset":19316,"inlinee":8507,"source_line":1740,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::impl$14::eq
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":199,"record_offset":19280,"parent_record_offset":19252,"ancestor_record_offset":19224,"inlinee":14666,"source_line":26,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$77::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":199,"record_offset":19740,"parent_record_offset":19712,"ancestor_record_offset":19684,"inlinee":14668,"source_line":323,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$84::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":199,"record_offset":19832,"parent_record_offset":19804,"ancestor_record_offset":19776,"inlinee":14670,"source_line":332,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::is_default_chimney_smoke
// 记录形式: compiled_procedure; instances=1
// 源码行: 145
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":587,"record_offset":121568,"rva":9197184,"va":5377906304}
// 编译实例 RVAs: 0x8c5680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::is_default_lantern_brightness
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13015
// 首条位置: {"module":1729,"inlinee":77845}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::is_default_stair_width
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x82BD
// 首条位置: {"module":587,"record_offset":121720,"rva":11794704,"va":5380503824}
// 编译实例 RVAs: 0xb3f910
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$1::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 173
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":587,"record_offset":121868,"rva":11794736,"va":5380503856}
// 编译实例 RVAs: 0xb3f930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31037678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380503856,"raw_hex":"e8ee98d901","size":5,"soft_fail":false,"target_va":5411541904,"va":5380504221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":587,"record_offset":122424,"parent_record_offset":121868,"ancestor_record_offset":121868,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":587,"record_offset":122396,"parent_record_offset":121868,"ancestor_record_offset":121868,"inlinee":9187,"source_line":672,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":587,"record_offset":122368,"parent_record_offset":121868,"ancestor_record_offset":121868,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":587,"record_offset":122080,"parent_record_offset":121868,"ancestor_record_offset":121868,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::DecoratorExtraInfo::try_as_chimney
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":587,"record_offset":122048,"parent_record_offset":121868,"ancestor_record_offset":121868,"inlinee":26086,"source_line":368,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::DecoratorExtraInfo::try_as_flag
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":587,"record_offset":122340,"parent_record_offset":121868,"ancestor_record_offset":121868,"inlinee":26089,"source_line":368,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::DecoratorExtraInfo::try_as_lantern
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":587,"record_offset":122256,"parent_record_offset":121868,"ancestor_record_offset":121868,"inlinee":26095,"source_line":368,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::DecoratorExtraInfo::try_as_stair
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":587,"record_offset":122112,"parent_record_offset":121868,"ancestor_record_offset":121868,"inlinee":26098,"source_line":368,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::DecoratorExtraInfo::try_as_window
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":587,"record_offset":122312,"parent_record_offset":121868,"ancestor_record_offset":121868,"inlinee":26092,"source_line":368,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$2::from
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 229
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x82C1
// 首条位置: {"module":599,"inlinee":33473}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":599,"record_offset":204740,"parent_record_offset":204716,"ancestor_record_offset":204716,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00599.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$3::from
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 235
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x656E
// 首条位置: {"module":475,"inlinee":25966}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":540,"record_offset":101644,"parent_record_offset":101548,"ancestor_record_offset":101548,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$97::from
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":540,"record_offset":101668,"parent_record_offset":101644,"ancestor_record_offset":101548,"inlinee":25964,"source_line":403,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$4::from
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 282
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6599
// 首条位置: {"module":599,"inlinee":26009}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1729,"record_offset":9704,"parent_record_offset":9664,"ancestor_record_offset":9664,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1729,"record_offset":9820,"parent_record_offset":9744,"ancestor_record_offset":9664,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::impl$3::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1729,"record_offset":9744,"parent_record_offset":9704,"ancestor_record_offset":9664,"inlinee":25966,"source_line":235,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$97::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1729,"record_offset":9844,"parent_record_offset":9820,"ancestor_record_offset":9664,"inlinee":25964,"source_line":403,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$23::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 288
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x81FF
// 首条位置: {"module":584,"inlinee":33279}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":584,"record_offset":29204,"parent_record_offset":29176,"ancestor_record_offset":29176,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":584,"record_offset":29752,"parent_record_offset":29720,"ancestor_record_offset":29176,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":584,"record_offset":29812,"parent_record_offset":29780,"ancestor_record_offset":29176,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":584,"record_offset":29608,"parent_record_offset":29584,"ancestor_record_offset":29176,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":584,"record_offset":29780,"parent_record_offset":29720,"ancestor_record_offset":29176,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":584,"record_offset":29356,"parent_record_offset":29316,"ancestor_record_offset":29176,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":584,"record_offset":29472,"parent_record_offset":29444,"ancestor_record_offset":29176,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":584,"record_offset":29560,"parent_record_offset":29444,"ancestor_record_offset":29176,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":584,"record_offset":29636,"parent_record_offset":29608,"ancestor_record_offset":29176,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":584,"record_offset":29584,"parent_record_offset":29560,"ancestor_record_offset":29176,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":584,"record_offset":29664,"parent_record_offset":29608,"ancestor_record_offset":29176,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":584,"record_offset":29260,"parent_record_offset":29232,"ancestor_record_offset":29176,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":584,"record_offset":29520,"parent_record_offset":29496,"ancestor_record_offset":29176,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":584,"record_offset":29384,"parent_record_offset":29356,"ancestor_record_offset":29176,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":584,"record_offset":29720,"parent_record_offset":29316,"ancestor_record_offset":29176,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":584,"record_offset":29232,"parent_record_offset":29204,"ancestor_record_offset":29176,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":584,"record_offset":29412,"parent_record_offset":29384,"ancestor_record_offset":29176,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":584,"record_offset":29444,"parent_record_offset":29412,"ancestor_record_offset":29176,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":584,"record_offset":29316,"parent_record_offset":29288,"ancestor_record_offset":29176,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":584,"record_offset":29288,"parent_record_offset":29260,"ancestor_record_offset":29176,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$25::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 288
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":584,"record_offset":81520,"rva":11733520,"va":5380442640}
// 编译实例 RVAs: 0xb30a10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field1_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2449817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442640,"raw_hex":"e899612500","size":5,"soft_fail":false,"target_va":5382892528,"va":5380442706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$5::from
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 294
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x81F2
// 首条位置: {"module":584,"inlinee":33266}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":584,"record_offset":3304,"parent_record_offset":2888,"ancestor_record_offset":2888,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":584,"record_offset":2912,"parent_record_offset":2888,"ancestor_record_offset":2888,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":584,"record_offset":3160,"parent_record_offset":3132,"ancestor_record_offset":2888,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":584,"record_offset":3220,"parent_record_offset":3188,"ancestor_record_offset":2888,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":584,"record_offset":3104,"parent_record_offset":3072,"ancestor_record_offset":2888,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":584,"record_offset":3132,"parent_record_offset":3072,"ancestor_record_offset":2888,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":584,"record_offset":3188,"parent_record_offset":3132,"ancestor_record_offset":2888,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":1610,"record_offset":5032,"parent_record_offset":5008,"ancestor_record_offset":4984,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":584,"record_offset":2936,"parent_record_offset":2912,"ancestor_record_offset":2888,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":584,"record_offset":3072,"parent_record_offset":2988,"ancestor_record_offset":2888,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1610,"record_offset":5060,"parent_record_offset":5032,"ancestor_record_offset":4984,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":584,"record_offset":2964,"parent_record_offset":2936,"ancestor_record_offset":2888,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1610,"record_offset":5144,"parent_record_offset":5084,"ancestor_record_offset":4984,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":584,"record_offset":3044,"parent_record_offset":2988,"ancestor_record_offset":2888,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1610,"record_offset":5116,"parent_record_offset":5084,"ancestor_record_offset":4984,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":584,"record_offset":3016,"parent_record_offset":2988,"ancestor_record_offset":2888,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1610,"record_offset":5084,"parent_record_offset":5060,"ancestor_record_offset":4984,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":584,"record_offset":2988,"parent_record_offset":2964,"ancestor_record_offset":2888,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$5::from::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 299
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x83AA
// 首条位置: {"module":599,"inlinee":33706}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":599,"record_offset":204692,"parent_record_offset":204612,"ancestor_record_offset":204612,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00599.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$58::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":599,"record_offset":204632,"parent_record_offset":204612,"ancestor_record_offset":204612,"inlinee":25001,"source_line":277,"archive":"_pdb/B/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":599,"record_offset":204740,"parent_record_offset":204716,"ancestor_record_offset":204612,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::impl$2::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":599,"record_offset":204716,"parent_record_offset":204692,"ancestor_record_offset":204612,"inlinee":33473,"source_line":229,"archive":"_pdb/B/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$89::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":599,"record_offset":204664,"parent_record_offset":204632,"ancestor_record_offset":204612,"inlinee":25003,"source_line":351,"archive":"_pdb/B/modules/00599.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 322
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1729,"record_offset":8680,"rva":24894176,"va":5393603296}
// 编译实例 RVAs: 0x17bdae0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18523691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393603296,"raw_hex":"e8d559e5fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5393605190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17936970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393603296,"raw_hex":"e84ab21101","size":5,"soft_fail":false,"target_va":5411542064,"va":5393605089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::decorator_edit::convert_before_after_states_to_changes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13100424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393603296,"raw_hex":"e8781a38ff","size":5,"soft_fail":false,"target_va":5380504240,"va":5393604659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DeferredDecoratorOpEvents::send
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-16295477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393603296,"raw_hex":"e8cb5907ff","size":5,"soft_fail":false,"target_va":5377308336,"va":5393603808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-18523524","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5393603296,"raw_hex":"e97c5ae5fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5393605023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1729,"record_offset":9624,"parent_record_offset":8680,"ancestor_record_offset":8680,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1729,"record_offset":10700,"parent_record_offset":8680,"ancestor_record_offset":8680,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::decorator_edit::impl$19::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1729,"record_offset":10636,"parent_record_offset":8680,"ancestor_record_offset":8680,"inlinee":33691,"source_line":70,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_pop_bucket.rs
//     evidence={"module":1729,"record_offset":9892,"parent_record_offset":8680,"ancestor_record_offset":8680,"inlinee":77842,"source_line":43,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$46::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1729,"record_offset":8944,"parent_record_offset":8680,"ancestor_record_offset":8680,"inlinee":10297,"source_line":2382,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$54::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1729,"record_offset":9920,"parent_record_offset":8680,"ancestor_record_offset":8680,"inlinee":10299,"source_line":2474,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1729,"record_offset":10824,"parent_record_offset":10800,"ancestor_record_offset":8680,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1729,"record_offset":10800,"parent_record_offset":10772,"ancestor_record_offset":8680,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1729,"record_offset":9704,"parent_record_offset":9664,"ancestor_record_offset":8680,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1729,"record_offset":9244,"parent_record_offset":9216,"ancestor_record_offset":8680,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1729,"record_offset":9304,"parent_record_offset":9272,"ancestor_record_offset":8680,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1729,"record_offset":9544,"parent_record_offset":9460,"ancestor_record_offset":8680,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1729,"record_offset":9484,"parent_record_offset":9460,"ancestor_record_offset":8680,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1729,"record_offset":9512,"parent_record_offset":9484,"ancestor_record_offset":8680,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1729,"record_offset":10212,"parent_record_offset":10180,"ancestor_record_offset":8680,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1729,"record_offset":10180,"parent_record_offset":10148,"ancestor_record_offset":8680,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1729,"record_offset":9340,"parent_record_offset":9184,"ancestor_record_offset":8680,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1729,"record_offset":10724,"parent_record_offset":10700,"ancestor_record_offset":8680,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1729,"record_offset":9576,"parent_record_offset":9416,"ancestor_record_offset":8680,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::impl$3::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1729,"record_offset":9744,"parent_record_offset":9704,"ancestor_record_offset":8680,"inlinee":25966,"source_line":235,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::impl$4::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1729,"record_offset":9664,"parent_record_offset":9624,"ancestor_record_offset":8680,"inlinee":26009,"source_line":282,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$97::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1729,"record_offset":9844,"parent_record_offset":9820,"ancestor_record_offset":8680,"inlinee":25964,"source_line":403,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1729,"record_offset":10108,"parent_record_offset":10068,"ancestor_record_offset":8680,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1729,"record_offset":10148,"parent_record_offset":10108,"ancestor_record_offset":8680,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1729,"record_offset":10256,"parent_record_offset":10068,"ancestor_record_offset":8680,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1729,"record_offset":10068,"parent_record_offset":10020,"ancestor_record_offset":8680,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1729,"record_offset":9216,"parent_record_offset":9184,"ancestor_record_offset":8680,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1729,"record_offset":9272,"parent_record_offset":9216,"ancestor_record_offset":8680,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1729,"record_offset":10348,"parent_record_offset":10020,"ancestor_record_offset":8680,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$42::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1729,"record_offset":8980,"parent_record_offset":8944,"ancestor_record_offset":8680,"inlinee":4805,"source_line":3151,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$53::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1729,"record_offset":9944,"parent_record_offset":9920,"ancestor_record_offset":8680,"inlinee":4824,"source_line":3275,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1729,"record_offset":10448,"parent_record_offset":10020,"ancestor_record_offset":8680,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1729,"record_offset":9184,"parent_record_offset":9092,"ancestor_record_offset":8680,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1729,"record_offset":10020,"parent_record_offset":9992,"ancestor_record_offset":8680,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_allocation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1729,"record_offset":9416,"parent_record_offset":9380,"ancestor_record_offset":8680,"inlinee":4809,"source_line":1358,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_iter_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1729,"record_offset":9380,"parent_record_offset":9016,"ancestor_record_offset":8680,"inlinee":4808,"source_line":1344,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1729,"record_offset":9060,"parent_record_offset":9016,"ancestor_record_offset":8680,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1729,"record_offset":9152,"parent_record_offset":9092,"ancestor_record_offset":8680,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1729,"record_offset":9124,"parent_record_offset":9092,"ancestor_record_offset":8680,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1729,"record_offset":9092,"parent_record_offset":9060,"ancestor_record_offset":8680,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1729,"record_offset":9460,"parent_record_offset":9416,"ancestor_record_offset":8680,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$19::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1729,"record_offset":9016,"parent_record_offset":8980,"ancestor_record_offset":8680,"inlinee":4807,"source_line":3343,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1729,"record_offset":9992,"parent_record_offset":9968,"ancestor_record_offset":8680,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$39::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1729,"record_offset":10772,"parent_record_offset":10748,"ancestor_record_offset":8680,"inlinee":4904,"source_line":3846,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$41::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1729,"record_offset":9968,"parent_record_offset":9944,"ancestor_record_offset":8680,"inlinee":4826,"source_line":3887,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::convert_before_after_states_to_changes
// 记录形式: compiled_procedure; instances=1
// 源码行: 380
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":587,"record_offset":122456,"rva":11795120,"va":5380504240}
// 编译实例 RVAs: 0xb3fab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":587,"record_offset":122628,"parent_record_offset":122456,"ancestor_record_offset":122456,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":587,"record_offset":122748,"parent_record_offset":122712,"ancestor_record_offset":122456,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":587,"record_offset":122980,"parent_record_offset":122952,"ancestor_record_offset":122456,"inlinee":4501,"source_line":38,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$102::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":587,"record_offset":122712,"parent_record_offset":122680,"ancestor_record_offset":122456,"inlinee":25016,"source_line":518,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$107::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":587,"record_offset":122772,"parent_record_offset":122748,"ancestor_record_offset":122456,"inlinee":24980,"source_line":584,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$122::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":587,"record_offset":122952,"parent_record_offset":122928,"ancestor_record_offset":122456,"inlinee":24988,"source_line":614,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$23::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":587,"record_offset":123104,"parent_record_offset":123080,"ancestor_record_offset":122456,"inlinee":24851,"source_line":21,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":587,"record_offset":123004,"parent_record_offset":122980,"ancestor_record_offset":122456,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$54::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":587,"record_offset":123160,"parent_record_offset":123136,"ancestor_record_offset":122456,"inlinee":33461,"source_line":242,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":587,"record_offset":122652,"parent_record_offset":122628,"ancestor_record_offset":122456,"inlinee":4988,"source_line":476,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":587,"record_offset":122888,"parent_record_offset":122828,"ancestor_record_offset":122456,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_attachment::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":587,"record_offset":122828,"parent_record_offset":122772,"ancestor_record_offset":122456,"inlinee":24984,"source_line":6,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$102::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":587,"record_offset":123036,"parent_record_offset":122952,"ancestor_record_offset":122456,"inlinee":8499,"source_line":17,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$104::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":587,"record_offset":122860,"parent_record_offset":122828,"ancestor_record_offset":122456,"inlinee":8505,"source_line":17,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::impl$15::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":587,"record_offset":122800,"parent_record_offset":122772,"ancestor_record_offset":122456,"inlinee":24982,"source_line":106,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 449
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12227
// 首条位置: {"module":1630,"inlinee":74279}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":210536,"parent_record_offset":210336,"ancestor_record_offset":210336,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] vectorize::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"vectorize","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vectorize-0.2.0/src/lib.rs
//     evidence={"module":1630,"record_offset":210360,"parent_record_offset":210336,"ancestor_record_offset":210336,"inlinee":74033,"source_line":17,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":210444,"parent_record_offset":210360,"ancestor_record_offset":210336,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1630,"record_offset":210504,"parent_record_offset":210360,"ancestor_record_offset":210336,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1630,"record_offset":210472,"parent_record_offset":210444,"ancestor_record_offset":210336,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":210416,"parent_record_offset":210360,"ancestor_record_offset":210336,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":210388,"parent_record_offset":210360,"ancestor_record_offset":210336,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 449
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12218
// 首条位置: {"module":1630,"inlinee":74264}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":120208,"parent_record_offset":120012,"ancestor_record_offset":120012,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] vectorize::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"vectorize","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vectorize-0.2.0/src/lib.rs
//     evidence={"module":1630,"record_offset":120032,"parent_record_offset":120012,"ancestor_record_offset":120012,"inlinee":74033,"source_line":17,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":120116,"parent_record_offset":120032,"ancestor_record_offset":120012,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1630,"record_offset":120176,"parent_record_offset":120032,"ancestor_record_offset":120012,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1630,"record_offset":120144,"parent_record_offset":120116,"ancestor_record_offset":120012,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":120088,"parent_record_offset":120032,"ancestor_record_offset":120012,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":120060,"parent_record_offset":120032,"ancestor_record_offset":120012,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::serialize::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 449
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x385E
// 首条位置: {"module":191,"inlinee":14430}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$27::default
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 449
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7BE7
// 首条位置: {"module":555,"inlinee":31719}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":555,"record_offset":23652,"parent_record_offset":23624,"ancestor_record_offset":23624,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":555,"record_offset":23852,"parent_record_offset":23824,"ancestor_record_offset":23624,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":555,"record_offset":23904,"parent_record_offset":23880,"ancestor_record_offset":23624,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":555,"record_offset":23880,"parent_record_offset":23824,"ancestor_record_offset":23624,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":555,"record_offset":23928,"parent_record_offset":23904,"ancestor_record_offset":23624,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":555,"record_offset":23796,"parent_record_offset":23768,"ancestor_record_offset":23624,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":555,"record_offset":23952,"parent_record_offset":23928,"ancestor_record_offset":23624,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":555,"record_offset":23708,"parent_record_offset":23680,"ancestor_record_offset":23624,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":555,"record_offset":23824,"parent_record_offset":23768,"ancestor_record_offset":23624,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":555,"record_offset":23680,"parent_record_offset":23652,"ancestor_record_offset":23624,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":555,"record_offset":23768,"parent_record_offset":23736,"ancestor_record_offset":23624,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":555,"record_offset":23736,"parent_record_offset":23708,"ancestor_record_offset":23624,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$29::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 449
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12043
// 首条位置: {"module":1608,"inlinee":73795}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$4::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1608,"record_offset":40812,"parent_record_offset":40784,"ancestor_record_offset":40784,"inlinee":15924,"source_line":1353,"archive":"_pdb/B/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$0::clone
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1608,"record_offset":40840,"parent_record_offset":40812,"ancestor_record_offset":40784,"inlinee":11988,"source_line":191,"archive":"_pdb/B/modules/01608.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::DecoratorHistoryDiff::is_empty
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 456
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7BCB
// 首条位置: {"module":555,"inlinee":31691}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":555,"record_offset":15820,"parent_record_offset":15800,"ancestor_record_offset":15800,"inlinee":4955,"source_line":605,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":555,"record_offset":15840,"parent_record_offset":15820,"ancestor_record_offset":15800,"inlinee":4956,"source_line":845,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::DecoratorHistoryDiff::cull_allow_list
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 461
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7FC4
// 首条位置: {"module":572,"inlinee":32708}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::flat_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":572,"record_offset":21496,"parent_record_offset":21044,"ancestor_record_offset":21044,"inlinee":18576,"source_line":1430,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":572,"record_offset":21068,"parent_record_offset":21044,"ancestor_record_offset":21044,"inlinee":4948,"source_line":440,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":572,"record_offset":21348,"parent_record_offset":21320,"ancestor_record_offset":21044,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":572,"record_offset":21408,"parent_record_offset":21376,"ancestor_record_offset":21044,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::FlatMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":572,"record_offset":21520,"parent_record_offset":21496,"ancestor_record_offset":21044,"inlinee":18578,"source_line":22,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":572,"record_offset":21292,"parent_record_offset":21256,"ancestor_record_offset":21044,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":572,"record_offset":21320,"parent_record_offset":21256,"ancestor_record_offset":21044,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":572,"record_offset":21376,"parent_record_offset":21320,"ancestor_record_offset":21044,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":572,"record_offset":21116,"parent_record_offset":21092,"ancestor_record_offset":21044,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":572,"record_offset":21256,"parent_record_offset":21168,"ancestor_record_offset":21044,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":572,"record_offset":21144,"parent_record_offset":21116,"ancestor_record_offset":21044,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":572,"record_offset":21228,"parent_record_offset":21168,"ancestor_record_offset":21044,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":572,"record_offset":21200,"parent_record_offset":21168,"ancestor_record_offset":21044,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":572,"record_offset":21168,"parent_record_offset":21144,"ancestor_record_offset":21044,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":572,"record_offset":21092,"parent_record_offset":21068,"ancestor_record_offset":21044,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 465
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11AAD
// 首条位置: {"module":1585,"inlinee":72365}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1585,"record_offset":232932,"parent_record_offset":232900,"ancestor_record_offset":232900,"inlinee":12667,"source_line":378,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1585,"record_offset":233224,"parent_record_offset":233196,"ancestor_record_offset":232900,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1585,"record_offset":233284,"parent_record_offset":233252,"ancestor_record_offset":232900,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1585,"record_offset":233168,"parent_record_offset":233136,"ancestor_record_offset":232900,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1585,"record_offset":233196,"parent_record_offset":233136,"ancestor_record_offset":232900,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1585,"record_offset":233252,"parent_record_offset":233196,"ancestor_record_offset":232900,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1585,"record_offset":232988,"parent_record_offset":232960,"ancestor_record_offset":232900,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":233136,"parent_record_offset":233044,"ancestor_record_offset":232900,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":233016,"parent_record_offset":232988,"ancestor_record_offset":232900,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":233108,"parent_record_offset":233044,"ancestor_record_offset":232900,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":233080,"parent_record_offset":233044,"ancestor_record_offset":232900,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":233044,"parent_record_offset":233016,"ancestor_record_offset":232900,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1585,"record_offset":232960,"parent_record_offset":232932,"ancestor_record_offset":232900,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 471
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11AAB
// 首条位置: {"module":1585,"inlinee":72363}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1585,"record_offset":235216,"parent_record_offset":235188,"ancestor_record_offset":235188,"inlinee":19952,"source_line":1125,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1585,"record_offset":235944,"parent_record_offset":235920,"ancestor_record_offset":235188,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1585,"record_offset":235452,"parent_record_offset":235428,"ancestor_record_offset":235188,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1585,"record_offset":235536,"parent_record_offset":235512,"ancestor_record_offset":235188,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1585,"record_offset":235476,"parent_record_offset":235452,"ancestor_record_offset":235188,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1585,"record_offset":236012,"parent_record_offset":235372,"ancestor_record_offset":235188,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1585,"record_offset":235684,"parent_record_offset":235656,"ancestor_record_offset":235188,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1585,"record_offset":235656,"parent_record_offset":235628,"ancestor_record_offset":235188,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1585,"record_offset":235804,"parent_record_offset":235780,"ancestor_record_offset":235188,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1585,"record_offset":235968,"parent_record_offset":235944,"ancestor_record_offset":235188,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1620,"record_offset":10140,"parent_record_offset":10112,"ancestor_record_offset":9740,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01620.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1585,"record_offset":235596,"parent_record_offset":235568,"ancestor_record_offset":235188,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1620,"record_offset":10172,"parent_record_offset":10140,"ancestor_record_offset":9740,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01620.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1585,"record_offset":235628,"parent_record_offset":235596,"ancestor_record_offset":235188,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1620,"record_offset":10112,"parent_record_offset":9916,"ancestor_record_offset":9740,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01620.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1585,"record_offset":235568,"parent_record_offset":235372,"ancestor_record_offset":235188,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1585,"record_offset":235428,"parent_record_offset":235372,"ancestor_record_offset":235188,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1585,"record_offset":236040,"parent_record_offset":235372,"ancestor_record_offset":235188,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1585,"record_offset":235512,"parent_record_offset":235372,"ancestor_record_offset":235188,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1585,"record_offset":236192,"parent_record_offset":235372,"ancestor_record_offset":235188,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1585,"record_offset":235920,"parent_record_offset":235896,"ancestor_record_offset":235188,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1585,"record_offset":235244,"parent_record_offset":235216,"ancestor_record_offset":235188,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1585,"record_offset":235272,"parent_record_offset":235244,"ancestor_record_offset":235188,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1585,"record_offset":235896,"parent_record_offset":235728,"ancestor_record_offset":235188,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1585,"record_offset":236232,"parent_record_offset":235272,"ancestor_record_offset":235188,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1620,"record_offset":10376,"parent_record_offset":10292,"ancestor_record_offset":9740,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01620.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":235836,"parent_record_offset":235752,"ancestor_record_offset":235188,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":235780,"parent_record_offset":235752,"ancestor_record_offset":235188,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":236132,"parent_record_offset":235372,"ancestor_record_offset":235188,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":235752,"parent_record_offset":235728,"ancestor_record_offset":235188,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":235340,"parent_record_offset":235308,"ancestor_record_offset":235188,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":235308,"parent_record_offset":235272,"ancestor_record_offset":235188,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":235372,"parent_record_offset":235340,"ancestor_record_offset":235188,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":236164,"parent_record_offset":235372,"ancestor_record_offset":235188,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":235728,"parent_record_offset":235372,"ancestor_record_offset":235188,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$32::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 477
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x839D
// 首条位置: {"module":599,"inlinee":33693}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$4::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":599,"record_offset":58364,"parent_record_offset":58328,"ancestor_record_offset":58328,"inlinee":15924,"source_line":1353,"archive":"_pdb/B/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$0::clone
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":599,"record_offset":58388,"parent_record_offset":58364,"ancestor_record_offset":58328,"inlinee":11988,"source_line":191,"archive":"_pdb/B/modules/00599.jsonl.gz"}
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
// 完整结构及原始类型记录: decorator_edit.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage,std::hash::random::RandomState>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage,std::hash::random::RandomState> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage>", "constructor": "tuple$", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorHistoryDiff
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryDiff", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,alloc::sync::Arc<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global> >", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "alloc::sync::Arc<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryDiff", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryDiff", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::UniqueArcUninit::new<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>", "constructor": "alloc::sync::UniqueArcUninit::new", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryDiff", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$216::deserialize<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,enum2$<serde_json::value::Value> >", "constructor": "serde::de::impls::impl$216::deserialize", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryDiff", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorHistoryEdit
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "constructor": "enum2$", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryEdit"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::new<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit> >", "constructor": "alloc::sync::Arc::new", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::Arc<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::Arc<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$216::deserialize<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,enum2$<serde_json::value::Value> >", "constructor": "serde::de::impls::impl$216::deserialize", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$54::fmt<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::impl$54::fmt", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::drain::Drain<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::vec::drain::Drain", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::drain::Drain<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::drain::Drain<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit> >", "constructor": "core::slice::iter::Iter", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$3::spec_extend<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,core::slice::iter::Iter<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit> >,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$3::spec_extend", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "core::slice::iter::Iter<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<alloc::sync::Arc<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global> >", "constructor": "core::marker::PhantomData", "arguments": ["alloc::sync::Arc<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$5::newtype_variant_seed<core::marker::PhantomData<alloc::sync::Arc<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global> > >", "constructor": "serde_json::value::de::impl$5::newtype_variant_seed", "arguments": ["core::marker::PhantomData<alloc::sync::Arc<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorHistoryEditChange
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditChange>", "constructor": "core::fmt::impl$74::fmt", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryEditChange"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorHistoryEditPlace
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditPlace>", "constructor": "core::fmt::impl$74::fmt", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryEditPlace"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorHistoryEditRemove
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditRemove>", "constructor": "core::fmt::impl$74::fmt", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryEditRemove"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorHistoryEditRotate
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditRotate>", "constructor": "core::fmt::impl$74::fmt", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryEditRotate"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorHistoryState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 31
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> >", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "constructor": "tuple$", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1>", "constructor": "core::slice::impl$0::sort_unstable_by_key::closure_env$0", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "country_core::resources::walls::decorator::DecoratorId", "country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::heapsort::heapsort<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1> >", "constructor": "core::slice::sort::unstable::heapsort::heapsort", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::ipnsort<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1> >", "constructor": "core::slice::sort::unstable::ipnsort", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,ref_mut$<serde_json::value::de::SeqDeserializer> >", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "ref_mut$<serde_json::value::de::SeqDeserializer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "constructor": "std::collections::hash::map::Keys", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >", "constructor": "core::iter::adapters::copied::Copied", "arguments": ["std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>", "constructor": "core::iter::adapters::filter::Filter", "arguments": ["core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >", "country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>", "country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$74::to_arc_slice<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0> >", "constructor": "alloc::sync::impl$74::to_arc_slice", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::rustc_entry::RustcOccupiedEntry::remove<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,alloc::alloc::Global>", "constructor": "hashbrown::rustc_entry::RustcOccupiedEntry::remove", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$13::clone<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global>", "constructor": "hashbrown::raw::impl$13::clone", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> >", "constructor": "hashbrown::map::impl$82::extend::closure_env$0", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >", "tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "tuple$<>", "country_core::resources::history::decorator_edit::impl$5::from::closure_env$0", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$44::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >", "constructor": "hashbrown::map::impl$44::fold::closure_env$0", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::walls::decorator::DecoratorInfo", "tuple$<>", "core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$44::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >,tuple$<> >", "constructor": "hashbrown::raw::RawIterRange::fold_impl", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>", "hashbrown::map::impl$44::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::with_capacity_in<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTable::with_capacity_in", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::erase<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTable::erase", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::impl$87::from_iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> >", "constructor": "std::collections::hash::map::impl$87::from_iter", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState", "std::hash::random::RandomState", "core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> >", "constructor": "core::fmt::impl$74::fmt", "arguments": ["std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::walls::decorator::DecoratorId,ref_mut$<country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0> >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::walls::decorator::DecoratorId,tuple$<> > > >", "constructor": "core::iter::adapters::copied::impl$1::try_fold", "arguments": ["std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "country_core::resources::walls::decorator::DecoratorId", "tuple$<>", "core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::walls::decorator::DecoratorId,ref_mut$<country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0> >", "enum2$<core::ops::control_flow::ControlFlow<country_core::resources::walls::decorator::DecoratorId,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::vec::Vec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::vec::Vec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global> > >", "constructor": "serde::ser::Serializer::collect_seq", "arguments": ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<alloc::vec::Vec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["alloc::vec::Vec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>", "constructor": "serde::de::impls::impl$204::deserialize::MapVisitor", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$2::deserialize_map<serde::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> >", "constructor": "serde_json::value::de::impl$2::deserialize_map", "arguments": ["serde::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> > >", "constructor": "serde_json::value::de::impl$2::deserialize_seq", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "constructor": "serde::de::impls::impl$188::deserialize::TupleVisitor", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$2::deserialize_tuple<serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >", "constructor": "serde_json::value::de::impl$2::deserialize_tuple", "arguments": ["serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >", "constructor": "ref$", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> > >", "constructor": "ref$", "arguments": ["ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<ref$<ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> > > >", "constructor": "ref$", "arguments": ["ref$<ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::to_value<ref$<ref$<ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> > > > >", "constructor": "serde_json::value::to_value", "arguments": ["ref$<ref$<ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> > >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_object<serde::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> >", "constructor": "serde_json::value::de::visit_object", "arguments": ["serde::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::quicksort::quicksort<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1> >", "constructor": "core::slice::sort::unstable::quicksort::quicksort", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryState>", "constructor": "core::fmt::impl$74::fmt", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorReplayCmd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 15
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>", "constructor": "tuple$", "arguments": ["country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState", "country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "constructor": "tuple$", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawIntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawIntoIter", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<hashbrown::raw::RawIntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["hashbrown::raw::RawIntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "constructor": "std::collections::hash::map::IntoIter", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0>", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> >", "constructor": "hashbrown::map::impl$82::extend::closure_env$0", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > >", "constructor": "core::iter::adapters::filter_map::filter_map_fold::closure_env$0", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>", "tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "tuple$<>", "country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > >", "constructor": "core::ops::function::impls::impl$3::call_mut", "arguments": ["tuple$<tuple$<>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators> >", "core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawIter::drop_elements<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> > >", "constructor": "hashbrown::raw::RawIter::drop_elements", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::impl$87::from_iter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> >", "constructor": "std::collections::hash::map::impl$87::from_iter", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>", "std::hash::random::RandomState", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2,tuple$<>,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>", "core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>", "country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2", "tuple$<>", "core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > >", "constructor": "ref_mut$", "arguments": ["core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<ref_mut$<core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > > >", "constructor": "ref_mut$", "arguments": ["ref_mut$<core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0,tuple$<>,ref_mut$<ref_mut$<core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>", "std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>", "country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0", "tuple$<>", "ref_mut$<ref_mut$<core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0> >", "constructor": "hashbrown::map::impl$82::extend", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::impl$1::fold<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2>,tuple$<>,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > >", "constructor": "core::iter::adapters::chain::impl$1::fold", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >", "core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2>", "tuple$<>", "core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 28
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<std::collections::hash::map::HashMap<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState> >", "constructor": "ref$", "arguments": ["std::collections::hash::map::HashMap<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "vectorize::serialize<ref$<std::collections::hash::map::HashMap<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState> >,enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >", "constructor": "vectorize::serialize", "arguments": ["ref$<std::collections::hash::map::HashMap<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState> >", "enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "constructor": "ref$", "arguments": ["country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "constructor": "tuple$", "arguments": ["ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >", "ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Iter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "constructor": "std::collections::hash::map::Iter", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,std::collections::hash::map::Iter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "std::collections::hash::map::Iter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,ref$<alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global> > >", "constructor": "serde::ser::Serializer::collect_seq", "arguments": ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "ref$<alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "constructor": "tuple$", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,ref_mut$<serde_json::value::de::SeqDeserializer> >", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "ref_mut$<serde_json::value::de::SeqDeserializer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$188::deserialize::TupleVisitor<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "constructor": "serde::de::impls::impl$188::deserialize::TupleVisitor", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impl$1::fmt<serde::de::impls::impl$188::deserialize::TupleVisitor<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "constructor": "serde::de::impl$1::fmt", "arguments": ["serde::de::impls::impl$188::deserialize::TupleVisitor<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "constructor": "std::collections::hash::map::Values", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>", "country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$74::to_arc_slice<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0> >", "constructor": "alloc::sync::impl$74::to_arc_slice", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTable", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >", "constructor": "ref_mut$", "arguments": ["hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<usize,ref_mut$<hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > >", "constructor": "tuple$", "arguments": ["usize", "ref_mut$<hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$16::clone_from_impl::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>", "constructor": "hashbrown::raw::impl$16::clone_from_impl::closure_env$0", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::scopeguard::ScopeGuard<tuple$<usize,ref_mut$<hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > >,hashbrown::raw::impl$16::clone_from_impl::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >", "constructor": "hashbrown::scopeguard::ScopeGuard", "arguments": ["tuple$<usize,ref_mut$<hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > >", "hashbrown::raw::impl$16::clone_from_impl::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<hashbrown::scopeguard::ScopeGuard<tuple$<usize,ref_mut$<hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > >,hashbrown::raw::impl$16::clone_from_impl::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["hashbrown::scopeguard::ScopeGuard<tuple$<usize,ref_mut$<hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > >,hashbrown::raw::impl$16::clone_from_impl::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$13::clone<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>", "constructor": "hashbrown::raw::impl$13::clone", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::clear<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTable::clear", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::impl$87::from_iter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >", "constructor": "std::collections::hash::map::impl$87::from_iter", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState", "alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0> >", "constructor": "hashbrown::map::impl$82::extend", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >", "constructor": "hashbrown::map::impl$82::extend::closure_env$0", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState", "alloc::alloc::Global", "alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$5::fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > > >", "constructor": "alloc::vec::into_iter::impl$5::fold", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> > >", "constructor": "serde_json::value::de::impl$2::deserialize_seq", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$2::deserialize_tuple<serde::de::impls::impl$188::deserialize::TupleVisitor<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "constructor": "serde_json::value::de::impl$2::deserialize_tuple", "arguments": ["serde::de::impls::impl$188::deserialize::TupleVisitor<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> > >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde::de::impls::impl$188::deserialize::TupleVisitor<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde::de::impls::impl$188::deserialize::TupleVisitor<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: decorator_edit.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [资源/历史 JSON schema，不是内存布局] DecoratorV5 history serialized observations
// 全部实际键和数据形态: {"/commands/DecoratorV5": {"observed_types": {"object": 345}, "object_instances": 345, "field_occurrences": {"Place": 144, "Change": 190, "Remove": 11}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/7/edits/0/DecoratorV5"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/8/edits/0/DecoratorV5"}]}, "/commands/DecoratorV5/Change": {"observed_types": {"object": 190}, "object_instances": 190, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/9/edits/0/DecoratorV5/Change"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/10/edits/0/DecoratorV5/Change"}]}, "/commands/DecoratorV5/Place": {"observed_types": {"object": 144}, "object_instances": 144, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/7/edits/0/DecoratorV5/Place"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/8/edits/0/DecoratorV5/Place"}]}, "/commands/DecoratorV5/Remove": {"observed_types": {"object": 11}, "object_instances": 11, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/42/edits/0/DecoratorV5/Remove"}, {"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/43/edits/0/DecoratorV5/Remove"}]}}
// offsets / Rust numeric types / missing defaults = unknown。
// [已有模块证据关联；不自动成为字段事实] evidence/building-wall-history/B-rectangle-cancel-helpers/schema-names.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "status": "evidence_association; original field ownership unknown"}, {"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState", "status": "evidence_association; original field ownership unknown"}, {"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::history::decorator_edit::DecoratorHistoryState", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/pdb-structure/type-layout-coverage.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::history::decorator_edit::DecoratorHistoryEdit", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: decorator_edit.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 19; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

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
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::DecoratorHistoryDiff::cull_allow_list
// 信息 ID: b141383c93d5687efade1f59
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=572, inline_record=21044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::DecoratorHistoryDiff::is_empty
// 信息 ID: f45af3ef8cceaba8315c712e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::is_decorator_history_diff_empty; module=555, inline_record=15800; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::History::add_decorator_diff; module=555, inline_record=25420; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::History::finish_input_recording; module=555, inline_record=34892; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=572, inline_record=21780; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=586, inline_record=47736; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::load_game::parse_saved_game_data; module=1608, inline_record=42040; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::load_game::parse_saved_game_data; module=1608, inline_record=45604; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize
// 信息 ID: 0b528b2a66f0389aca94f7ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impls::impl$216::deserialize<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,enum2$<serde_json::value::Value> >; module=593, inline_record=41184; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<serde::de::impls::impl$188::deserialize::TupleVisitor<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >; module=1630, inline_record=39492; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >; module=1630, inline_record=170984; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<serde::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> >; module=1630, inline_record=251100; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1729, inline_record=37088; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1729, inline_record=37216; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1729, inline_record=37344; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1729, inline_record=37516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 266d84ffba15c1d320736bc8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=209156; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=228968; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=394236; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: b14a069e64bbcd691ff1cbec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Field> > @ 0x15d18e6 -> 0x17c0890 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor> @ 0x15ca797 -> 0x17c1170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: c1d7352142980bf66c35e435
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=209032; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=228788; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=239368; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=394084; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=399488; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Field> >; module=1630, inline_record=471944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 056e88e87e9a11ba0a16c986
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1729, inline_record=452; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1729, inline_record=1324; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1729, inline_record=1544; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1729, inline_record=1764; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1729, inline_record=2416; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1729, inline_record=2636; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1729, inline_record=2856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 613f0986ef6249b2a081dfea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1729, inline_record=36860; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 12759a86dc2fdcf5e708ae48
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=207504; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=227240; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=237856; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=392548; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=397932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
// 信息 ID: 8fa56d3043e90b78bfb2e6e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=210336; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: c1e9ed20488ff77eb4d82ee8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=23360; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=66120; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=90908; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=119724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
// 信息 ID: 308180aa2016fac8d9e71dc7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=120012; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 信息 ID: bcf1cd795d4827e62f16f5cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::resources::history::decorator_edit::_::impl$0::deserialize -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impls::impl$216::deserialize<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,enum2$<serde_json::value::Value> > @ 0xb5d50c -> 0x17bff80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::serialize
// 信息 ID: e02865fdb500580d9e0a7e79
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=17424; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1729, inline_record=34488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::serialize::impl$0::serialize
// 信息 ID: 267a13fc3446434014e7f8cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::history::decorator_edit::_::impl$0::serialize::__SerializeWith>; module=191, inline_record=208948; 不是独立调用边。
// [PDB 内联宿主] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::history::decorator_edit::_::impl$0::serialize::__SerializeWith>; module=1627, inline_record=184704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: 70e87fd8d76cbdefe09f6ed5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::history::decorator_edit::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,alloc::sync::Arc<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global> > @ 0x4c16a0 -> 0x4f46e0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> > @ 0x4c1c16 -> 0x4f4c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> > @ 0x4c1cae -> 0x4f4c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,ref$<alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global> > > @ 0x4b71a1 -> 0x4f5000 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,ref$<alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global> > > @ 0x4b7287 -> 0x4f5000 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> > @ 0x1586e96 -> 0x17c0960 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> > @ 0x1586f2e -> 0x17c0960 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,alloc::sync::Arc<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global> > @ 0x1583ba0 -> 0x17c13b0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,ref$<alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global> > > @ 0x1582291 -> 0x17c1490 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,ref$<alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global> > > @ 0x1582377 -> 0x17c1490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: aca2a3c708c74133123a0950
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::decorator_edit::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::to_value<ref$<ref$<ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> > > > > @ 0x15a46d0 -> 0x17c0cf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::convert_before_after_states_to_changes
// 信息 ID: 08813b96ed5bceeb37af2791
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> @ 0x17be033 -> 0xb3fab0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$11::fmt
// 信息 ID: ded646aab9c5097ca7beb5d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditChange>; module=1729, inline_record=3992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$13::fmt
// 信息 ID: eba214e76a6b86850ad76da0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditPlace>; module=1729, inline_record=3596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$15::fmt
// 信息 ID: fdb1875b7019b10858ab6527
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditRemove>; module=1729, inline_record=4600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$18::fmt
// 信息 ID: 9702cdbfc09c2a1e14937e0d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditRotate>; module=1729, inline_record=5124; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$19::clone
// 信息 ID: fb8fb20ca324a2e035b08313
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::raw::impl$13::clone<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global>; module=599, inline_record=56296; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::decorator_storage::DecoratorStorage::restore_decorators<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>; module=1618, inline_record=60956; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>; module=1729, inline_record=10636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$1::from
// 信息 ID: b3ef09128b966004b03dd8f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$44::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >,tuple$<> > @ 0xb763b7 -> 0xb3f930 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$0> > @ 0x12ff879 -> 0xb3f930 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$0> > @ 0x12ffa2b -> 0xb3f930 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::decorator_storage::DecoratorStorage::restore_decorators<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> @ 0x155c228 -> 0xb3f930 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$20::fmt
// 信息 ID: 8a5c2c13d5dd51fc8104f958
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryState>; module=1729, inline_record=4808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$22::eq
// 信息 ID: b28021d7c71d6fda6aea4fb7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::map::impl$82::extend<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0> >; module=1613, inline_record=28088; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$23::default
// 信息 ID: bc6e5568fe6eb5762efe0c06
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::decorator_archivist::DecoratorArchivist::extract_before_after_impl; module=584, inline_record=29176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$25::fmt
// 信息 ID: 21ed05d7adc3550516e95b2e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$27::default
// 信息 ID: 4a4cc5e1b4c2eb28d46d0df9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::History::add; module=555, inline_record=23624; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::History::add_decorator_diff; module=555, inline_record=25968; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::History::start_input_recording; module=555, inline_record=32032; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::History::try_start_input_recording; module=555, inline_record=33548; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::impl$16::deserialize::impl$0::visit_seq<ref_mut$<serde_json::value::de::SeqDeserializer> >; module=586, inline_record=6924; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::impl$16::deserialize::impl$0::visit_map<ref_mut$<serde_json::value::de::MapDeserializer> >; module=586, inline_record=11108; 不是独立调用边。
// [PDB 内联宿主] system_tree::convert_preplaced_trees::convert_preplaced_trees; module=3666, inline_record=7100; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$29::clone
// 信息 ID: caf1b33f6564aa321227392a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::load_game::parse_saved_game_data; module=1608, inline_record=40784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$2::from
// 信息 ID: 31ca7f3586e81224f89e93f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$44::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >,tuple$<> >; module=599, inline_record=204716; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::decorator_storage::DecoratorStorage::restore_decorators<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>; module=1618, inline_record=53700; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$32::clone
// 信息 ID: 8c2e709eeb7605c9d34b20a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::raw::impl$13::clone<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>; module=599, inline_record=58328; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$3::from
// 信息 ID: f5a9f282a00afad1765e7ed1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::decorator_storage::DecoratorStorage::restore_snapshot; module=540, inline_record=101548; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::decorator_storage::DecoratorStorage::restore_decorators<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>; module=1618, inline_record=65004; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>; module=1729, inline_record=9744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$4::from
// 信息 ID: 073fbbe2eaf63428144ee7b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>; module=1729, inline_record=9664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$5::from
// 信息 ID: 8aca244c08598e746bac7bb6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > >; module=584, inline_record=2888; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::decorator_archivist::DecoratorArchivist::extract_before_after_impl; module=584, inline_record=28652; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2,tuple$<>,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > >; module=1610, inline_record=4984; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$5::from::closure$0
// 信息 ID: 0450e76c4dd7a8d5e834fef7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::decorator_edit::impl$5::from", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$5::from::closure_env$0 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$44::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$5::from::closure_env$0 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$44::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$5::from::closure_env$0 <- std::collections::hash::map::impl$87::from_iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$44::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >,tuple$<> >; module=599, inline_record=204612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0
// 信息 ID: 7f2e7b62d3350dce0ec873b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::decorator_edit::impl$7::cull_allow_list", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0 <- alloc::sync::impl$74::to_arc_slice<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0> >; module=1585, inline_record=232900; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0> >; module=1585, inline_record=237196; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure$0
// 信息 ID: c1aa04fe4b93b9099a2faad9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::decorator_edit::impl$7::cull_allow_list", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0 <- alloc::sync::impl$74::to_arc_slice<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0 <- core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::walls::decorator::DecoratorId,ref_mut$<country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0> >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::walls::decorator::DecoratorId,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0> >; module=1585, inline_record=235188; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::walls::decorator::DecoratorId,ref_mut$<country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0> >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::walls::decorator::DecoratorId,tuple$<> > > >; module=1620, inline_record=9740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$8::fmt
// 信息 ID: b63af92807c37ff10cbd6122
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::sync::impl$54::fmt<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>; module=593, inline_record=56428; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$9::clone
// 信息 ID: eae7cdab19d6fa985f34d3df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::backup::DecoratorBackup::clear; module=470, inline_record=1500; 不是独立调用边。
// [PDB 内联宿主] system_decorator::backup::create_or_restore_decorator_backup; module=470, inline_record=5244; 不是独立调用边。
// [PDB 内联宿主] system_decorator::backup::create_or_restore_decorator_backup; module=470, inline_record=8064; 不是独立调用边。
// [PDB 内联宿主] system_decorator::backup::create_or_restore_decorator_backup; module=470, inline_record=11864; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_extend::impl$3::spec_extend<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,core::slice::iter::Iter<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit> >,alloc::alloc::Global>; module=1503, inline_record=1204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::is_default
// 信息 ID: 68080ad3d5b981d6fd135c13
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 21 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=19224; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=19316; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=19408; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=19500; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=19592; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=19684; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=19776; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1729, inline_record=41728; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::is_default_chimney_smoke
// 信息 ID: 8776647afb68aaca08c9b612
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 31 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x3b865d -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x4f4c66 -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x4f4c85 -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x4f4dde -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x4f4e67 -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::mei_post_update @ 0x688817 -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::maintain_plaster @ 0x75c9d8 -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve @ 0x76c2f2 -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a6b80 -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a68e6 -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a696b -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a68f4 -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a55fc -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x7ac0e8 -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x7ac12f -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x7ac1aa -> 0x8c5680 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::is_default_lantern_brightness
// 信息 ID: 4fa399ed398243b74f1fd9b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1729, inline_record=43168; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=1729, inline_record=45028; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::is_default_stair_width
// 信息 ID: df854dc03c5344b10a9aaab3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x4f4c79 -> 0xb3f910 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x4f4c93 -> 0xb3f910 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x4f4e38 -> 0xb3f910 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x4f4e9b -> 0xb3f910 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1729, inline_record=43224; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=1729, inline_record=45056; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>
// 信息 ID: 6dba7a1c2f819d775aa26692
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::decorator_edit::send_state_diff_events -> ["country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::walls::decorator_storage::DecoratorStorage::restore_decorators<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> @ 0x155d13f -> 0x17bdae0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
