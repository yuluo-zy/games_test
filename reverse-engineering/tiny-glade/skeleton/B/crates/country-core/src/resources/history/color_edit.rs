// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/history/color_edit.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 5e8ece64c352ebeeff292708ac9ca29ba6879a6dd0f5a567e9abdb5f1501da75
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: color_edit.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 10, 560, 593, 1630, 1729
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1729,"record_offset":30132,"rva":24901792,"va":5393610912}
// 编译实例 RVAs: 0x17bf8a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f9e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18064195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393610912,"raw_hex":"e843a31301","size":5,"soft_fail":false,"target_va":5411675680,"va":5393611480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18235943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393610912,"raw_hex":"e827421601","size":5,"soft_fail":false,"target_va":5411847152,"va":5393611204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1729,"record_offset":30332,"parent_record_offset":30132,"ancestor_record_offset":30132,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1729,"record_offset":30352,"parent_record_offset":30332,"ancestor_record_offset":30132,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1729,"record_offset":30372,"parent_record_offset":30352,"ancestor_record_offset":30132,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1729,"record_offset":30392,"parent_record_offset":30372,"ancestor_record_offset":30132,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12262
// 首条位置: {"module":1630,"inlinee":74338}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::value::impl$26::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":484136,"parent_record_offset":484112,"ancestor_record_offset":484112,"inlinee":74331,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":484384,"parent_record_offset":484360,"ancestor_record_offset":484112,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":484360,"parent_record_offset":484308,"ancestor_record_offset":484112,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":484332,"parent_record_offset":484308,"ancestor_record_offset":484112,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":484308,"parent_record_offset":484284,"ancestor_record_offset":484112,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":484284,"parent_record_offset":484264,"ancestor_record_offset":484112,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":484224,"parent_record_offset":484192,"ancestor_record_offset":484112,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":484192,"parent_record_offset":484164,"ancestor_record_offset":484112,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$26::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1630,"record_offset":484164,"parent_record_offset":484136,"ancestor_record_offset":484112,"inlinee":74332,"source_line":615,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13013
// 首条位置: {"module":1729,"inlinee":77843}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1729,"record_offset":26928,"parent_record_offset":26868,"ancestor_record_offset":26868,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1729,"record_offset":26964,"parent_record_offset":26868,"ancestor_record_offset":26868,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1729,"record_offset":26900,"parent_record_offset":26868,"ancestor_record_offset":26868,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12319
// 首条位置: {"module":1630,"inlinee":74521}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":718152,"parent_record_offset":718120,"ancestor_record_offset":718120,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":718176,"parent_record_offset":718152,"ancestor_record_offset":718120,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":718200,"parent_record_offset":718176,"ancestor_record_offset":718120,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":718224,"parent_record_offset":718200,"ancestor_record_offset":718120,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$10::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x122F7
// 首条位置: {"module":1630,"inlinee":74487}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":676000,"parent_record_offset":675972,"ancestor_record_offset":675972,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":676768,"parent_record_offset":676744,"ancestor_record_offset":675972,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":676744,"parent_record_offset":676664,"ancestor_record_offset":675972,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":676704,"parent_record_offset":676664,"ancestor_record_offset":675972,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":676664,"parent_record_offset":676628,"ancestor_record_offset":675972,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":676628,"parent_record_offset":676592,"ancestor_record_offset":675972,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":676200,"parent_record_offset":676176,"ancestor_record_offset":675972,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":676520,"parent_record_offset":676096,"ancestor_record_offset":675972,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":676224,"parent_record_offset":676200,"ancestor_record_offset":675972,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":676248,"parent_record_offset":676224,"ancestor_record_offset":675972,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":676176,"parent_record_offset":676144,"ancestor_record_offset":675972,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":676144,"parent_record_offset":676096,"ancestor_record_offset":675972,"inlinee":74489,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":676096,"parent_record_offset":676064,"ancestor_record_offset":675972,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":676032,"parent_record_offset":676000,"ancestor_record_offset":675972,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":676064,"parent_record_offset":676032,"ancestor_record_offset":675972,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$11::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x122F5
// 首条位置: {"module":1630,"inlinee":74485}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":677360,"parent_record_offset":674420,"ancestor_record_offset":674420,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":677636,"parent_record_offset":674420,"ancestor_record_offset":674420,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":678368,"parent_record_offset":674420,"ancestor_record_offset":674420,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":674460,"parent_record_offset":674420,"ancestor_record_offset":674420,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":677084,"parent_record_offset":674420,"ancestor_record_offset":674420,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":675276,"parent_record_offset":675252,"ancestor_record_offset":674420,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":675252,"parent_record_offset":675196,"ancestor_record_offset":674420,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":674616,"parent_record_offset":674588,"ancestor_record_offset":674420,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":674672,"parent_record_offset":674644,"ancestor_record_offset":674420,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":674696,"parent_record_offset":674672,"ancestor_record_offset":674420,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":675632,"parent_record_offset":675608,"ancestor_record_offset":674420,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":675224,"parent_record_offset":675196,"ancestor_record_offset":674420,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":675196,"parent_record_offset":675172,"ancestor_record_offset":674420,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":675680,"parent_record_offset":675656,"ancestor_record_offset":674420,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":675656,"parent_record_offset":675632,"ancestor_record_offset":674420,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":675172,"parent_record_offset":675148,"ancestor_record_offset":674420,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":675608,"parent_record_offset":675584,"ancestor_record_offset":674420,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":675584,"parent_record_offset":675328,"ancestor_record_offset":674420,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":676200,"parent_record_offset":676176,"ancestor_record_offset":674420,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":677200,"parent_record_offset":677176,"ancestor_record_offset":674420,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":675540,"parent_record_offset":675512,"ancestor_record_offset":674420,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":674644,"parent_record_offset":674616,"ancestor_record_offset":674420,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":677176,"parent_record_offset":677140,"ancestor_record_offset":674420,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":674812,"parent_record_offset":674724,"ancestor_record_offset":674420,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":675040,"parent_record_offset":674532,"ancestor_record_offset":674420,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":674836,"parent_record_offset":674812,"ancestor_record_offset":674420,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":677256,"parent_record_offset":677200,"ancestor_record_offset":674420,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":676224,"parent_record_offset":676200,"ancestor_record_offset":674420,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":676248,"parent_record_offset":676224,"ancestor_record_offset":674420,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":674724,"parent_record_offset":674696,"ancestor_record_offset":674420,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":674780,"parent_record_offset":674756,"ancestor_record_offset":674420,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":674756,"parent_record_offset":674724,"ancestor_record_offset":674420,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":676176,"parent_record_offset":676144,"ancestor_record_offset":674420,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$10::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":675972,"parent_record_offset":675940,"ancestor_record_offset":674420,"inlinee":74487,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":676144,"parent_record_offset":676096,"ancestor_record_offset":674420,"inlinee":74489,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":1630,"record_offset":678392,"parent_record_offset":678368,"ancestor_record_offset":674420,"inlinee":29592,"source_line":336,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1630,"record_offset":678240,"parent_record_offset":678216,"ancestor_record_offset":674420,"inlinee":32780,"source_line":69,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1630,"record_offset":678288,"parent_record_offset":678264,"ancestor_record_offset":674420,"inlinee":32782,"source_line":69,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":678440,"parent_record_offset":678416,"ancestor_record_offset":674420,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":678416,"parent_record_offset":678392,"ancestor_record_offset":674420,"inlinee":6675,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":678528,"parent_record_offset":678504,"ancestor_record_offset":674420,"inlinee":6705,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":674496,"parent_record_offset":674460,"ancestor_record_offset":674420,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":677112,"parent_record_offset":677084,"ancestor_record_offset":674420,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":676096,"parent_record_offset":676064,"ancestor_record_offset":674420,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":677316,"parent_record_offset":677292,"ancestor_record_offset":674420,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":675940,"parent_record_offset":674532,"ancestor_record_offset":674420,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$75::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":678312,"parent_record_offset":678288,"ancestor_record_offset":674420,"inlinee":6539,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":674588,"parent_record_offset":674532,"ancestor_record_offset":674420,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":676032,"parent_record_offset":676000,"ancestor_record_offset":674420,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":676000,"parent_record_offset":675972,"ancestor_record_offset":674420,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":676064,"parent_record_offset":676032,"ancestor_record_offset":674420,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":678264,"parent_record_offset":678240,"ancestor_record_offset":674420,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":674532,"parent_record_offset":674496,"ancestor_record_offset":674420,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":677140,"parent_record_offset":677112,"ancestor_record_offset":674420,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$12::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12309
// 首条位置: {"module":1630,"inlinee":74505}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":695784,"parent_record_offset":695752,"ancestor_record_offset":695752,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":695808,"parent_record_offset":695784,"ancestor_record_offset":695752,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":695832,"parent_record_offset":695808,"ancestor_record_offset":695752,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":695856,"parent_record_offset":695832,"ancestor_record_offset":695752,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$13::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12307
// 首条位置: {"module":1630,"inlinee":74503}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":695608,"parent_record_offset":695580,"ancestor_record_offset":695580,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":696376,"parent_record_offset":696352,"ancestor_record_offset":695580,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":696352,"parent_record_offset":696272,"ancestor_record_offset":695580,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":696312,"parent_record_offset":696272,"ancestor_record_offset":695580,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":696272,"parent_record_offset":696236,"ancestor_record_offset":695580,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":696236,"parent_record_offset":696200,"ancestor_record_offset":695580,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":695808,"parent_record_offset":695784,"ancestor_record_offset":695580,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":696128,"parent_record_offset":695704,"ancestor_record_offset":695580,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":695832,"parent_record_offset":695808,"ancestor_record_offset":695580,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":695856,"parent_record_offset":695832,"ancestor_record_offset":695580,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":695784,"parent_record_offset":695752,"ancestor_record_offset":695580,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$12::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":695752,"parent_record_offset":695704,"ancestor_record_offset":695580,"inlinee":74505,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":695704,"parent_record_offset":695672,"ancestor_record_offset":695580,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":695640,"parent_record_offset":695608,"ancestor_record_offset":695580,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":695672,"parent_record_offset":695640,"ancestor_record_offset":695580,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$14::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12305
// 首条位置: {"module":1630,"inlinee":74501}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":696968,"parent_record_offset":694028,"ancestor_record_offset":694028,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":698208,"parent_record_offset":694028,"ancestor_record_offset":694028,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":694068,"parent_record_offset":694028,"ancestor_record_offset":694028,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":696692,"parent_record_offset":694028,"ancestor_record_offset":694028,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":694884,"parent_record_offset":694860,"ancestor_record_offset":694028,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":694860,"parent_record_offset":694804,"ancestor_record_offset":694028,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":694224,"parent_record_offset":694196,"ancestor_record_offset":694028,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":694280,"parent_record_offset":694252,"ancestor_record_offset":694028,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":694304,"parent_record_offset":694280,"ancestor_record_offset":694028,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":695240,"parent_record_offset":695216,"ancestor_record_offset":694028,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":694832,"parent_record_offset":694804,"ancestor_record_offset":694028,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":694804,"parent_record_offset":694780,"ancestor_record_offset":694028,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":695288,"parent_record_offset":695264,"ancestor_record_offset":694028,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":695264,"parent_record_offset":695240,"ancestor_record_offset":694028,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":694780,"parent_record_offset":694756,"ancestor_record_offset":694028,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":695216,"parent_record_offset":695192,"ancestor_record_offset":694028,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":695192,"parent_record_offset":694936,"ancestor_record_offset":694028,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":695808,"parent_record_offset":695784,"ancestor_record_offset":694028,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":696808,"parent_record_offset":696784,"ancestor_record_offset":694028,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":695148,"parent_record_offset":695120,"ancestor_record_offset":694028,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":694252,"parent_record_offset":694224,"ancestor_record_offset":694028,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":696784,"parent_record_offset":696748,"ancestor_record_offset":694028,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":694420,"parent_record_offset":694332,"ancestor_record_offset":694028,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":694648,"parent_record_offset":694140,"ancestor_record_offset":694028,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":694444,"parent_record_offset":694420,"ancestor_record_offset":694028,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":696864,"parent_record_offset":696808,"ancestor_record_offset":694028,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":697332,"parent_record_offset":697280,"ancestor_record_offset":694028,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":695832,"parent_record_offset":695808,"ancestor_record_offset":694028,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":695856,"parent_record_offset":695832,"ancestor_record_offset":694028,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":694332,"parent_record_offset":694304,"ancestor_record_offset":694028,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":694388,"parent_record_offset":694364,"ancestor_record_offset":694028,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":694364,"parent_record_offset":694332,"ancestor_record_offset":694028,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":695784,"parent_record_offset":695752,"ancestor_record_offset":694028,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":697232,"parent_record_offset":697208,"ancestor_record_offset":694028,"inlinee":28073,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":697280,"parent_record_offset":697256,"ancestor_record_offset":694028,"inlinee":28014,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$12::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":695752,"parent_record_offset":695704,"ancestor_record_offset":694028,"inlinee":74505,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$13::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":695580,"parent_record_offset":695548,"ancestor_record_offset":694028,"inlinee":74503,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":698048,"parent_record_offset":698024,"ancestor_record_offset":694028,"inlinee":31381,"source_line":591,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":698096,"parent_record_offset":698072,"ancestor_record_offset":694028,"inlinee":31375,"source_line":155,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":698280,"parent_record_offset":698256,"ancestor_record_offset":694028,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":698256,"parent_record_offset":698232,"ancestor_record_offset":694028,"inlinee":6705,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":694104,"parent_record_offset":694068,"ancestor_record_offset":694028,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":696720,"parent_record_offset":696692,"ancestor_record_offset":694028,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":695704,"parent_record_offset":695672,"ancestor_record_offset":694028,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":696924,"parent_record_offset":696900,"ancestor_record_offset":694028,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":695548,"parent_record_offset":694140,"ancestor_record_offset":694028,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":697304,"parent_record_offset":697280,"ancestor_record_offset":694028,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":694196,"parent_record_offset":694140,"ancestor_record_offset":694028,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":695640,"parent_record_offset":695608,"ancestor_record_offset":694028,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":695608,"parent_record_offset":695580,"ancestor_record_offset":694028,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":695672,"parent_record_offset":695640,"ancestor_record_offset":694028,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":697256,"parent_record_offset":697232,"ancestor_record_offset":694028,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":694140,"parent_record_offset":694104,"ancestor_record_offset":694028,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":696748,"parent_record_offset":696720,"ancestor_record_offset":694028,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$15::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12311
// 首条位置: {"module":1630,"inlinee":74513}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":708476,"parent_record_offset":708444,"ancestor_record_offset":708444,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":708500,"parent_record_offset":708476,"ancestor_record_offset":708444,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":708524,"parent_record_offset":708500,"ancestor_record_offset":708444,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":708548,"parent_record_offset":708524,"ancestor_record_offset":708444,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$16::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1230F
// 首条位置: {"module":1630,"inlinee":74511}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":708300,"parent_record_offset":708272,"ancestor_record_offset":708272,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":709068,"parent_record_offset":709044,"ancestor_record_offset":708272,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":709044,"parent_record_offset":708964,"ancestor_record_offset":708272,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":709004,"parent_record_offset":708964,"ancestor_record_offset":708272,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":708964,"parent_record_offset":708928,"ancestor_record_offset":708272,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":708928,"parent_record_offset":708892,"ancestor_record_offset":708272,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":708500,"parent_record_offset":708476,"ancestor_record_offset":708272,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":708820,"parent_record_offset":708396,"ancestor_record_offset":708272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":708524,"parent_record_offset":708500,"ancestor_record_offset":708272,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":708548,"parent_record_offset":708524,"ancestor_record_offset":708272,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":708476,"parent_record_offset":708444,"ancestor_record_offset":708272,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$15::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":708444,"parent_record_offset":708396,"ancestor_record_offset":708272,"inlinee":74513,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":708396,"parent_record_offset":708364,"ancestor_record_offset":708272,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":708332,"parent_record_offset":708300,"ancestor_record_offset":708272,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":708364,"parent_record_offset":708332,"ancestor_record_offset":708272,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$17::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1230D
// 首条位置: {"module":1630,"inlinee":74509}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":709660,"parent_record_offset":706720,"ancestor_record_offset":706720,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":710900,"parent_record_offset":706720,"ancestor_record_offset":706720,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":706760,"parent_record_offset":706720,"ancestor_record_offset":706720,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":709384,"parent_record_offset":706720,"ancestor_record_offset":706720,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":707576,"parent_record_offset":707552,"ancestor_record_offset":706720,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":707552,"parent_record_offset":707496,"ancestor_record_offset":706720,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":706916,"parent_record_offset":706888,"ancestor_record_offset":706720,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":706972,"parent_record_offset":706944,"ancestor_record_offset":706720,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":706996,"parent_record_offset":706972,"ancestor_record_offset":706720,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":707932,"parent_record_offset":707908,"ancestor_record_offset":706720,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":707524,"parent_record_offset":707496,"ancestor_record_offset":706720,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":707496,"parent_record_offset":707472,"ancestor_record_offset":706720,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":707980,"parent_record_offset":707956,"ancestor_record_offset":706720,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":707956,"parent_record_offset":707932,"ancestor_record_offset":706720,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":707472,"parent_record_offset":707448,"ancestor_record_offset":706720,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":707908,"parent_record_offset":707884,"ancestor_record_offset":706720,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":707884,"parent_record_offset":707628,"ancestor_record_offset":706720,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":708500,"parent_record_offset":708476,"ancestor_record_offset":706720,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":709500,"parent_record_offset":709476,"ancestor_record_offset":706720,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":707840,"parent_record_offset":707812,"ancestor_record_offset":706720,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":706944,"parent_record_offset":706916,"ancestor_record_offset":706720,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":709476,"parent_record_offset":709440,"ancestor_record_offset":706720,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":707112,"parent_record_offset":707024,"ancestor_record_offset":706720,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":707340,"parent_record_offset":706832,"ancestor_record_offset":706720,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":707136,"parent_record_offset":707112,"ancestor_record_offset":706720,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":709556,"parent_record_offset":709500,"ancestor_record_offset":706720,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":710024,"parent_record_offset":709972,"ancestor_record_offset":706720,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":708524,"parent_record_offset":708500,"ancestor_record_offset":706720,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":708548,"parent_record_offset":708524,"ancestor_record_offset":706720,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":707024,"parent_record_offset":706996,"ancestor_record_offset":706720,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":707080,"parent_record_offset":707056,"ancestor_record_offset":706720,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":707056,"parent_record_offset":707024,"ancestor_record_offset":706720,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":708476,"parent_record_offset":708444,"ancestor_record_offset":706720,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":709924,"parent_record_offset":709900,"ancestor_record_offset":706720,"inlinee":28073,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":709972,"parent_record_offset":709948,"ancestor_record_offset":706720,"inlinee":28014,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$15::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":708444,"parent_record_offset":708396,"ancestor_record_offset":706720,"inlinee":74513,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$16::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":708272,"parent_record_offset":708240,"ancestor_record_offset":706720,"inlinee":74511,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":710740,"parent_record_offset":710716,"ancestor_record_offset":706720,"inlinee":31381,"source_line":591,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":710788,"parent_record_offset":710764,"ancestor_record_offset":706720,"inlinee":31375,"source_line":155,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":710972,"parent_record_offset":710948,"ancestor_record_offset":706720,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":710948,"parent_record_offset":710924,"ancestor_record_offset":706720,"inlinee":6705,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":706796,"parent_record_offset":706760,"ancestor_record_offset":706720,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":709412,"parent_record_offset":709384,"ancestor_record_offset":706720,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":708396,"parent_record_offset":708364,"ancestor_record_offset":706720,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":709616,"parent_record_offset":709592,"ancestor_record_offset":706720,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":708240,"parent_record_offset":706832,"ancestor_record_offset":706720,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":709996,"parent_record_offset":709972,"ancestor_record_offset":706720,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":706888,"parent_record_offset":706832,"ancestor_record_offset":706720,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":708332,"parent_record_offset":708300,"ancestor_record_offset":706720,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":708300,"parent_record_offset":708272,"ancestor_record_offset":706720,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":708364,"parent_record_offset":708332,"ancestor_record_offset":706720,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":709948,"parent_record_offset":709924,"ancestor_record_offset":706720,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":706832,"parent_record_offset":706796,"ancestor_record_offset":706720,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":709440,"parent_record_offset":709412,"ancestor_record_offset":706720,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$18::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12322
// 首条位置: {"module":1630,"inlinee":74530}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":726716,"parent_record_offset":726684,"ancestor_record_offset":726684,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":726740,"parent_record_offset":726716,"ancestor_record_offset":726684,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":726764,"parent_record_offset":726740,"ancestor_record_offset":726684,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":726788,"parent_record_offset":726764,"ancestor_record_offset":726684,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$19::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12320
// 首条位置: {"module":1630,"inlinee":74528}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":726540,"parent_record_offset":726512,"ancestor_record_offset":726512,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":727308,"parent_record_offset":727284,"ancestor_record_offset":726512,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":727284,"parent_record_offset":727204,"ancestor_record_offset":726512,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":727244,"parent_record_offset":727204,"ancestor_record_offset":726512,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":727204,"parent_record_offset":727168,"ancestor_record_offset":726512,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":727168,"parent_record_offset":727132,"ancestor_record_offset":726512,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":726740,"parent_record_offset":726716,"ancestor_record_offset":726512,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":727060,"parent_record_offset":726636,"ancestor_record_offset":726512,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":726764,"parent_record_offset":726740,"ancestor_record_offset":726512,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":726788,"parent_record_offset":726764,"ancestor_record_offset":726512,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":726716,"parent_record_offset":726684,"ancestor_record_offset":726512,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$18::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":726684,"parent_record_offset":726636,"ancestor_record_offset":726512,"inlinee":74530,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":726636,"parent_record_offset":726604,"ancestor_record_offset":726512,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":726572,"parent_record_offset":726540,"ancestor_record_offset":726512,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":726604,"parent_record_offset":726572,"ancestor_record_offset":726512,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12317
// 首条位置: {"module":1630,"inlinee":74519}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":717976,"parent_record_offset":717948,"ancestor_record_offset":717948,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":718744,"parent_record_offset":718720,"ancestor_record_offset":717948,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":718720,"parent_record_offset":718640,"ancestor_record_offset":717948,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":718680,"parent_record_offset":718640,"ancestor_record_offset":717948,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":718640,"parent_record_offset":718604,"ancestor_record_offset":717948,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":718604,"parent_record_offset":718568,"ancestor_record_offset":717948,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":718176,"parent_record_offset":718152,"ancestor_record_offset":717948,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":718496,"parent_record_offset":718072,"ancestor_record_offset":717948,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":718200,"parent_record_offset":718176,"ancestor_record_offset":717948,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":718224,"parent_record_offset":718200,"ancestor_record_offset":717948,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":718152,"parent_record_offset":718120,"ancestor_record_offset":717948,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":718120,"parent_record_offset":718072,"ancestor_record_offset":717948,"inlinee":74521,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":718072,"parent_record_offset":718040,"ancestor_record_offset":717948,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":718008,"parent_record_offset":717976,"ancestor_record_offset":717948,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":718040,"parent_record_offset":718008,"ancestor_record_offset":717948,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$20::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1231E
// 首条位置: {"module":1630,"inlinee":74526}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":727900,"parent_record_offset":724960,"ancestor_record_offset":724960,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":729108,"parent_record_offset":724960,"ancestor_record_offset":724960,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":725000,"parent_record_offset":724960,"ancestor_record_offset":724960,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":727624,"parent_record_offset":724960,"ancestor_record_offset":724960,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":725816,"parent_record_offset":725792,"ancestor_record_offset":724960,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":725792,"parent_record_offset":725736,"ancestor_record_offset":724960,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":725156,"parent_record_offset":725128,"ancestor_record_offset":724960,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":725212,"parent_record_offset":725184,"ancestor_record_offset":724960,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":725236,"parent_record_offset":725212,"ancestor_record_offset":724960,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":726172,"parent_record_offset":726148,"ancestor_record_offset":724960,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":725764,"parent_record_offset":725736,"ancestor_record_offset":724960,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":725736,"parent_record_offset":725712,"ancestor_record_offset":724960,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":726220,"parent_record_offset":726196,"ancestor_record_offset":724960,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":726196,"parent_record_offset":726172,"ancestor_record_offset":724960,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":725712,"parent_record_offset":725688,"ancestor_record_offset":724960,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":726148,"parent_record_offset":726124,"ancestor_record_offset":724960,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":726124,"parent_record_offset":725868,"ancestor_record_offset":724960,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":726740,"parent_record_offset":726716,"ancestor_record_offset":724960,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":727740,"parent_record_offset":727716,"ancestor_record_offset":724960,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":726080,"parent_record_offset":726052,"ancestor_record_offset":724960,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":725184,"parent_record_offset":725156,"ancestor_record_offset":724960,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":727716,"parent_record_offset":727680,"ancestor_record_offset":724960,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":725352,"parent_record_offset":725264,"ancestor_record_offset":724960,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":725580,"parent_record_offset":725072,"ancestor_record_offset":724960,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":725376,"parent_record_offset":725352,"ancestor_record_offset":724960,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":727796,"parent_record_offset":727740,"ancestor_record_offset":724960,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":728264,"parent_record_offset":728212,"ancestor_record_offset":724960,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":726764,"parent_record_offset":726740,"ancestor_record_offset":724960,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":726788,"parent_record_offset":726764,"ancestor_record_offset":724960,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":725264,"parent_record_offset":725236,"ancestor_record_offset":724960,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":725320,"parent_record_offset":725296,"ancestor_record_offset":724960,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":725296,"parent_record_offset":725264,"ancestor_record_offset":724960,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":726716,"parent_record_offset":726684,"ancestor_record_offset":724960,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":728164,"parent_record_offset":728140,"ancestor_record_offset":724960,"inlinee":28073,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":728212,"parent_record_offset":728188,"ancestor_record_offset":724960,"inlinee":28014,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$18::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":726684,"parent_record_offset":726636,"ancestor_record_offset":724960,"inlinee":74530,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$19::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":726512,"parent_record_offset":726480,"ancestor_record_offset":724960,"inlinee":74528,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1630,"record_offset":728980,"parent_record_offset":728956,"ancestor_record_offset":724960,"inlinee":32780,"source_line":69,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1630,"record_offset":729028,"parent_record_offset":729004,"ancestor_record_offset":724960,"inlinee":32782,"source_line":69,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":729180,"parent_record_offset":729156,"ancestor_record_offset":724960,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":729156,"parent_record_offset":729132,"ancestor_record_offset":724960,"inlinee":6705,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":725036,"parent_record_offset":725000,"ancestor_record_offset":724960,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":727652,"parent_record_offset":727624,"ancestor_record_offset":724960,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":726636,"parent_record_offset":726604,"ancestor_record_offset":724960,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":727856,"parent_record_offset":727832,"ancestor_record_offset":724960,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":726480,"parent_record_offset":725072,"ancestor_record_offset":724960,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":728236,"parent_record_offset":728212,"ancestor_record_offset":724960,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$75::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":729052,"parent_record_offset":729028,"ancestor_record_offset":724960,"inlinee":6539,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":725128,"parent_record_offset":725072,"ancestor_record_offset":724960,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":726572,"parent_record_offset":726540,"ancestor_record_offset":724960,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":726540,"parent_record_offset":726512,"ancestor_record_offset":724960,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":726604,"parent_record_offset":726572,"ancestor_record_offset":724960,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":728188,"parent_record_offset":728164,"ancestor_record_offset":724960,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":725072,"parent_record_offset":725036,"ancestor_record_offset":724960,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":727680,"parent_record_offset":727652,"ancestor_record_offset":724960,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$21::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x122B1
// 首条位置: {"module":1630,"inlinee":74417}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":579404,"parent_record_offset":579372,"ancestor_record_offset":579372,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":579428,"parent_record_offset":579404,"ancestor_record_offset":579372,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":579452,"parent_record_offset":579428,"ancestor_record_offset":579372,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":579476,"parent_record_offset":579452,"ancestor_record_offset":579372,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$22::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x122AF
// 首条位置: {"module":1630,"inlinee":74415}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":579228,"parent_record_offset":579200,"ancestor_record_offset":579200,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":579996,"parent_record_offset":579972,"ancestor_record_offset":579200,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":579972,"parent_record_offset":579892,"ancestor_record_offset":579200,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":579932,"parent_record_offset":579892,"ancestor_record_offset":579200,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":579892,"parent_record_offset":579856,"ancestor_record_offset":579200,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":579856,"parent_record_offset":579820,"ancestor_record_offset":579200,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":579428,"parent_record_offset":579404,"ancestor_record_offset":579200,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":579748,"parent_record_offset":579324,"ancestor_record_offset":579200,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":579452,"parent_record_offset":579428,"ancestor_record_offset":579200,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":579476,"parent_record_offset":579452,"ancestor_record_offset":579200,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":579404,"parent_record_offset":579372,"ancestor_record_offset":579200,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$21::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":579372,"parent_record_offset":579324,"ancestor_record_offset":579200,"inlinee":74417,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":579324,"parent_record_offset":579292,"ancestor_record_offset":579200,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":579260,"parent_record_offset":579228,"ancestor_record_offset":579200,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":579292,"parent_record_offset":579260,"ancestor_record_offset":579200,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$23::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x122AD
// 首条位置: {"module":1630,"inlinee":74413}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":580588,"parent_record_offset":577644,"ancestor_record_offset":577644,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":580892,"parent_record_offset":577644,"ancestor_record_offset":577644,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":581684,"parent_record_offset":577644,"ancestor_record_offset":577644,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":577688,"parent_record_offset":577644,"ancestor_record_offset":577644,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":580312,"parent_record_offset":577644,"ancestor_record_offset":577644,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":578504,"parent_record_offset":578480,"ancestor_record_offset":577644,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":578480,"parent_record_offset":578424,"ancestor_record_offset":577644,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":577844,"parent_record_offset":577816,"ancestor_record_offset":577644,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":577900,"parent_record_offset":577872,"ancestor_record_offset":577644,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":577924,"parent_record_offset":577900,"ancestor_record_offset":577644,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":578860,"parent_record_offset":578836,"ancestor_record_offset":577644,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":578452,"parent_record_offset":578424,"ancestor_record_offset":577644,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":578424,"parent_record_offset":578400,"ancestor_record_offset":577644,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":578908,"parent_record_offset":578884,"ancestor_record_offset":577644,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":578884,"parent_record_offset":578860,"ancestor_record_offset":577644,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":578400,"parent_record_offset":578376,"ancestor_record_offset":577644,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":578836,"parent_record_offset":578812,"ancestor_record_offset":577644,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":578812,"parent_record_offset":578556,"ancestor_record_offset":577644,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":579428,"parent_record_offset":579404,"ancestor_record_offset":577644,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":580428,"parent_record_offset":580404,"ancestor_record_offset":577644,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":578768,"parent_record_offset":578740,"ancestor_record_offset":577644,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":577872,"parent_record_offset":577844,"ancestor_record_offset":577644,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":580404,"parent_record_offset":580368,"ancestor_record_offset":577644,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":578040,"parent_record_offset":577952,"ancestor_record_offset":577644,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":578268,"parent_record_offset":577760,"ancestor_record_offset":577644,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":578064,"parent_record_offset":578040,"ancestor_record_offset":577644,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":580484,"parent_record_offset":580428,"ancestor_record_offset":577644,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":581624,"parent_record_offset":581572,"ancestor_record_offset":577644,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":579452,"parent_record_offset":579428,"ancestor_record_offset":577644,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":579476,"parent_record_offset":579452,"ancestor_record_offset":577644,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":577952,"parent_record_offset":577924,"ancestor_record_offset":577644,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":578008,"parent_record_offset":577984,"ancestor_record_offset":577644,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":577984,"parent_record_offset":577952,"ancestor_record_offset":577644,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":579404,"parent_record_offset":579372,"ancestor_record_offset":577644,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$21::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":579372,"parent_record_offset":579324,"ancestor_record_offset":577644,"inlinee":74417,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$22::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":579200,"parent_record_offset":579168,"ancestor_record_offset":577644,"inlinee":74415,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":581524,"parent_record_offset":581500,"ancestor_record_offset":577644,"inlinee":31381,"source_line":591,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":581572,"parent_record_offset":581548,"ancestor_record_offset":577644,"inlinee":31375,"source_line":155,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":581756,"parent_record_offset":581732,"ancestor_record_offset":577644,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_bool
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":581732,"parent_record_offset":581708,"ancestor_record_offset":577644,"inlinee":6619,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":581844,"parent_record_offset":581820,"ancestor_record_offset":577644,"inlinee":6705,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":577724,"parent_record_offset":577688,"ancestor_record_offset":577644,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":580340,"parent_record_offset":580312,"ancestor_record_offset":577644,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":579324,"parent_record_offset":579292,"ancestor_record_offset":577644,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":580544,"parent_record_offset":580520,"ancestor_record_offset":577644,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":579168,"parent_record_offset":577760,"ancestor_record_offset":577644,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":580848,"parent_record_offset":580824,"ancestor_record_offset":577644,"inlinee":6537,"source_line":72,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":581596,"parent_record_offset":581572,"ancestor_record_offset":577644,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":577816,"parent_record_offset":577760,"ancestor_record_offset":577644,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":579260,"parent_record_offset":579228,"ancestor_record_offset":577644,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":579228,"parent_record_offset":579200,"ancestor_record_offset":577644,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":579292,"parent_record_offset":579260,"ancestor_record_offset":577644,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":581548,"parent_record_offset":581524,"ancestor_record_offset":577644,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":577760,"parent_record_offset":577724,"ancestor_record_offset":577644,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":580368,"parent_record_offset":580340,"ancestor_record_offset":577644,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$24::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1229B
// 首条位置: {"module":1630,"inlinee":74395}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":554144,"parent_record_offset":554112,"ancestor_record_offset":554112,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":554168,"parent_record_offset":554144,"ancestor_record_offset":554112,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":554192,"parent_record_offset":554168,"ancestor_record_offset":554112,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":554216,"parent_record_offset":554192,"ancestor_record_offset":554112,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$25::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12299
// 首条位置: {"module":1630,"inlinee":74393}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":553968,"parent_record_offset":553940,"ancestor_record_offset":553940,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":554736,"parent_record_offset":554712,"ancestor_record_offset":553940,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":554712,"parent_record_offset":554632,"ancestor_record_offset":553940,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":554672,"parent_record_offset":554632,"ancestor_record_offset":553940,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":554632,"parent_record_offset":554596,"ancestor_record_offset":553940,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":554596,"parent_record_offset":554560,"ancestor_record_offset":553940,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":554168,"parent_record_offset":554144,"ancestor_record_offset":553940,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":554488,"parent_record_offset":554064,"ancestor_record_offset":553940,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":554192,"parent_record_offset":554168,"ancestor_record_offset":553940,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":554216,"parent_record_offset":554192,"ancestor_record_offset":553940,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":554144,"parent_record_offset":554112,"ancestor_record_offset":553940,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$24::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":554112,"parent_record_offset":554064,"ancestor_record_offset":553940,"inlinee":74395,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":554064,"parent_record_offset":554032,"ancestor_record_offset":553940,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":554000,"parent_record_offset":553968,"ancestor_record_offset":553940,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":554032,"parent_record_offset":554000,"ancestor_record_offset":553940,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$26::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12297
// 首条位置: {"module":1630,"inlinee":74391}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":555328,"parent_record_offset":552388,"ancestor_record_offset":552388,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":555636,"parent_record_offset":552388,"ancestor_record_offset":552388,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":556400,"parent_record_offset":552388,"ancestor_record_offset":552388,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":552428,"parent_record_offset":552388,"ancestor_record_offset":552388,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":555052,"parent_record_offset":552388,"ancestor_record_offset":552388,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":553244,"parent_record_offset":553220,"ancestor_record_offset":552388,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":553220,"parent_record_offset":553164,"ancestor_record_offset":552388,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":552584,"parent_record_offset":552556,"ancestor_record_offset":552388,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":552640,"parent_record_offset":552612,"ancestor_record_offset":552388,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":552664,"parent_record_offset":552640,"ancestor_record_offset":552388,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":553600,"parent_record_offset":553576,"ancestor_record_offset":552388,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":553192,"parent_record_offset":553164,"ancestor_record_offset":552388,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":553164,"parent_record_offset":553140,"ancestor_record_offset":552388,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":553648,"parent_record_offset":553624,"ancestor_record_offset":552388,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":553624,"parent_record_offset":553600,"ancestor_record_offset":552388,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":553140,"parent_record_offset":553116,"ancestor_record_offset":552388,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":553576,"parent_record_offset":553552,"ancestor_record_offset":552388,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":553552,"parent_record_offset":553296,"ancestor_record_offset":552388,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":554168,"parent_record_offset":554144,"ancestor_record_offset":552388,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":555168,"parent_record_offset":555144,"ancestor_record_offset":552388,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":553508,"parent_record_offset":553480,"ancestor_record_offset":552388,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":552612,"parent_record_offset":552584,"ancestor_record_offset":552388,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":555144,"parent_record_offset":555108,"ancestor_record_offset":552388,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":552780,"parent_record_offset":552692,"ancestor_record_offset":552388,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":553008,"parent_record_offset":552500,"ancestor_record_offset":552388,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":552804,"parent_record_offset":552780,"ancestor_record_offset":552388,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":555224,"parent_record_offset":555168,"ancestor_record_offset":552388,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":556340,"parent_record_offset":556288,"ancestor_record_offset":552388,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":554192,"parent_record_offset":554168,"ancestor_record_offset":552388,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":554216,"parent_record_offset":554192,"ancestor_record_offset":552388,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":552692,"parent_record_offset":552664,"ancestor_record_offset":552388,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":552748,"parent_record_offset":552724,"ancestor_record_offset":552388,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":552724,"parent_record_offset":552692,"ancestor_record_offset":552388,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":554144,"parent_record_offset":554112,"ancestor_record_offset":552388,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$24::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":554112,"parent_record_offset":554064,"ancestor_record_offset":552388,"inlinee":74395,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$25::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":553940,"parent_record_offset":553908,"ancestor_record_offset":552388,"inlinee":74393,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":556240,"parent_record_offset":556216,"ancestor_record_offset":552388,"inlinee":31381,"source_line":591,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":556288,"parent_record_offset":556264,"ancestor_record_offset":552388,"inlinee":31375,"source_line":155,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":556472,"parent_record_offset":556448,"ancestor_record_offset":552388,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":556560,"parent_record_offset":556536,"ancestor_record_offset":552388,"inlinee":6671,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":556448,"parent_record_offset":556424,"ancestor_record_offset":552388,"inlinee":6705,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":552464,"parent_record_offset":552428,"ancestor_record_offset":552388,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":555080,"parent_record_offset":555052,"ancestor_record_offset":552388,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":554064,"parent_record_offset":554032,"ancestor_record_offset":552388,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":555284,"parent_record_offset":555260,"ancestor_record_offset":552388,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":553908,"parent_record_offset":552500,"ancestor_record_offset":552388,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":556312,"parent_record_offset":556288,"ancestor_record_offset":552388,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":555592,"parent_record_offset":555568,"ancestor_record_offset":552388,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":552556,"parent_record_offset":552500,"ancestor_record_offset":552388,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":554000,"parent_record_offset":553968,"ancestor_record_offset":552388,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":553968,"parent_record_offset":553940,"ancestor_record_offset":552388,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":554032,"parent_record_offset":554000,"ancestor_record_offset":552388,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":556264,"parent_record_offset":556240,"ancestor_record_offset":552388,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":552500,"parent_record_offset":552464,"ancestor_record_offset":552388,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":555108,"parent_record_offset":555080,"ancestor_record_offset":552388,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$27::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12272
// 首条位置: {"module":1630,"inlinee":74354}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":511188,"parent_record_offset":511156,"ancestor_record_offset":511156,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":511212,"parent_record_offset":511188,"ancestor_record_offset":511156,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":511236,"parent_record_offset":511212,"ancestor_record_offset":511156,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":511260,"parent_record_offset":511236,"ancestor_record_offset":511156,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$28::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12270
// 首条位置: {"module":1630,"inlinee":74352}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":511012,"parent_record_offset":510984,"ancestor_record_offset":510984,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":511780,"parent_record_offset":511756,"ancestor_record_offset":510984,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":511756,"parent_record_offset":511676,"ancestor_record_offset":510984,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":511716,"parent_record_offset":511676,"ancestor_record_offset":510984,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":511676,"parent_record_offset":511640,"ancestor_record_offset":510984,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":511640,"parent_record_offset":511604,"ancestor_record_offset":510984,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":511212,"parent_record_offset":511188,"ancestor_record_offset":510984,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":511532,"parent_record_offset":511108,"ancestor_record_offset":510984,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":511236,"parent_record_offset":511212,"ancestor_record_offset":510984,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":511260,"parent_record_offset":511236,"ancestor_record_offset":510984,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":511188,"parent_record_offset":511156,"ancestor_record_offset":510984,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$27::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":511156,"parent_record_offset":511108,"ancestor_record_offset":510984,"inlinee":74354,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":511108,"parent_record_offset":511076,"ancestor_record_offset":510984,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":511044,"parent_record_offset":511012,"ancestor_record_offset":510984,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":511076,"parent_record_offset":511044,"ancestor_record_offset":510984,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$29::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1226E
// 首条位置: {"module":1630,"inlinee":74350}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":512372,"parent_record_offset":509428,"ancestor_record_offset":509428,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":512676,"parent_record_offset":509428,"ancestor_record_offset":509428,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":513468,"parent_record_offset":509428,"ancestor_record_offset":509428,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":509472,"parent_record_offset":509428,"ancestor_record_offset":509428,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":512096,"parent_record_offset":509428,"ancestor_record_offset":509428,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":510288,"parent_record_offset":510264,"ancestor_record_offset":509428,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":510264,"parent_record_offset":510208,"ancestor_record_offset":509428,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":509628,"parent_record_offset":509600,"ancestor_record_offset":509428,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":509684,"parent_record_offset":509656,"ancestor_record_offset":509428,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":509708,"parent_record_offset":509684,"ancestor_record_offset":509428,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":510644,"parent_record_offset":510620,"ancestor_record_offset":509428,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":510236,"parent_record_offset":510208,"ancestor_record_offset":509428,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":510208,"parent_record_offset":510184,"ancestor_record_offset":509428,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":510692,"parent_record_offset":510668,"ancestor_record_offset":509428,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":510668,"parent_record_offset":510644,"ancestor_record_offset":509428,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":510184,"parent_record_offset":510160,"ancestor_record_offset":509428,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":510620,"parent_record_offset":510596,"ancestor_record_offset":509428,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":510596,"parent_record_offset":510340,"ancestor_record_offset":509428,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":511212,"parent_record_offset":511188,"ancestor_record_offset":509428,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":512212,"parent_record_offset":512188,"ancestor_record_offset":509428,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":510552,"parent_record_offset":510524,"ancestor_record_offset":509428,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":509656,"parent_record_offset":509628,"ancestor_record_offset":509428,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":512188,"parent_record_offset":512152,"ancestor_record_offset":509428,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":509824,"parent_record_offset":509736,"ancestor_record_offset":509428,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":510052,"parent_record_offset":509544,"ancestor_record_offset":509428,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":509848,"parent_record_offset":509824,"ancestor_record_offset":509428,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":512268,"parent_record_offset":512212,"ancestor_record_offset":509428,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":513408,"parent_record_offset":513356,"ancestor_record_offset":509428,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":511236,"parent_record_offset":511212,"ancestor_record_offset":509428,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":511260,"parent_record_offset":511236,"ancestor_record_offset":509428,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":509736,"parent_record_offset":509708,"ancestor_record_offset":509428,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":509792,"parent_record_offset":509768,"ancestor_record_offset":509428,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":509768,"parent_record_offset":509736,"ancestor_record_offset":509428,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":511188,"parent_record_offset":511156,"ancestor_record_offset":509428,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$27::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":511156,"parent_record_offset":511108,"ancestor_record_offset":509428,"inlinee":74354,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$28::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":510984,"parent_record_offset":510952,"ancestor_record_offset":509428,"inlinee":74352,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":513308,"parent_record_offset":513284,"ancestor_record_offset":509428,"inlinee":31381,"source_line":591,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":513356,"parent_record_offset":513332,"ancestor_record_offset":509428,"inlinee":31375,"source_line":155,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":513540,"parent_record_offset":513516,"ancestor_record_offset":509428,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_bool
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":513516,"parent_record_offset":513492,"ancestor_record_offset":509428,"inlinee":6619,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":513628,"parent_record_offset":513604,"ancestor_record_offset":509428,"inlinee":6705,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":509508,"parent_record_offset":509472,"ancestor_record_offset":509428,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":512124,"parent_record_offset":512096,"ancestor_record_offset":509428,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":511108,"parent_record_offset":511076,"ancestor_record_offset":509428,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":512328,"parent_record_offset":512304,"ancestor_record_offset":509428,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":510952,"parent_record_offset":509544,"ancestor_record_offset":509428,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":512632,"parent_record_offset":512608,"ancestor_record_offset":509428,"inlinee":6537,"source_line":72,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":513380,"parent_record_offset":513356,"ancestor_record_offset":509428,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":509600,"parent_record_offset":509544,"ancestor_record_offset":509428,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":511044,"parent_record_offset":511012,"ancestor_record_offset":509428,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":511012,"parent_record_offset":510984,"ancestor_record_offset":509428,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":511076,"parent_record_offset":511044,"ancestor_record_offset":509428,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":513332,"parent_record_offset":513308,"ancestor_record_offset":509428,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":509544,"parent_record_offset":509508,"ancestor_record_offset":509428,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":512152,"parent_record_offset":512124,"ancestor_record_offset":509428,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12315
// 首条位置: {"module":1630,"inlinee":74517}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":719336,"parent_record_offset":716396,"ancestor_record_offset":716396,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":720544,"parent_record_offset":716396,"ancestor_record_offset":716396,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":716436,"parent_record_offset":716396,"ancestor_record_offset":716396,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":719060,"parent_record_offset":716396,"ancestor_record_offset":716396,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":717252,"parent_record_offset":717228,"ancestor_record_offset":716396,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":717228,"parent_record_offset":717172,"ancestor_record_offset":716396,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":716592,"parent_record_offset":716564,"ancestor_record_offset":716396,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":716648,"parent_record_offset":716620,"ancestor_record_offset":716396,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":716672,"parent_record_offset":716648,"ancestor_record_offset":716396,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":717608,"parent_record_offset":717584,"ancestor_record_offset":716396,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":717200,"parent_record_offset":717172,"ancestor_record_offset":716396,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":717172,"parent_record_offset":717148,"ancestor_record_offset":716396,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":717656,"parent_record_offset":717632,"ancestor_record_offset":716396,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":717632,"parent_record_offset":717608,"ancestor_record_offset":716396,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":717148,"parent_record_offset":717124,"ancestor_record_offset":716396,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":717584,"parent_record_offset":717560,"ancestor_record_offset":716396,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":717560,"parent_record_offset":717304,"ancestor_record_offset":716396,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":718176,"parent_record_offset":718152,"ancestor_record_offset":716396,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":719176,"parent_record_offset":719152,"ancestor_record_offset":716396,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":717516,"parent_record_offset":717488,"ancestor_record_offset":716396,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":716620,"parent_record_offset":716592,"ancestor_record_offset":716396,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":719152,"parent_record_offset":719116,"ancestor_record_offset":716396,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":716788,"parent_record_offset":716700,"ancestor_record_offset":716396,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":717016,"parent_record_offset":716508,"ancestor_record_offset":716396,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":716812,"parent_record_offset":716788,"ancestor_record_offset":716396,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":719232,"parent_record_offset":719176,"ancestor_record_offset":716396,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":719700,"parent_record_offset":719648,"ancestor_record_offset":716396,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":718200,"parent_record_offset":718176,"ancestor_record_offset":716396,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":718224,"parent_record_offset":718200,"ancestor_record_offset":716396,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":716700,"parent_record_offset":716672,"ancestor_record_offset":716396,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":716756,"parent_record_offset":716732,"ancestor_record_offset":716396,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":716732,"parent_record_offset":716700,"ancestor_record_offset":716396,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":718152,"parent_record_offset":718120,"ancestor_record_offset":716396,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":719600,"parent_record_offset":719576,"ancestor_record_offset":716396,"inlinee":28073,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":719648,"parent_record_offset":719624,"ancestor_record_offset":716396,"inlinee":28014,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":718120,"parent_record_offset":718072,"ancestor_record_offset":716396,"inlinee":74521,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":717948,"parent_record_offset":717916,"ancestor_record_offset":716396,"inlinee":74519,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1630,"record_offset":720416,"parent_record_offset":720392,"ancestor_record_offset":716396,"inlinee":32780,"source_line":69,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1630,"record_offset":720464,"parent_record_offset":720440,"ancestor_record_offset":716396,"inlinee":32782,"source_line":69,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":720616,"parent_record_offset":720592,"ancestor_record_offset":716396,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":720592,"parent_record_offset":720568,"ancestor_record_offset":716396,"inlinee":6705,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":716472,"parent_record_offset":716436,"ancestor_record_offset":716396,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":719088,"parent_record_offset":719060,"ancestor_record_offset":716396,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":718072,"parent_record_offset":718040,"ancestor_record_offset":716396,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":719292,"parent_record_offset":719268,"ancestor_record_offset":716396,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":717916,"parent_record_offset":716508,"ancestor_record_offset":716396,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":719672,"parent_record_offset":719648,"ancestor_record_offset":716396,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$75::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":720488,"parent_record_offset":720464,"ancestor_record_offset":716396,"inlinee":6539,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":716564,"parent_record_offset":716508,"ancestor_record_offset":716396,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":718008,"parent_record_offset":717976,"ancestor_record_offset":716396,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":717976,"parent_record_offset":717948,"ancestor_record_offset":716396,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":718040,"parent_record_offset":718008,"ancestor_record_offset":716396,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":719624,"parent_record_offset":719600,"ancestor_record_offset":716396,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":716508,"parent_record_offset":716472,"ancestor_record_offset":716396,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":719116,"parent_record_offset":719088,"ancestor_record_offset":716396,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$30::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1229F
// 首条位置: {"module":1630,"inlinee":74399}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":558916,"parent_record_offset":558884,"ancestor_record_offset":558884,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":558940,"parent_record_offset":558916,"ancestor_record_offset":558884,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":558964,"parent_record_offset":558940,"ancestor_record_offset":558884,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":558988,"parent_record_offset":558964,"ancestor_record_offset":558884,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$31::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1229E
// 首条位置: {"module":1630,"inlinee":74398}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":558740,"parent_record_offset":558712,"ancestor_record_offset":558712,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":559508,"parent_record_offset":559484,"ancestor_record_offset":558712,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":559484,"parent_record_offset":559404,"ancestor_record_offset":558712,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":559444,"parent_record_offset":559404,"ancestor_record_offset":558712,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":559404,"parent_record_offset":559368,"ancestor_record_offset":558712,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":559368,"parent_record_offset":559332,"ancestor_record_offset":558712,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":558940,"parent_record_offset":558916,"ancestor_record_offset":558712,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":559260,"parent_record_offset":558836,"ancestor_record_offset":558712,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":558964,"parent_record_offset":558940,"ancestor_record_offset":558712,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":558988,"parent_record_offset":558964,"ancestor_record_offset":558712,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":558916,"parent_record_offset":558884,"ancestor_record_offset":558712,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$30::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":558884,"parent_record_offset":558836,"ancestor_record_offset":558712,"inlinee":74399,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":558836,"parent_record_offset":558804,"ancestor_record_offset":558712,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":558772,"parent_record_offset":558740,"ancestor_record_offset":558712,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":558804,"parent_record_offset":558772,"ancestor_record_offset":558712,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$32::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1229C
// 首条位置: {"module":1630,"inlinee":74396}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":560100,"parent_record_offset":557160,"ancestor_record_offset":557160,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":561340,"parent_record_offset":557160,"ancestor_record_offset":557160,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":557200,"parent_record_offset":557160,"ancestor_record_offset":557160,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":559824,"parent_record_offset":557160,"ancestor_record_offset":557160,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":558016,"parent_record_offset":557992,"ancestor_record_offset":557160,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":557992,"parent_record_offset":557936,"ancestor_record_offset":557160,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":557356,"parent_record_offset":557328,"ancestor_record_offset":557160,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":557412,"parent_record_offset":557384,"ancestor_record_offset":557160,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":557436,"parent_record_offset":557412,"ancestor_record_offset":557160,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":558372,"parent_record_offset":558348,"ancestor_record_offset":557160,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":557964,"parent_record_offset":557936,"ancestor_record_offset":557160,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":557936,"parent_record_offset":557912,"ancestor_record_offset":557160,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":558420,"parent_record_offset":558396,"ancestor_record_offset":557160,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":558396,"parent_record_offset":558372,"ancestor_record_offset":557160,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":557912,"parent_record_offset":557888,"ancestor_record_offset":557160,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":558348,"parent_record_offset":558324,"ancestor_record_offset":557160,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":558324,"parent_record_offset":558068,"ancestor_record_offset":557160,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":558940,"parent_record_offset":558916,"ancestor_record_offset":557160,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":559940,"parent_record_offset":559916,"ancestor_record_offset":557160,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":558280,"parent_record_offset":558252,"ancestor_record_offset":557160,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":557384,"parent_record_offset":557356,"ancestor_record_offset":557160,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":559916,"parent_record_offset":559880,"ancestor_record_offset":557160,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":557552,"parent_record_offset":557464,"ancestor_record_offset":557160,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":557780,"parent_record_offset":557272,"ancestor_record_offset":557160,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":557576,"parent_record_offset":557552,"ancestor_record_offset":557160,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":559996,"parent_record_offset":559940,"ancestor_record_offset":557160,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":560464,"parent_record_offset":560412,"ancestor_record_offset":557160,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":558964,"parent_record_offset":558940,"ancestor_record_offset":557160,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":558988,"parent_record_offset":558964,"ancestor_record_offset":557160,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":557464,"parent_record_offset":557436,"ancestor_record_offset":557160,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":557520,"parent_record_offset":557496,"ancestor_record_offset":557160,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":557496,"parent_record_offset":557464,"ancestor_record_offset":557160,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":558916,"parent_record_offset":558884,"ancestor_record_offset":557160,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":560364,"parent_record_offset":560340,"ancestor_record_offset":557160,"inlinee":28073,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":560412,"parent_record_offset":560388,"ancestor_record_offset":557160,"inlinee":28014,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$30::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":558884,"parent_record_offset":558836,"ancestor_record_offset":557160,"inlinee":74399,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$31::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":558712,"parent_record_offset":558680,"ancestor_record_offset":557160,"inlinee":74398,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":561180,"parent_record_offset":561156,"ancestor_record_offset":557160,"inlinee":31381,"source_line":591,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":561228,"parent_record_offset":561204,"ancestor_record_offset":557160,"inlinee":31375,"source_line":155,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":561412,"parent_record_offset":561388,"ancestor_record_offset":557160,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":561388,"parent_record_offset":561364,"ancestor_record_offset":557160,"inlinee":6705,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":557236,"parent_record_offset":557200,"ancestor_record_offset":557160,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":559852,"parent_record_offset":559824,"ancestor_record_offset":557160,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":558836,"parent_record_offset":558804,"ancestor_record_offset":557160,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":560056,"parent_record_offset":560032,"ancestor_record_offset":557160,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":558680,"parent_record_offset":557272,"ancestor_record_offset":557160,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":560436,"parent_record_offset":560412,"ancestor_record_offset":557160,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":557328,"parent_record_offset":557272,"ancestor_record_offset":557160,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":558772,"parent_record_offset":558740,"ancestor_record_offset":557160,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":558740,"parent_record_offset":558712,"ancestor_record_offset":557160,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":558804,"parent_record_offset":558772,"ancestor_record_offset":557160,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":560388,"parent_record_offset":560364,"ancestor_record_offset":557160,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":557272,"parent_record_offset":557236,"ancestor_record_offset":557160,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":559880,"parent_record_offset":559852,"ancestor_record_offset":557160,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$33::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x122CB
// 首条位置: {"module":1630,"inlinee":74443}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":611776,"parent_record_offset":611744,"ancestor_record_offset":611744,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":611800,"parent_record_offset":611776,"ancestor_record_offset":611744,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":611824,"parent_record_offset":611800,"ancestor_record_offset":611744,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":611848,"parent_record_offset":611824,"ancestor_record_offset":611744,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$34::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x122C9
// 首条位置: {"module":1630,"inlinee":74441}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":611600,"parent_record_offset":611572,"ancestor_record_offset":611572,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":612368,"parent_record_offset":612344,"ancestor_record_offset":611572,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":612344,"parent_record_offset":612264,"ancestor_record_offset":611572,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":612304,"parent_record_offset":612264,"ancestor_record_offset":611572,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":612264,"parent_record_offset":612228,"ancestor_record_offset":611572,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":612228,"parent_record_offset":612192,"ancestor_record_offset":611572,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":611800,"parent_record_offset":611776,"ancestor_record_offset":611572,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":612120,"parent_record_offset":611696,"ancestor_record_offset":611572,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":611824,"parent_record_offset":611800,"ancestor_record_offset":611572,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":611848,"parent_record_offset":611824,"ancestor_record_offset":611572,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":611776,"parent_record_offset":611744,"ancestor_record_offset":611572,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$33::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":611744,"parent_record_offset":611696,"ancestor_record_offset":611572,"inlinee":74443,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":611696,"parent_record_offset":611664,"ancestor_record_offset":611572,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":611632,"parent_record_offset":611600,"ancestor_record_offset":611572,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":611664,"parent_record_offset":611632,"ancestor_record_offset":611572,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$35::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x122C7
// 首条位置: {"module":1630,"inlinee":74439}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":612960,"parent_record_offset":610016,"ancestor_record_offset":610016,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":613236,"parent_record_offset":610016,"ancestor_record_offset":610016,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":614000,"parent_record_offset":610016,"ancestor_record_offset":610016,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":610060,"parent_record_offset":610016,"ancestor_record_offset":610016,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":612684,"parent_record_offset":610016,"ancestor_record_offset":610016,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":610876,"parent_record_offset":610852,"ancestor_record_offset":610016,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":610852,"parent_record_offset":610796,"ancestor_record_offset":610016,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":610216,"parent_record_offset":610188,"ancestor_record_offset":610016,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":610272,"parent_record_offset":610244,"ancestor_record_offset":610016,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":610296,"parent_record_offset":610272,"ancestor_record_offset":610016,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":611232,"parent_record_offset":611208,"ancestor_record_offset":610016,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":610824,"parent_record_offset":610796,"ancestor_record_offset":610016,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":610796,"parent_record_offset":610772,"ancestor_record_offset":610016,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":611280,"parent_record_offset":611256,"ancestor_record_offset":610016,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":611256,"parent_record_offset":611232,"ancestor_record_offset":610016,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":610772,"parent_record_offset":610748,"ancestor_record_offset":610016,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":611208,"parent_record_offset":611184,"ancestor_record_offset":610016,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":611184,"parent_record_offset":610928,"ancestor_record_offset":610016,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":611800,"parent_record_offset":611776,"ancestor_record_offset":610016,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":612800,"parent_record_offset":612776,"ancestor_record_offset":610016,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":611140,"parent_record_offset":611112,"ancestor_record_offset":610016,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":610244,"parent_record_offset":610216,"ancestor_record_offset":610016,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":612776,"parent_record_offset":612740,"ancestor_record_offset":610016,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":610412,"parent_record_offset":610324,"ancestor_record_offset":610016,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":610640,"parent_record_offset":610132,"ancestor_record_offset":610016,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":610436,"parent_record_offset":610412,"ancestor_record_offset":610016,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":612856,"parent_record_offset":612800,"ancestor_record_offset":610016,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":613940,"parent_record_offset":613888,"ancestor_record_offset":610016,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":611824,"parent_record_offset":611800,"ancestor_record_offset":610016,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":611848,"parent_record_offset":611824,"ancestor_record_offset":610016,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":610324,"parent_record_offset":610296,"ancestor_record_offset":610016,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":610380,"parent_record_offset":610356,"ancestor_record_offset":610016,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":610356,"parent_record_offset":610324,"ancestor_record_offset":610016,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":611776,"parent_record_offset":611744,"ancestor_record_offset":610016,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$33::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":611744,"parent_record_offset":611696,"ancestor_record_offset":610016,"inlinee":74443,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$34::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":611572,"parent_record_offset":611540,"ancestor_record_offset":610016,"inlinee":74441,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":613840,"parent_record_offset":613816,"ancestor_record_offset":610016,"inlinee":31381,"source_line":591,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":613888,"parent_record_offset":613864,"ancestor_record_offset":610016,"inlinee":31375,"source_line":155,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":614072,"parent_record_offset":614048,"ancestor_record_offset":610016,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":614048,"parent_record_offset":614024,"ancestor_record_offset":610016,"inlinee":6675,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":614160,"parent_record_offset":614136,"ancestor_record_offset":610016,"inlinee":6705,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":610096,"parent_record_offset":610060,"ancestor_record_offset":610016,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":612712,"parent_record_offset":612684,"ancestor_record_offset":610016,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":611696,"parent_record_offset":611664,"ancestor_record_offset":610016,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":612916,"parent_record_offset":612892,"ancestor_record_offset":610016,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":611540,"parent_record_offset":610132,"ancestor_record_offset":610016,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":613912,"parent_record_offset":613888,"ancestor_record_offset":610016,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":610188,"parent_record_offset":610132,"ancestor_record_offset":610016,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":611632,"parent_record_offset":611600,"ancestor_record_offset":610016,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":611600,"parent_record_offset":611572,"ancestor_record_offset":610016,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":611664,"parent_record_offset":611632,"ancestor_record_offset":610016,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":613864,"parent_record_offset":613840,"ancestor_record_offset":610016,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":610132,"parent_record_offset":610096,"ancestor_record_offset":610016,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":612740,"parent_record_offset":612712,"ancestor_record_offset":610016,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$36::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x122C3
// 首条位置: {"module":1630,"inlinee":74435}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":601984,"parent_record_offset":601952,"ancestor_record_offset":601952,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":602008,"parent_record_offset":601984,"ancestor_record_offset":601952,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":602032,"parent_record_offset":602008,"ancestor_record_offset":601952,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":602056,"parent_record_offset":602032,"ancestor_record_offset":601952,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$37::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x122C1
// 首条位置: {"module":1630,"inlinee":74433}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":601808,"parent_record_offset":601780,"ancestor_record_offset":601780,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":602576,"parent_record_offset":602552,"ancestor_record_offset":601780,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":602552,"parent_record_offset":602472,"ancestor_record_offset":601780,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":602512,"parent_record_offset":602472,"ancestor_record_offset":601780,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":602472,"parent_record_offset":602436,"ancestor_record_offset":601780,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":602436,"parent_record_offset":602400,"ancestor_record_offset":601780,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":602008,"parent_record_offset":601984,"ancestor_record_offset":601780,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":602328,"parent_record_offset":601904,"ancestor_record_offset":601780,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":602032,"parent_record_offset":602008,"ancestor_record_offset":601780,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":602056,"parent_record_offset":602032,"ancestor_record_offset":601780,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":601984,"parent_record_offset":601952,"ancestor_record_offset":601780,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$36::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":601952,"parent_record_offset":601904,"ancestor_record_offset":601780,"inlinee":74435,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":601904,"parent_record_offset":601872,"ancestor_record_offset":601780,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":601840,"parent_record_offset":601808,"ancestor_record_offset":601780,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":601872,"parent_record_offset":601840,"ancestor_record_offset":601780,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$38::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x122BF
// 首条位置: {"module":1630,"inlinee":74431}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":603168,"parent_record_offset":600228,"ancestor_record_offset":600228,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":604408,"parent_record_offset":600228,"ancestor_record_offset":600228,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":600268,"parent_record_offset":600228,"ancestor_record_offset":600228,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":602892,"parent_record_offset":600228,"ancestor_record_offset":600228,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":601084,"parent_record_offset":601060,"ancestor_record_offset":600228,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":601060,"parent_record_offset":601004,"ancestor_record_offset":600228,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":600424,"parent_record_offset":600396,"ancestor_record_offset":600228,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":600480,"parent_record_offset":600452,"ancestor_record_offset":600228,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":600504,"parent_record_offset":600480,"ancestor_record_offset":600228,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":601440,"parent_record_offset":601416,"ancestor_record_offset":600228,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":601032,"parent_record_offset":601004,"ancestor_record_offset":600228,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":601004,"parent_record_offset":600980,"ancestor_record_offset":600228,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":601488,"parent_record_offset":601464,"ancestor_record_offset":600228,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":601464,"parent_record_offset":601440,"ancestor_record_offset":600228,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":600980,"parent_record_offset":600956,"ancestor_record_offset":600228,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":601416,"parent_record_offset":601392,"ancestor_record_offset":600228,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":601392,"parent_record_offset":601136,"ancestor_record_offset":600228,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":602008,"parent_record_offset":601984,"ancestor_record_offset":600228,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":603008,"parent_record_offset":602984,"ancestor_record_offset":600228,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":601348,"parent_record_offset":601320,"ancestor_record_offset":600228,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":600452,"parent_record_offset":600424,"ancestor_record_offset":600228,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":602984,"parent_record_offset":602948,"ancestor_record_offset":600228,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":600620,"parent_record_offset":600532,"ancestor_record_offset":600228,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":600848,"parent_record_offset":600340,"ancestor_record_offset":600228,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":600644,"parent_record_offset":600620,"ancestor_record_offset":600228,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":603064,"parent_record_offset":603008,"ancestor_record_offset":600228,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":603532,"parent_record_offset":603480,"ancestor_record_offset":600228,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":602032,"parent_record_offset":602008,"ancestor_record_offset":600228,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":602056,"parent_record_offset":602032,"ancestor_record_offset":600228,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":600532,"parent_record_offset":600504,"ancestor_record_offset":600228,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":600588,"parent_record_offset":600564,"ancestor_record_offset":600228,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":600564,"parent_record_offset":600532,"ancestor_record_offset":600228,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":601984,"parent_record_offset":601952,"ancestor_record_offset":600228,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":603432,"parent_record_offset":603408,"ancestor_record_offset":600228,"inlinee":28073,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":603480,"parent_record_offset":603456,"ancestor_record_offset":600228,"inlinee":28014,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$36::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":601952,"parent_record_offset":601904,"ancestor_record_offset":600228,"inlinee":74435,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$37::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":601780,"parent_record_offset":601748,"ancestor_record_offset":600228,"inlinee":74433,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":604248,"parent_record_offset":604224,"ancestor_record_offset":600228,"inlinee":31381,"source_line":591,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":604296,"parent_record_offset":604272,"ancestor_record_offset":600228,"inlinee":31375,"source_line":155,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":604480,"parent_record_offset":604456,"ancestor_record_offset":600228,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":604456,"parent_record_offset":604432,"ancestor_record_offset":600228,"inlinee":6705,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":600304,"parent_record_offset":600268,"ancestor_record_offset":600228,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":602920,"parent_record_offset":602892,"ancestor_record_offset":600228,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":601904,"parent_record_offset":601872,"ancestor_record_offset":600228,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":603124,"parent_record_offset":603100,"ancestor_record_offset":600228,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":601748,"parent_record_offset":600340,"ancestor_record_offset":600228,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":603504,"parent_record_offset":603480,"ancestor_record_offset":600228,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":600396,"parent_record_offset":600340,"ancestor_record_offset":600228,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":601840,"parent_record_offset":601808,"ancestor_record_offset":600228,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":601808,"parent_record_offset":601780,"ancestor_record_offset":600228,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":601872,"parent_record_offset":601840,"ancestor_record_offset":600228,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":603456,"parent_record_offset":603432,"ancestor_record_offset":600228,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":600340,"parent_record_offset":600304,"ancestor_record_offset":600228,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":602948,"parent_record_offset":602920,"ancestor_record_offset":600228,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$39::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1228C
// 首条位置: {"module":1630,"inlinee":74380}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":538992,"parent_record_offset":538960,"ancestor_record_offset":538960,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":539016,"parent_record_offset":538992,"ancestor_record_offset":538960,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":539040,"parent_record_offset":539016,"ancestor_record_offset":538960,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":539064,"parent_record_offset":539040,"ancestor_record_offset":538960,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12327
// 首条位置: {"module":1630,"inlinee":74535}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":737920,"parent_record_offset":737888,"ancestor_record_offset":737888,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":737944,"parent_record_offset":737920,"ancestor_record_offset":737888,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":737968,"parent_record_offset":737944,"ancestor_record_offset":737888,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":737992,"parent_record_offset":737968,"ancestor_record_offset":737888,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$40::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1228A
// 首条位置: {"module":1630,"inlinee":74378}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":538816,"parent_record_offset":538788,"ancestor_record_offset":538788,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":539584,"parent_record_offset":539560,"ancestor_record_offset":538788,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":539560,"parent_record_offset":539480,"ancestor_record_offset":538788,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":539520,"parent_record_offset":539480,"ancestor_record_offset":538788,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":539480,"parent_record_offset":539444,"ancestor_record_offset":538788,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":539444,"parent_record_offset":539408,"ancestor_record_offset":538788,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":539016,"parent_record_offset":538992,"ancestor_record_offset":538788,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":539336,"parent_record_offset":538912,"ancestor_record_offset":538788,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":539040,"parent_record_offset":539016,"ancestor_record_offset":538788,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":539064,"parent_record_offset":539040,"ancestor_record_offset":538788,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":538992,"parent_record_offset":538960,"ancestor_record_offset":538788,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$39::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":538960,"parent_record_offset":538912,"ancestor_record_offset":538788,"inlinee":74380,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":538912,"parent_record_offset":538880,"ancestor_record_offset":538788,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":538848,"parent_record_offset":538816,"ancestor_record_offset":538788,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":538880,"parent_record_offset":538848,"ancestor_record_offset":538788,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$41::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12288
// 首条位置: {"module":1630,"inlinee":74376}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":540176,"parent_record_offset":537232,"ancestor_record_offset":537232,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":540452,"parent_record_offset":537232,"ancestor_record_offset":537232,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":541216,"parent_record_offset":537232,"ancestor_record_offset":537232,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":537276,"parent_record_offset":537232,"ancestor_record_offset":537232,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":539900,"parent_record_offset":537232,"ancestor_record_offset":537232,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":538092,"parent_record_offset":538068,"ancestor_record_offset":537232,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":538068,"parent_record_offset":538012,"ancestor_record_offset":537232,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":537432,"parent_record_offset":537404,"ancestor_record_offset":537232,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":537488,"parent_record_offset":537460,"ancestor_record_offset":537232,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":537512,"parent_record_offset":537488,"ancestor_record_offset":537232,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":538448,"parent_record_offset":538424,"ancestor_record_offset":537232,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":538040,"parent_record_offset":538012,"ancestor_record_offset":537232,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":538012,"parent_record_offset":537988,"ancestor_record_offset":537232,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":538496,"parent_record_offset":538472,"ancestor_record_offset":537232,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":538472,"parent_record_offset":538448,"ancestor_record_offset":537232,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":537988,"parent_record_offset":537964,"ancestor_record_offset":537232,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":538424,"parent_record_offset":538400,"ancestor_record_offset":537232,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":538400,"parent_record_offset":538144,"ancestor_record_offset":537232,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":539016,"parent_record_offset":538992,"ancestor_record_offset":537232,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":540016,"parent_record_offset":539992,"ancestor_record_offset":537232,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":538356,"parent_record_offset":538328,"ancestor_record_offset":537232,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":537460,"parent_record_offset":537432,"ancestor_record_offset":537232,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":539992,"parent_record_offset":539956,"ancestor_record_offset":537232,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":537628,"parent_record_offset":537540,"ancestor_record_offset":537232,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":537856,"parent_record_offset":537348,"ancestor_record_offset":537232,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":537652,"parent_record_offset":537628,"ancestor_record_offset":537232,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":540072,"parent_record_offset":540016,"ancestor_record_offset":537232,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":541156,"parent_record_offset":541104,"ancestor_record_offset":537232,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":539040,"parent_record_offset":539016,"ancestor_record_offset":537232,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":539064,"parent_record_offset":539040,"ancestor_record_offset":537232,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":537540,"parent_record_offset":537512,"ancestor_record_offset":537232,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":537596,"parent_record_offset":537572,"ancestor_record_offset":537232,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":537572,"parent_record_offset":537540,"ancestor_record_offset":537232,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":538992,"parent_record_offset":538960,"ancestor_record_offset":537232,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$39::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":538960,"parent_record_offset":538912,"ancestor_record_offset":537232,"inlinee":74380,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$40::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":538788,"parent_record_offset":538756,"ancestor_record_offset":537232,"inlinee":74378,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":541056,"parent_record_offset":541032,"ancestor_record_offset":537232,"inlinee":31381,"source_line":591,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":541104,"parent_record_offset":541080,"ancestor_record_offset":537232,"inlinee":31375,"source_line":155,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":541288,"parent_record_offset":541264,"ancestor_record_offset":537232,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":541264,"parent_record_offset":541240,"ancestor_record_offset":537232,"inlinee":6675,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":541376,"parent_record_offset":541352,"ancestor_record_offset":537232,"inlinee":6705,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":537312,"parent_record_offset":537276,"ancestor_record_offset":537232,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":539928,"parent_record_offset":539900,"ancestor_record_offset":537232,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":538912,"parent_record_offset":538880,"ancestor_record_offset":537232,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":540132,"parent_record_offset":540108,"ancestor_record_offset":537232,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":538756,"parent_record_offset":537348,"ancestor_record_offset":537232,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":541128,"parent_record_offset":541104,"ancestor_record_offset":537232,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":537404,"parent_record_offset":537348,"ancestor_record_offset":537232,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":538848,"parent_record_offset":538816,"ancestor_record_offset":537232,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":538816,"parent_record_offset":538788,"ancestor_record_offset":537232,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":538880,"parent_record_offset":538848,"ancestor_record_offset":537232,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":541080,"parent_record_offset":541056,"ancestor_record_offset":537232,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":537348,"parent_record_offset":537312,"ancestor_record_offset":537232,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":539956,"parent_record_offset":539928,"ancestor_record_offset":537232,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$4::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12325
// 首条位置: {"module":1630,"inlinee":74533}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":737744,"parent_record_offset":737716,"ancestor_record_offset":737716,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":738512,"parent_record_offset":738488,"ancestor_record_offset":737716,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":738488,"parent_record_offset":738408,"ancestor_record_offset":737716,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":738448,"parent_record_offset":738408,"ancestor_record_offset":737716,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":738408,"parent_record_offset":738372,"ancestor_record_offset":737716,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":738372,"parent_record_offset":738336,"ancestor_record_offset":737716,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":737944,"parent_record_offset":737920,"ancestor_record_offset":737716,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":738264,"parent_record_offset":737840,"ancestor_record_offset":737716,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":737968,"parent_record_offset":737944,"ancestor_record_offset":737716,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":737992,"parent_record_offset":737968,"ancestor_record_offset":737716,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":737920,"parent_record_offset":737888,"ancestor_record_offset":737716,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":737888,"parent_record_offset":737840,"ancestor_record_offset":737716,"inlinee":74535,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":737840,"parent_record_offset":737808,"ancestor_record_offset":737716,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":737776,"parent_record_offset":737744,"ancestor_record_offset":737716,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":737808,"parent_record_offset":737776,"ancestor_record_offset":737716,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$5::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12323
// 首条位置: {"module":1630,"inlinee":74531}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":739104,"parent_record_offset":736164,"ancestor_record_offset":736164,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":740312,"parent_record_offset":736164,"ancestor_record_offset":736164,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":736204,"parent_record_offset":736164,"ancestor_record_offset":736164,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":738828,"parent_record_offset":736164,"ancestor_record_offset":736164,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":737020,"parent_record_offset":736996,"ancestor_record_offset":736164,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":736996,"parent_record_offset":736940,"ancestor_record_offset":736164,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":736360,"parent_record_offset":736332,"ancestor_record_offset":736164,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":736416,"parent_record_offset":736388,"ancestor_record_offset":736164,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":736440,"parent_record_offset":736416,"ancestor_record_offset":736164,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":737376,"parent_record_offset":737352,"ancestor_record_offset":736164,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":736968,"parent_record_offset":736940,"ancestor_record_offset":736164,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":736940,"parent_record_offset":736916,"ancestor_record_offset":736164,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":737424,"parent_record_offset":737400,"ancestor_record_offset":736164,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":737400,"parent_record_offset":737376,"ancestor_record_offset":736164,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":736916,"parent_record_offset":736892,"ancestor_record_offset":736164,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":737352,"parent_record_offset":737328,"ancestor_record_offset":736164,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":737328,"parent_record_offset":737072,"ancestor_record_offset":736164,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":737944,"parent_record_offset":737920,"ancestor_record_offset":736164,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":738944,"parent_record_offset":738920,"ancestor_record_offset":736164,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":737284,"parent_record_offset":737256,"ancestor_record_offset":736164,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":736388,"parent_record_offset":736360,"ancestor_record_offset":736164,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":738920,"parent_record_offset":738884,"ancestor_record_offset":736164,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":736556,"parent_record_offset":736468,"ancestor_record_offset":736164,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":736784,"parent_record_offset":736276,"ancestor_record_offset":736164,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":736580,"parent_record_offset":736556,"ancestor_record_offset":736164,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":739000,"parent_record_offset":738944,"ancestor_record_offset":736164,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":739468,"parent_record_offset":739416,"ancestor_record_offset":736164,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":737968,"parent_record_offset":737944,"ancestor_record_offset":736164,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":737992,"parent_record_offset":737968,"ancestor_record_offset":736164,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":736468,"parent_record_offset":736440,"ancestor_record_offset":736164,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":736524,"parent_record_offset":736500,"ancestor_record_offset":736164,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":736500,"parent_record_offset":736468,"ancestor_record_offset":736164,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":737920,"parent_record_offset":737888,"ancestor_record_offset":736164,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":739368,"parent_record_offset":739344,"ancestor_record_offset":736164,"inlinee":28073,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":739416,"parent_record_offset":739392,"ancestor_record_offset":736164,"inlinee":28014,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":737888,"parent_record_offset":737840,"ancestor_record_offset":736164,"inlinee":74535,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$4::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":737716,"parent_record_offset":737684,"ancestor_record_offset":736164,"inlinee":74533,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1630,"record_offset":740184,"parent_record_offset":740160,"ancestor_record_offset":736164,"inlinee":32780,"source_line":69,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1630,"record_offset":740232,"parent_record_offset":740208,"ancestor_record_offset":736164,"inlinee":32782,"source_line":69,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":740384,"parent_record_offset":740360,"ancestor_record_offset":736164,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":740360,"parent_record_offset":740336,"ancestor_record_offset":736164,"inlinee":6705,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":736240,"parent_record_offset":736204,"ancestor_record_offset":736164,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":738856,"parent_record_offset":738828,"ancestor_record_offset":736164,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":737840,"parent_record_offset":737808,"ancestor_record_offset":736164,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":739060,"parent_record_offset":739036,"ancestor_record_offset":736164,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":737684,"parent_record_offset":736276,"ancestor_record_offset":736164,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":739440,"parent_record_offset":739416,"ancestor_record_offset":736164,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$75::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":740256,"parent_record_offset":740232,"ancestor_record_offset":736164,"inlinee":6539,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":736332,"parent_record_offset":736276,"ancestor_record_offset":736164,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":737776,"parent_record_offset":737744,"ancestor_record_offset":736164,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":737744,"parent_record_offset":737716,"ancestor_record_offset":736164,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":737808,"parent_record_offset":737776,"ancestor_record_offset":736164,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":739392,"parent_record_offset":739368,"ancestor_record_offset":736164,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":736276,"parent_record_offset":736240,"ancestor_record_offset":736164,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":738884,"parent_record_offset":738856,"ancestor_record_offset":736164,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x122A7
// 首条位置: {"module":1630,"inlinee":74407}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":569580,"parent_record_offset":569548,"ancestor_record_offset":569548,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":569604,"parent_record_offset":569580,"ancestor_record_offset":569548,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":569628,"parent_record_offset":569604,"ancestor_record_offset":569548,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":569652,"parent_record_offset":569628,"ancestor_record_offset":569548,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$7::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x122A5
// 首条位置: {"module":1630,"inlinee":74405}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":569404,"parent_record_offset":569376,"ancestor_record_offset":569376,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":570172,"parent_record_offset":570148,"ancestor_record_offset":569376,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":570148,"parent_record_offset":570068,"ancestor_record_offset":569376,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":570108,"parent_record_offset":570068,"ancestor_record_offset":569376,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":570068,"parent_record_offset":570032,"ancestor_record_offset":569376,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":570032,"parent_record_offset":569996,"ancestor_record_offset":569376,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":569604,"parent_record_offset":569580,"ancestor_record_offset":569376,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":569924,"parent_record_offset":569500,"ancestor_record_offset":569376,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":569628,"parent_record_offset":569604,"ancestor_record_offset":569376,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":569652,"parent_record_offset":569628,"ancestor_record_offset":569376,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":569580,"parent_record_offset":569548,"ancestor_record_offset":569376,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":569548,"parent_record_offset":569500,"ancestor_record_offset":569376,"inlinee":74407,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":569500,"parent_record_offset":569468,"ancestor_record_offset":569376,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":569436,"parent_record_offset":569404,"ancestor_record_offset":569376,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":569468,"parent_record_offset":569436,"ancestor_record_offset":569376,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$8::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x122A3
// 首条位置: {"module":1630,"inlinee":74403}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":570764,"parent_record_offset":567824,"ancestor_record_offset":567824,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":571972,"parent_record_offset":567824,"ancestor_record_offset":567824,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":567864,"parent_record_offset":567824,"ancestor_record_offset":567824,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":570488,"parent_record_offset":567824,"ancestor_record_offset":567824,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":568680,"parent_record_offset":568656,"ancestor_record_offset":567824,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":568656,"parent_record_offset":568600,"ancestor_record_offset":567824,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":568020,"parent_record_offset":567992,"ancestor_record_offset":567824,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":568076,"parent_record_offset":568048,"ancestor_record_offset":567824,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":568100,"parent_record_offset":568076,"ancestor_record_offset":567824,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":569036,"parent_record_offset":569012,"ancestor_record_offset":567824,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":568628,"parent_record_offset":568600,"ancestor_record_offset":567824,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":568600,"parent_record_offset":568576,"ancestor_record_offset":567824,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":569084,"parent_record_offset":569060,"ancestor_record_offset":567824,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":569060,"parent_record_offset":569036,"ancestor_record_offset":567824,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":568576,"parent_record_offset":568552,"ancestor_record_offset":567824,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":569012,"parent_record_offset":568988,"ancestor_record_offset":567824,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":568988,"parent_record_offset":568732,"ancestor_record_offset":567824,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":569604,"parent_record_offset":569580,"ancestor_record_offset":567824,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":570604,"parent_record_offset":570580,"ancestor_record_offset":567824,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":568944,"parent_record_offset":568916,"ancestor_record_offset":567824,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":568048,"parent_record_offset":568020,"ancestor_record_offset":567824,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":570580,"parent_record_offset":570544,"ancestor_record_offset":567824,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":568216,"parent_record_offset":568128,"ancestor_record_offset":567824,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":568444,"parent_record_offset":567936,"ancestor_record_offset":567824,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":568240,"parent_record_offset":568216,"ancestor_record_offset":567824,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":570660,"parent_record_offset":570604,"ancestor_record_offset":567824,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":571128,"parent_record_offset":571076,"ancestor_record_offset":567824,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":569628,"parent_record_offset":569604,"ancestor_record_offset":567824,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":569652,"parent_record_offset":569628,"ancestor_record_offset":567824,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":568128,"parent_record_offset":568100,"ancestor_record_offset":567824,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":568184,"parent_record_offset":568160,"ancestor_record_offset":567824,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":568160,"parent_record_offset":568128,"ancestor_record_offset":567824,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":569580,"parent_record_offset":569548,"ancestor_record_offset":567824,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":571028,"parent_record_offset":571004,"ancestor_record_offset":567824,"inlinee":28073,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":571076,"parent_record_offset":571052,"ancestor_record_offset":567824,"inlinee":28014,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":569548,"parent_record_offset":569500,"ancestor_record_offset":567824,"inlinee":74407,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$7::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1630,"record_offset":569376,"parent_record_offset":569344,"ancestor_record_offset":567824,"inlinee":74405,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1630,"record_offset":571844,"parent_record_offset":571820,"ancestor_record_offset":567824,"inlinee":32780,"source_line":69,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1630,"record_offset":571892,"parent_record_offset":571868,"ancestor_record_offset":567824,"inlinee":32782,"source_line":69,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":572044,"parent_record_offset":572020,"ancestor_record_offset":567824,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":572020,"parent_record_offset":571996,"ancestor_record_offset":567824,"inlinee":6705,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":567900,"parent_record_offset":567864,"ancestor_record_offset":567824,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":570516,"parent_record_offset":570488,"ancestor_record_offset":567824,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":569500,"parent_record_offset":569468,"ancestor_record_offset":567824,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":570720,"parent_record_offset":570696,"ancestor_record_offset":567824,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":569344,"parent_record_offset":567936,"ancestor_record_offset":567824,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":571100,"parent_record_offset":571076,"ancestor_record_offset":567824,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$75::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":571916,"parent_record_offset":571892,"ancestor_record_offset":567824,"inlinee":6539,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":567992,"parent_record_offset":567936,"ancestor_record_offset":567824,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":569436,"parent_record_offset":569404,"ancestor_record_offset":567824,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":569404,"parent_record_offset":569376,"ancestor_record_offset":567824,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":569468,"parent_record_offset":569436,"ancestor_record_offset":567824,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":571052,"parent_record_offset":571028,"ancestor_record_offset":567824,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":567936,"parent_record_offset":567900,"ancestor_record_offset":567824,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":570544,"parent_record_offset":570516,"ancestor_record_offset":567824,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x122F9
// 首条位置: {"module":1630,"inlinee":74489}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":676176,"parent_record_offset":676144,"ancestor_record_offset":676144,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":676200,"parent_record_offset":676176,"ancestor_record_offset":676144,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":676224,"parent_record_offset":676200,"ancestor_record_offset":676144,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":676248,"parent_record_offset":676224,"ancestor_record_offset":676144,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1729,"record_offset":26024,"rva":24899552,"va":5393608672}
// 编译实例 RVAs: 0x17befe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t17868814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e80ea81001","size":5,"soft_fail":false,"target_va":5411478016,"va":5393609197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18528080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e8b048e5fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5393609579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14085da90
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16129283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e8fde209ff","size":5,"soft_fail":false,"target_va":5377481360,"va":5393610638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140887df0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15954709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e8eb8c0cff","size":5,"soft_fail":false,"target_va":5377654256,"va":5393608960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888d70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15950892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e8d49b0cff","size":5,"soft_fail":false,"target_va":5377658224,"va":5393609111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140889390
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15949885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e8c39f0cff","size":5,"soft_fail":false,"target_va":5377659792,"va":5393609672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140898f40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15885378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e8be9b0dff","size":5,"soft_fail":false,"target_va":5377724224,"va":5393609597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14143c450
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3681325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e8d3d3c7ff","size":5,"soft_fail":false,"target_va":5389927504,"va":5393608824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415d2890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2017478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e83a37e1ff","size":5,"soft_fail":false,"target_va":5391591568,"va":5393609041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415d4850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2010141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e8e353e1ff","size":5,"soft_fail":false,"target_va":5391599696,"va":5393609832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415d7e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1996493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e83389e1ff","size":5,"soft_fail":false,"target_va":5391613584,"va":5393610072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415d9c80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1988949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e8aba6e1ff","size":5,"soft_fail":false,"target_va":5391621248,"va":5393610192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415da540
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1986669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e893afe1ff","size":5,"soft_fail":false,"target_va":5391623488,"va":5393610152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415dbb60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1980725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e8cbc6e1ff","size":5,"soft_fail":false,"target_va":5391629152,"va":5393609872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415dce20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1976005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e83bd9e1ff","size":5,"soft_fail":false,"target_va":5391633952,"va":5393609952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415dfa00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1964613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e8bb05e2ff","size":5,"soft_fail":false,"target_va":5391645184,"va":5393609792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415e0c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1959877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e83b18e2ff","size":5,"soft_fail":false,"target_va":5391649840,"va":5393609712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415e8d60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1927005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e8a398e2ff","size":5,"soft_fail":false,"target_va":5391682912,"va":5393609912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415eb650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1916365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e833c2e2ff","size":5,"soft_fail":false,"target_va":5391693392,"va":5393609752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415ed160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1909717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e82bdce2ff","size":5,"soft_fail":false,"target_va":5391700320,"va":5393610032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415ee1b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1904878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e812efe2ff","size":5,"soft_fail":false,"target_va":5391704496,"va":5393609369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415ef070
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1901845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e8ebfae2ff","size":5,"soft_fail":false,"target_va":5391708272,"va":5393610112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f07b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1895773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e8a312e3ff","size":5,"soft_fail":false,"target_va":5391714224,"va":5393609992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f9b40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18066002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e852aa1301","size":5,"soft_fail":false,"target_va":5411674944,"va":5393608937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::de::_$LT$impl$u20$serde_json..value..Value$GT$::unexpected
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t17869365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393608672,"raw_hex":"e835aa1001","size":5,"soft_fail":false,"target_va":5411478544,"va":5393609174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::value::de::impl$2::deserialize_enum
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1729,"record_offset":26212,"parent_record_offset":26024,"ancestor_record_offset":26024,"inlinee":26604,"source_line":255,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1729,"record_offset":27240,"parent_record_offset":27216,"ancestor_record_offset":26024,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1729,"record_offset":27216,"parent_record_offset":27128,"ancestor_record_offset":26024,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1729,"record_offset":26392,"parent_record_offset":26368,"ancestor_record_offset":26024,"inlinee":6033,"source_line":1680,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1729,"record_offset":26448,"parent_record_offset":26424,"ancestor_record_offset":26024,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1729,"record_offset":26500,"parent_record_offset":26472,"ancestor_record_offset":26024,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1729,"record_offset":26524,"parent_record_offset":26500,"ancestor_record_offset":26024,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1729,"record_offset":27608,"parent_record_offset":27584,"ancestor_record_offset":26024,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1729,"record_offset":27156,"parent_record_offset":27128,"ancestor_record_offset":26024,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1729,"record_offset":27128,"parent_record_offset":27104,"ancestor_record_offset":26024,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1729,"record_offset":27656,"parent_record_offset":27632,"ancestor_record_offset":26024,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1729,"record_offset":27632,"parent_record_offset":27608,"ancestor_record_offset":26024,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1729,"record_offset":27104,"parent_record_offset":27080,"ancestor_record_offset":26024,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1729,"record_offset":27584,"parent_record_offset":27560,"ancestor_record_offset":26024,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1729,"record_offset":27052,"parent_record_offset":27024,"ancestor_record_offset":26024,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1729,"record_offset":27184,"parent_record_offset":27156,"ancestor_record_offset":26024,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1729,"record_offset":26472,"parent_record_offset":26448,"ancestor_record_offset":26024,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1729,"record_offset":26640,"parent_record_offset":26552,"ancestor_record_offset":26024,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1729,"record_offset":26996,"parent_record_offset":26212,"ancestor_record_offset":26024,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1729,"record_offset":26664,"parent_record_offset":26640,"ancestor_record_offset":26024,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1729,"record_offset":26928,"parent_record_offset":26868,"ancestor_record_offset":26024,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1729,"record_offset":26964,"parent_record_offset":26868,"ancestor_record_offset":26024,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1729,"record_offset":26552,"parent_record_offset":26524,"ancestor_record_offset":26024,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1729,"record_offset":26608,"parent_record_offset":26584,"ancestor_record_offset":26024,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1729,"record_offset":26584,"parent_record_offset":26552,"ancestor_record_offset":26024,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/color_edit.rs
//     evidence={"module":1729,"record_offset":26868,"parent_record_offset":26212,"ancestor_record_offset":26024,"inlinee":77843,"source_line":15,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1729,"record_offset":26900,"parent_record_offset":26868,"ancestor_record_offset":26024,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$17::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1729,"record_offset":26368,"parent_record_offset":26212,"ancestor_record_offset":26024,"inlinee":26606,"source_line":877,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1729,"record_offset":26424,"parent_record_offset":26212,"ancestor_record_offset":26024,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":10,"record_offset":3348,"rva":247248,"va":5368956368}
// 编译实例 RVAs: 0x3c5d0, 0x17be2b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003e0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368956368,"raw_hex":"e855130000","size":5,"soft_fail":false,"target_va":5368963232,"va":5368958278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b6100
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4692628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368956368,"raw_hex":"e8949a4700","size":5,"soft_fail":false,"target_va":5373649152,"va":5368956519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b7780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4698245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368956368,"raw_hex":"e885b04700","size":5,"soft_fail":false,"target_va":5373654912,"va":5368956662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404baab0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4710017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368956368,"raw_hex":"e881de4700","size":5,"soft_fail":false,"target_va":5373668016,"va":5368957994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404baf70
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4712601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368956368,"raw_hex":"e899e84700","size":5,"soft_fail":false,"target_va":5373669232,"va":5368956626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bc390
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4717007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368956368,"raw_hex":"e8cff94700","size":5,"soft_fail":false,"target_va":5373674384,"va":5368957372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bdc80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4722282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368956368,"raw_hex":"e86a0e4800","size":5,"soft_fail":false,"target_va":5373680768,"va":5368958481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404be300
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4725307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368956368,"raw_hex":"e83b1a4800","size":5,"soft_fail":false,"target_va":5373682432,"va":5368957120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404be900
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4726222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368956368,"raw_hex":"e8ce1d4800","size":5,"soft_fail":false,"target_va":5373683968,"va":5368957741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404c11a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4737279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368956368,"raw_hex":"e8ff484800","size":5,"soft_fail":false,"target_va":5373694368,"va":5368957084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b36350
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13141723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393605296,"raw_hex":"e8257937ff","size":5,"soft_fail":false,"target_va":5380465488,"va":5393607206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141580d20
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2348588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393605296,"raw_hex":"e8d429dcff","size":5,"soft_fail":false,"target_va":5391256864,"va":5393605447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141583160
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2339451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393605296,"raw_hex":"e8854ddcff","size":5,"soft_fail":false,"target_va":5391266144,"va":5393605590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141583a40
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2338511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393605296,"raw_hex":"e83151dcff","size":5,"soft_fail":false,"target_va":5391268416,"va":5393606922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141584ab0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2333393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393605296,"raw_hex":"e82f65dcff","size":5,"soft_fail":false,"target_va":5391272624,"va":5393606012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141586090
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2329190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393605296,"raw_hex":"e89a75dcff","size":5,"soft_fail":false,"target_va":5391278224,"va":5393607409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141586af0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2324679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393605296,"raw_hex":"e83987dcff","size":5,"soft_fail":false,"target_va":5391280880,"va":5393605554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141588150
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2319697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393605296,"raw_hex":"e8af9adcff","size":5,"soft_fail":false,"target_va":5391286608,"va":5393606300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14158cf00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2300178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393605296,"raw_hex":"e8eee6dcff","size":5,"soft_fail":false,"target_va":5391306496,"va":5393606669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14158dc20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: color_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2296197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393605296,"raw_hex":"e87bf6dcff","size":5,"soft_fail":false,"target_va":5391309856,"va":5393606048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=56
//     evidence={"assembly":"\tcallq\t42359661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368956368,"raw_hex":"e86d5b8602","size":5,"soft_fail":false,"target_va":5411318240,"va":5368958574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::error::Error::io
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42519125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368956368,"raw_hex":"e855ca8802","size":5,"soft_fail":false,"target_va":5411477664,"va":5368958534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$10::serialize_field
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":6320,"parent_record_offset":3348,"ancestor_record_offset":3348,"inlinee":5725,"source_line":794,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct_variant
//     owner=registry_dependency_definition; count=30
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":3632,"parent_record_offset":3348,"ancestor_record_offset":3348,"inlinee":5433,"source_line":420,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":4008,"parent_record_offset":3976,"ancestor_record_offset":3348,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":3876,"parent_record_offset":3848,"ancestor_record_offset":3348,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":3936,"parent_record_offset":3904,"ancestor_record_offset":3348,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":4056,"parent_record_offset":4032,"ancestor_record_offset":3348,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":4032,"parent_record_offset":4008,"ancestor_record_offset":3348,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":3904,"parent_record_offset":3876,"ancestor_record_offset":3348,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":10,"record_offset":3808,"parent_record_offset":3780,"ancestor_record_offset":3348,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":10,"record_offset":3976,"parent_record_offset":3808,"ancestor_record_offset":3348,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":10,"record_offset":3752,"parent_record_offset":3724,"ancestor_record_offset":3348,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":10,"record_offset":4096,"parent_record_offset":3808,"ancestor_record_offset":3348,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":10,"record_offset":3848,"parent_record_offset":3808,"ancestor_record_offset":3348,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":10,"record_offset":3780,"parent_record_offset":3752,"ancestor_record_offset":3348,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":10,"record_offset":10672,"parent_record_offset":10624,"ancestor_record_offset":3348,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":10,"record_offset":5292,"parent_record_offset":5268,"ancestor_record_offset":3348,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=38
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":3668,"parent_record_offset":3632,"ancestor_record_offset":3348,"inlinee":5420,"source_line":1849,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object_value
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":5324,"parent_record_offset":4736,"ancestor_record_offset":3348,"inlinee":5429,"source_line":1894,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":5804,"parent_record_offset":4736,"ancestor_record_offset":3348,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=registry_dependency_definition; count=30
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":4148,"parent_record_offset":3632,"ancestor_record_offset":3348,"inlinee":5182,"source_line":219,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=32
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":6344,"parent_record_offset":6320,"ancestor_record_offset":3348,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":10,"record_offset":3724,"parent_record_offset":3696,"ancestor_record_offset":3348,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":10,"record_offset":3696,"parent_record_offset":3668,"ancestor_record_offset":3348,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::color_edit::impl$1::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x834F
// 首条位置: {"module":593,"inlinee":33615}
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
// 完整结构及原始类型记录: color_edit.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::color_edit::ColorEdit
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::resources::history::color_edit::ColorEdit>", "constructor": "enum2$", "arguments": ["country_core::resources::history::color_edit::ColorEdit"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<enum2$<country_core::resources::history::color_edit::ColorEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["enum2$<country_core::resources::history::color_edit::ColorEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::Arc<enum2$<country_core::resources::history::color_edit::ColorEdit>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::Arc<enum2$<country_core::resources::history::color_edit::ColorEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$216::deserialize<enum2$<country_core::resources::history::color_edit::ColorEdit>,enum2$<serde_json::value::Value> >", "constructor": "serde::de::impls::impl$216::deserialize", "arguments": ["enum2$<country_core::resources::history::color_edit::ColorEdit>", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$54::fmt<enum2$<country_core::resources::history::color_edit::ColorEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::impl$54::fmt", "arguments": ["enum2$<country_core::resources::history::color_edit::ColorEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<alloc::sync::Arc<enum2$<country_core::resources::history::color_edit::ColorEdit>,alloc::alloc::Global> >", "constructor": "core::marker::PhantomData", "arguments": ["alloc::sync::Arc<enum2$<country_core::resources::history::color_edit::ColorEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$5::newtype_variant_seed<core::marker::PhantomData<alloc::sync::Arc<enum2$<country_core::resources::history::color_edit::ColorEdit>,alloc::alloc::Global> > >", "constructor": "serde_json::value::de::impl$5::newtype_variant_seed", "arguments": ["core::marker::PhantomData<alloc::sync::Arc<enum2$<country_core::resources::history::color_edit::ColorEdit>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::color_edit::ColorReplayCmd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: color_edit.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [资源/历史 JSON schema，不是内存布局] Color history serialized observations
// 全部实际键和数据形态: {"/commands/Color": {"observed_types": {"object": 114}, "object_instances": 114, "field_occurrences": {"Wall": 53, "Roof": 23, "Halftimber": 13, "Decorator": 10, "StairRailing": 7, "FlatRoof": 4, "StairLadder": 1, "Brightness": 3}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/02-demo4/history.json", "pointer": "/History/edit_chain/30/edits/0/Color"}, {"file": "assets/starting-builds/demo/02-demo4/history.json", "pointer": "/History/edit_chain/31/edits/0/Color"}]}, "/commands/Color/Brightness": {"observed_types": {"object": 3}, "object_instances": 3, "field_occurrences": {"decorator_id": 3, "from": 3, "to": 3}, "present_in_every_observed_object": ["decorator_id", "from", "to"], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/26-bath-house/history.json", "pointer": "/History/edit_chain/27/edits/0/Color/Brightness"}, {"file": "assets/starting-builds/full/26-bath-house/history.json", "pointer": "/History/edit_chain/28/edits/0/Color/Brightness"}]}, "/commands/Color/Brightness/decorator_id": {"observed_types": {"integer": 3}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [7, 10], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/26-bath-house/history.json", "pointer": "/History/edit_chain/27/edits/0/Color/Brightness/decorator_id"}, {"file": "assets/starting-builds/full/26-bath-house/history.json", "pointer": "/History/edit_chain/28/edits/0/Color/Brightness/decorator_id"}]}, "/commands/Color/Brightness/from": {"observed_types": {"number": 3}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [1.0, 1.0], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/26-bath-house/history.json", "pointer": "/History/edit_chain/27/edits/0/Color/Brightness/from"}, {"file": "assets/starting-builds/full/26-bath-house/history.json", "pointer": "/History/edit_chain/28/edits/0/Color/Brightness/from"}]}, "/commands/Color/Brightness/to": {"observed_types": {"number": 3}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [4.0, 4.0], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/26-bath-house/history.json", "pointer": "/History/edit_chain/27/edits/0/Color/Brightness/to"}, {"file": "assets/starting-builds/full/26-bath-house/history.json", "pointer": "/History/edit_chain/28/edits/0/Color/Brightness/to"}]}, "/commands/Color/Decorator": {"observed_types": {"object": 10}, "object_instances": 10, "field_occurrences": {"decorator_id": 10, "from": 10, "to": 10, "wall_id": 6}, "present_in_every_observed_object": ["decorator_id", "from", "to"], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/53/edits/0/Color/Decorator"}, {"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/75/edits/0/Color/Decorator"}]}, "/commands/Color/Decorator/decorator_id": {"observed_types": {"integer": 10}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [1, 15], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/53/edits/0/Color/Decorator/decorator_id"}, {"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/75/edits/0/Color/Decorator/decorator_id"}]}, "/commands/Color/Decorator/from": {"observed_types": {"integer": 10}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0, 100003], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/53/edits/0/Color/Decorator/from"}, {"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/75/edits/0/Color/Decorator/from"}]}, "/commands/Color/Decorator/to": {"observed_types": {"integer": 10}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [1, 100002], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/53/edits/0/Color/Decorator/to"}, {"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/75/edits/0/Color/Decorator/to"}]}, "/commands/Color/Decorator/wall_id": {"observed_types": {"integer": 6}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [3, 32], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/53/edits/0/Color/Decorator/wall_id"}, {"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/75/edits/0/Color/Decorator/wall_id"}]}, "/commands/Color/FlatRoof": {"observed_types": {"object": 4}, "object_instances": 4, "field_occurrences": {"wall_id": 4, "from": 4, "to": 4}, "present_in_every_observed_object": ["wall_id", "from", "to"], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/11-multi-level/history.json", "pointer": "/History/edit_chain/44/edits/0/Color/FlatRoof"}, {"file": "assets/starting-builds/full/20-green-platform/history.json", "pointer": "/History/edit_chain/29/edits/0/Color/FlatRoof"}]}, "/commands/Color/FlatRoof/from": {"observed_types": {"string": 3, "object": 1}, "object_instances": 1, "field_occurrences": {"Override": 1}, "present_in_every_observed_object": ["Override"], "array_length_counts": {}, "numeric_range": null, "observed_string_values": ["MatchingStoneOnly"], "sources": [{"file": "assets/starting-builds/full/11-multi-level/history.json", "pointer": "/History/edit_chain/44/edits/0/Color/FlatRoof/from"}, {"file": "assets/starting-builds/full/20-green-platform/history.json", "pointer": "/History/edit_chain/29/edits/0/Color/FlatRoof/from"}]}, "/commands/Color/FlatRoof/from/Override": {"observed_types": {"object": 1}, "object_instances": 1, "field_occurrences": {"color_id": 1, "material": 1, "pattern_id": 1}, "present_in_every_observed_object": ["color_id", "material", "pattern_id"], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/20-green-platform/history.json", "pointer": "/History/edit_chain/31/edits/0/Color/FlatRoof/from/Override"}]}, "/commands/Color/FlatRoof/from/Override/color_id": {"observed_types": {"integer": 1}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [6, 6], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/20-green-platform/history.json", "pointer": "/History/edit_chain/31/edits/0/Color/FlatRoof/from/Override/color_id"}]}, "/commands/Color/FlatRoof/from/Override/material": {"observed_types": {"string": 1}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": null, "observed_string_values": ["Wood"], "sources": [{"file": "assets/starting-builds/full/20-green-platform/history.json", "pointer": "/History/edit_chain/31/edits/0/Color/FlatRoof/from/Override/material"}]}, "/commands/Color/FlatRoof/from/Override/pattern_id": {"observed_types": {"integer": 1}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [2, 2], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/20-green-platform/history.json", "pointer": "/History/edit_chain/31/edits/0/Color/FlatRoof/from/Override/pattern_id"}]}, "/commands/Color/FlatRoof/to": {"observed_types": {"object": 4}, "object_instances": 4, "field_occurrences": {"Override": 4}, "present_in_every_observed_object": ["Override"], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/11-multi-level/history.json", "pointer": "/History/edit_chain/44/edits/0/Color/FlatRoof/to"}, {"file": "assets/starting-builds/full/20-green-platform/history.json", "pointer": "/History/edit_chain/29/edits/0/Color/FlatRoof/to"}]}, "/commands/Color/FlatRoof/to/Override": {"observed_types": {"object": 4}, "object_instances": 4, "field_occurrences": {"color_id": 4, "material": 4, "pattern_id": 4}, "present_in_every_observed_object": ["color_id", "material", "pattern_id"], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/11-multi-level/history.json", "pointer": "/History/edit_chain/44/edits/0/Color/FlatRoof/to/Override"}, {"file": "assets/starting-builds/full/20-green-platform/history.json", "pointer": "/History/edit_chain/29/edits/0/Color/FlatRoof/to/Override"}]}, "/commands/Color/FlatRoof/to/Override/color_id": {"observed_types": {"integer": 4}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [4, 6], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/11-multi-level/history.json", "pointer": "/History/edit_chain/44/edits/0/Color/FlatRoof/to/Override/color_id"}, {"file": "assets/starting-builds/full/20-green-platform/history.json", "pointer": "/History/edit_chain/29/edits/0/Color/FlatRoof/to/Override/color_id"}]}, "/commands/Color/FlatRoof/to/Override/material": {"observed_types": {"string": 4}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": null, "observed_string_values": ["Stone", "Wood"], "sources": [{"file": "assets/starting-builds/full/11-multi-level/history.json", "pointer": "/History/edit_chain/44/edits/0/Color/FlatRoof/to/Override/material"}, {"file": "assets/starting-builds/full/20-green-platform/history.json", "pointer": "/History/edit_chain/29/edits/0/Color/FlatRoof/to/Override/material"}]}, "/commands/Color/FlatRoof/to/Override/pattern_id": {"observed_types": {"integer": 4}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [1, 2], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/11-multi-level/history.json", "pointer": "/History/edit_chain/44/edits/0/Color/FlatRoof/to/Override/pattern_id"}, {"file": "assets/starting-builds/full/20-green-platform/history.json", "pointer": "/History/edit_chain/29/edits/0/Color/FlatRoof/to/Override/pattern_id"}]}, "/commands/Color/FlatRoof/wall_id": {"observed_types": {"integer": 4}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [3, 6], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/11-multi-level/history.json", "pointer": "/History/edit_chain/44/edits/0/Color/FlatRoof/wall_id"}, {"file": "assets/starting-builds/full/20-green-platform/history.json", "pointer": "/History/edit_chain/29/edits/0/Color/FlatRoof/wall_id"}]}, "/commands/Color/Halftimber": {"observed_types": {"object": 13}, "object_instances": 13, "field_occurrences": {"from": 13, "to": 13, "wall_id": 13}, "present_in_every_observed_object": ["from", "to", "wall_id"], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/8/edits/0/Color/Halftimber"}, {"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/77/edits/0/Color/Halftimber"}]}, "/commands/Color/Halftimber/from": {"observed_types": {"integer": 13}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0, 5], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/8/edits/0/Color/Halftimber/from"}, {"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/77/edits/0/Color/Halftimber/from"}]}, "/commands/Color/Halftimber/to": {"observed_types": {"integer": 13}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [1, 6], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/8/edits/0/Color/Halftimber/to"}, {"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/77/edits/0/Color/Halftimber/to"}]}, "/commands/Color/Halftimber/wall_id": {"observed_types": {"integer": 13}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [3, 47], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/8/edits/0/Color/Halftimber/wall_id"}, {"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/77/edits/0/Color/Halftimber/wall_id"}]}, "/commands/Color/Roof": {"observed_types": {"object": 23}, "object_instances": 23, "field_occurrences": {"from": 23, "to": 23, "wall_id": 23}, "present_in_every_observed_object": ["from", "to", "wall_id"], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/3/edits/0/Color/Roof"}, {"file": "assets/starting-builds/full/02-gate/history.json", "pointer": "/History/edit_chain/8/edits/0/Color/Roof"}]}, "/commands/Color/Roof/from": {"observed_types": {"integer": 23}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0, 7], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/3/edits/0/Color/Roof/from"}, {"file": "assets/starting-builds/full/02-gate/history.json", "pointer": "/History/edit_chain/8/edits/0/Color/Roof/from"}]}, "/commands/Color/Roof/to": {"observed_types": {"integer": 23}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [1, 7], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/3/edits/0/Color/Roof/to"}, {"file": "assets/starting-builds/full/02-gate/history.json", "pointer": "/History/edit_chain/8/edits/0/Color/Roof/to"}]}, "/commands/Color/Roof/wall_id": {"observed_types": {"integer": 23}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [3, 47], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/3/edits/0/Color/Roof/wall_id"}, {"file": "assets/starting-builds/full/02-gate/history.json", "pointer": "/History/edit_chain/8/edits/0/Color/Roof/wall_id"}]}, "/commands/Color/StairLadder": {"observed_types": {"object": 1}, "object_instances": 1, "field_occurrences": {"decorator_id": 1, "from": 1, "to": 1}, "present_in_every_observed_object": ["decorator_id", "from", "to"], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/15-lake-house/history.json", "pointer": "/History/edit_chain/18/edits/0/Color/StairLadder"}]}, "/commands/Color/StairLadder/decorator_id": {"observed_types": {"integer": 1}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0, 0], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/15-lake-house/history.json", "pointer": "/History/edit_chain/18/edits/0/Color/StairLadder/decorator_id"}]}, "/commands/Color/StairLadder/from": {"observed_types": {"integer": 1}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0, 0], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/15-lake-house/history.json", "pointer": "/History/edit_chain/18/edits/0/Color/StairLadder/from"}]}, "/commands/Color/StairLadder/to": {"observed_types": {"integer": 1}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [3, 3], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/15-lake-house/history.json", "pointer": "/History/edit_chain/18/edits/0/Color/StairLadder/to"}]}, "/commands/Color/StairRailing": {"observed_types": {"object": 7}, "object_instances": 7, "field_occurrences": {"decorator_id": 7, "from": 7, "to": 7}, "present_in_every_observed_object": ["decorator_id", "from", "to"], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/04-small-bridge/history.json", "pointer": "/History/edit_chain/77/edits/0/Color/StairRailing"}, {"file": "assets/starting-builds/full/08-water-bridge/history.json", "pointer": "/History/edit_chain/79/edits/0/Color/StairRailing"}]}, "/commands/Color/StairRailing/decorator_id": {"observed_types": {"integer": 7}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [1, 16], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/04-small-bridge/history.json", "pointer": "/History/edit_chain/77/edits/0/Color/StairRailing/decorator_id"}, {"file": "assets/starting-builds/full/08-water-bridge/history.json", "pointer": "/History/edit_chain/79/edits/0/Color/StairRailing/decorator_id"}]}, "/commands/Color/StairRailing/from": {"observed_types": {"string": 7}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": null, "observed_string_values": ["Low", "None", "Wooden"], "sources": [{"file": "assets/starting-builds/full/04-small-bridge/history.json", "pointer": "/History/edit_chain/77/edits/0/Color/StairRailing/from"}, {"file": "assets/starting-builds/full/08-water-bridge/history.json", "pointer": "/History/edit_chain/79/edits/0/Color/StairRailing/from"}]}, "/commands/Color/StairRailing/to": {"observed_types": {"string": 7}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": null, "observed_string_values": ["High", "Low", "None", "Wooden"], "sources": [{"file": "assets/starting-builds/full/04-small-bridge/history.json", "pointer": "/History/edit_chain/77/edits/0/Color/StairRailing/to"}, {"file": "assets/starting-builds/full/08-water-bridge/history.json", "pointer": "/History/edit_chain/79/edits/0/Color/StairRailing/to"}]}, "/commands/Color/Wall": {"observed_types": {"object": 53}, "object_instances": 53, "field_occurrences": {"from": 53, "to": 53, "wall_id": 53}, "present_in_every_observed_object": ["from", "to", "wall_id"], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/02-demo4/history.json", "pointer": "/History/edit_chain/30/edits/0/Color/Wall"}, {"file": "assets/starting-builds/demo/02-demo4/history.json", "pointer": "/History/edit_chain/31/edits/0/Color/Wall"}]}, "/commands/Color/Wall/from": {"observed_types": {"integer": 53}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0, 9], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/02-demo4/history.json", "pointer": "/History/edit_chain/30/edits/0/Color/Wall/from"}, {"file": "assets/starting-builds/demo/02-demo4/history.json", "pointer": "/History/edit_chain/31/edits/0/Color/Wall/from"}]}, "/commands/Color/Wall/to": {"observed_types": {"integer": 53}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [1, 11], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/02-demo4/history.json", "pointer": "/History/edit_chain/30/edits/0/Color/Wall/to"}, {"file": "assets/starting-builds/demo/02-demo4/history.json", "pointer": "/History/edit_chain/31/edits/0/Color/Wall/to"}]}, "/commands/Color/Wall/wall_id": {"observed_types": {"integer": 53}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [1, 108], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/02-demo4/history.json", "pointer": "/History/edit_chain/30/edits/0/Color/Wall/wall_id"}, {"file": "assets/starting-builds/demo/02-demo4/history.json", "pointer": "/History/edit_chain/31/edits/0/Color/Wall/wall_id"}]}}
// offsets / Rust numeric types / missing defaults = unknown。
// [已有模块证据关联；不自动成为字段事实] evidence/pdb-structure/type-layout-coverage.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::history::color_edit::ColorEdit", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: color_edit.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::history::replay_mode::HoldToFastReplay> >,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::history::replay_mode::HoldToFastReplay> >", "bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>", "bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>", "bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::history::replay_mode::HoldToFastReplay> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::reader::EventReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::event::reader::EventReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: 37b853ebdb7f2331fa50d43e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::color_edit::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::history::color_edit::_::impl$0::deserialize::__Field> > @ 0x15d28d6 -> 0x17bf8a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: f598455fd011e620f122c3bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::history::color_edit::_::impl$0::deserialize::__Field> >; module=1630, inline_record=484112; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: d142d409f7e5377efc2a500f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::color_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1729, inline_record=26868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 信息 ID: e9b435a28c4b87363bd3a1b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=718120; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$10::deserialize
// 信息 ID: 455e0279567d773e9fccaf1e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=675972; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$11::visit_map
// 信息 ID: 2c6f359f3825d5b6a18fa026
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=674420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$12::visit_str
// 信息 ID: 7648aa30f35b8c2fa485cbba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=695752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$13::deserialize
// 信息 ID: 287795c7571321ed842531c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=695580; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$14::visit_map
// 信息 ID: fe0e4a69b432f454d0a41c37
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=694028; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$15::visit_str
// 信息 ID: 0103e1c9812b81fa49d8ef61
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=708444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$16::deserialize
// 信息 ID: 73c5b010c680393f62c6d003
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=708272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$17::visit_map
// 信息 ID: 8d82acbf46eb8b627dcabb45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=706720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$18::visit_str
// 信息 ID: a55016995c21f724270a5e76
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=726684; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$19::deserialize
// 信息 ID: 3c90d78b6ee2ca16c5b8c3d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=726512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 信息 ID: d8e0cdb6f63537e93987b7c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=717948; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$20::visit_map
// 信息 ID: f18076bb13d834a9cd488565
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=724960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$21::visit_str
// 信息 ID: 7f1bfdf8aaeacd64491fc275
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=579372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$22::deserialize
// 信息 ID: 62db583c0ab2a9d6df0855b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=579200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$23::visit_map
// 信息 ID: 8fe5147f004f2bada25922d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=577644; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$24::visit_str
// 信息 ID: 583a94693bd8449d277ba0a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=554112; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$25::deserialize
// 信息 ID: a843d5c4d5f47dc330e6f98f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=553940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$26::visit_map
// 信息 ID: d34fcce9d1925749f64846d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=552388; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$27::visit_str
// 信息 ID: a4a1ec35cac24236aec8dd0a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=511156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$28::deserialize
// 信息 ID: d87d52c7081930512713f443
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=510984; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$29::visit_map
// 信息 ID: cd1e43f87b7e088175511b32
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=509428; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 信息 ID: 065aba231dc2884a1c21bc5b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=716396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$30::visit_str
// 信息 ID: ebf56f15828172a8446ac1b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=558884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$31::deserialize
// 信息 ID: a60821fe9750c1016d8cef61
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=558712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$32::visit_map
// 信息 ID: 8eaf617065c6f1d5518975d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=557160; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$33::visit_str
// 信息 ID: cee6f6106b7bb3c6ee113ca2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=611744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$34::deserialize
// 信息 ID: 7027eb4680f883a009b59f01
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=611572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$35::visit_map
// 信息 ID: 73751715c9cc5162864d03da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=610016; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$36::visit_str
// 信息 ID: 612fb8cec15e5b57d2b6ccb3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=601952; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$37::deserialize
// 信息 ID: 132b89a6788e1cc1f30111bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=601780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$38::visit_map
// 信息 ID: 2c90c5ec1e792a7e5c71c6a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=600228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$39::visit_str
// 信息 ID: e5bc73972181ce42de89429e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=538960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str
// 信息 ID: 4ca33bb8ae01a63933d55f6d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=737888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$40::deserialize
// 信息 ID: 824e3d2c7cc13ee7638ad7a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=538788; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$41::visit_map
// 信息 ID: 4fa43ba9cbcacf8e1968ae07
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=537232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$4::deserialize
// 信息 ID: 23184162eefc7a269b4b2652
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=737716; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$5::visit_map
// 信息 ID: eba8812b26b001d73c843369
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=736164; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str
// 信息 ID: 217e1e482d85769bc62cf6e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=569548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$7::deserialize
// 信息 ID: 8311f002bbf8bcd90f7db947
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=569376; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$8::visit_map
// 信息 ID: a8393879d14057d27a862f41
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=567824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str
// 信息 ID: b479e222ff8dcaef363dad33
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=676144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 信息 ID: 996d8d7ce583586a4da53694
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::resources::history::color_edit::_::impl$0::deserialize -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impls::impl$216::deserialize<enum2$<country_core::resources::history::color_edit::ColorEdit>,enum2$<serde_json::value::Value> > @ 0xb5d3dc -> 0x17befe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: 8d6f174af30b2d46fb7027b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::history::color_edit::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::color_edit::impl$1::fmt
// 信息 ID: e1d46922579a05700621e0be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::sync::impl$54::fmt<enum2$<country_core::resources::history::color_edit::ColorEdit>,alloc::alloc::Global>; module=593, inline_record=58048; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
