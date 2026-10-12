// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/history/roof_edit.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: ed4409191b93a4204c23034aa274cd1e8b55c6c9a50571a9ba446a4de8fb910c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: roof_edit.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 86, 564, 586, 593, 1630
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
// 函数符号: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":586,"record_offset":39684,"rva":11768528,"va":5380477648}
// 编译实例 RVAs: 0xb392d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f9e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31197752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380477648,"raw_hex":"e8380adc01","size":5,"soft_fail":false,"target_va":5411675680,"va":5380477923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":586,"record_offset":39880,"parent_record_offset":39684,"ancestor_record_offset":39684,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":586,"record_offset":39908,"parent_record_offset":39880,"ancestor_record_offset":39684,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":586,"record_offset":39936,"parent_record_offset":39908,"ancestor_record_offset":39684,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":586,"record_offset":39964,"parent_record_offset":39936,"ancestor_record_offset":39684,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12264
// 首条位置: {"module":1630,"inlinee":74340}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::value::impl$26::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":485128,"parent_record_offset":485104,"ancestor_record_offset":485104,"inlinee":74331,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":485376,"parent_record_offset":485352,"ancestor_record_offset":485104,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":485352,"parent_record_offset":485300,"ancestor_record_offset":485104,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":485324,"parent_record_offset":485300,"ancestor_record_offset":485104,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":485300,"parent_record_offset":485276,"ancestor_record_offset":485104,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":485276,"parent_record_offset":485256,"ancestor_record_offset":485104,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":485216,"parent_record_offset":485184,"ancestor_record_offset":485104,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":485184,"parent_record_offset":485156,"ancestor_record_offset":485104,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$26::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1630,"record_offset":485156,"parent_record_offset":485128,"ancestor_record_offset":485104,"inlinee":74332,"source_line":615,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8284
// 首条位置: {"module":586,"inlinee":33412}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":586,"record_offset":36492,"parent_record_offset":36432,"ancestor_record_offset":36432,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":586,"record_offset":36528,"parent_record_offset":36432,"ancestor_record_offset":36432,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":586,"record_offset":36464,"parent_record_offset":36432,"ancestor_record_offset":36432,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":586,"record_offset":40464,"rva":11768832,"va":5380477952}
// 编译实例 RVAs: 0xb39400
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":586,"record_offset":40680,"parent_record_offset":40464,"ancestor_record_offset":40464,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":586,"record_offset":40704,"parent_record_offset":40680,"ancestor_record_offset":40464,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":586,"record_offset":40728,"parent_record_offset":40704,"ancestor_record_offset":40464,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":586,"record_offset":40752,"parent_record_offset":40728,"ancestor_record_offset":40464,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$10::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12314
// 首条位置: {"module":1630,"inlinee":74516}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":713240,"parent_record_offset":713212,"ancestor_record_offset":713212,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":714004,"parent_record_offset":713980,"ancestor_record_offset":713212,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":713980,"parent_record_offset":713900,"ancestor_record_offset":713212,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":713940,"parent_record_offset":713900,"ancestor_record_offset":713212,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":713900,"parent_record_offset":713864,"ancestor_record_offset":713212,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":713864,"parent_record_offset":713828,"ancestor_record_offset":713212,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":713436,"parent_record_offset":713412,"ancestor_record_offset":713212,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":713756,"parent_record_offset":713336,"ancestor_record_offset":713212,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":713460,"parent_record_offset":713436,"ancestor_record_offset":713212,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":713484,"parent_record_offset":713460,"ancestor_record_offset":713212,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":713412,"parent_record_offset":713380,"ancestor_record_offset":713212,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/roof_edit.rs
//     evidence={"module":1630,"record_offset":713380,"parent_record_offset":713336,"ancestor_record_offset":713212,"inlinee":33432,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":713336,"parent_record_offset":713304,"ancestor_record_offset":713212,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":713272,"parent_record_offset":713240,"ancestor_record_offset":713212,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":713304,"parent_record_offset":713272,"ancestor_record_offset":713212,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$11::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12312
// 首条位置: {"module":1630,"inlinee":74514}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":714596,"parent_record_offset":711660,"ancestor_record_offset":711660,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":714872,"parent_record_offset":711660,"ancestor_record_offset":711660,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":715604,"parent_record_offset":711660,"ancestor_record_offset":711660,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":711700,"parent_record_offset":711660,"ancestor_record_offset":711660,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":714320,"parent_record_offset":711660,"ancestor_record_offset":711660,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":712516,"parent_record_offset":712492,"ancestor_record_offset":711660,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":712492,"parent_record_offset":712436,"ancestor_record_offset":711660,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":711856,"parent_record_offset":711828,"ancestor_record_offset":711660,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":711912,"parent_record_offset":711884,"ancestor_record_offset":711660,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":711936,"parent_record_offset":711912,"ancestor_record_offset":711660,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":712872,"parent_record_offset":712848,"ancestor_record_offset":711660,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":712464,"parent_record_offset":712436,"ancestor_record_offset":711660,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":712436,"parent_record_offset":712412,"ancestor_record_offset":711660,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":712920,"parent_record_offset":712896,"ancestor_record_offset":711660,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":712896,"parent_record_offset":712872,"ancestor_record_offset":711660,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":712412,"parent_record_offset":712388,"ancestor_record_offset":711660,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":712848,"parent_record_offset":712824,"ancestor_record_offset":711660,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":712824,"parent_record_offset":712568,"ancestor_record_offset":711660,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":713436,"parent_record_offset":713412,"ancestor_record_offset":711660,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":714436,"parent_record_offset":714412,"ancestor_record_offset":711660,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":712780,"parent_record_offset":712752,"ancestor_record_offset":711660,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":711884,"parent_record_offset":711856,"ancestor_record_offset":711660,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":714412,"parent_record_offset":714376,"ancestor_record_offset":711660,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":712052,"parent_record_offset":711964,"ancestor_record_offset":711660,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":712280,"parent_record_offset":711772,"ancestor_record_offset":711660,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":712076,"parent_record_offset":712052,"ancestor_record_offset":711660,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":714492,"parent_record_offset":714436,"ancestor_record_offset":711660,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":713460,"parent_record_offset":713436,"ancestor_record_offset":711660,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":713484,"parent_record_offset":713460,"ancestor_record_offset":711660,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":711964,"parent_record_offset":711936,"ancestor_record_offset":711660,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":712020,"parent_record_offset":711996,"ancestor_record_offset":711660,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":711996,"parent_record_offset":711964,"ancestor_record_offset":711660,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":713412,"parent_record_offset":713380,"ancestor_record_offset":711660,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$10::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/roof_edit.rs
//     evidence={"module":1630,"record_offset":713212,"parent_record_offset":713180,"ancestor_record_offset":711660,"inlinee":74516,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/roof_edit.rs
//     evidence={"module":1630,"record_offset":713380,"parent_record_offset":713336,"ancestor_record_offset":711660,"inlinee":33432,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":1630,"record_offset":715628,"parent_record_offset":715604,"ancestor_record_offset":711660,"inlinee":29592,"source_line":336,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1630,"record_offset":715476,"parent_record_offset":715452,"ancestor_record_offset":711660,"inlinee":32780,"source_line":69,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1630,"record_offset":715524,"parent_record_offset":715500,"ancestor_record_offset":711660,"inlinee":32782,"source_line":69,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":715676,"parent_record_offset":715652,"ancestor_record_offset":711660,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":715652,"parent_record_offset":715628,"ancestor_record_offset":711660,"inlinee":6675,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":715764,"parent_record_offset":715740,"ancestor_record_offset":711660,"inlinee":6705,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":711736,"parent_record_offset":711700,"ancestor_record_offset":711660,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":714348,"parent_record_offset":714320,"ancestor_record_offset":711660,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":713336,"parent_record_offset":713304,"ancestor_record_offset":711660,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":714552,"parent_record_offset":714528,"ancestor_record_offset":711660,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":713180,"parent_record_offset":711772,"ancestor_record_offset":711660,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$75::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":715548,"parent_record_offset":715524,"ancestor_record_offset":711660,"inlinee":6539,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":711828,"parent_record_offset":711772,"ancestor_record_offset":711660,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":713272,"parent_record_offset":713240,"ancestor_record_offset":711660,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":713240,"parent_record_offset":713212,"ancestor_record_offset":711660,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":713304,"parent_record_offset":713272,"ancestor_record_offset":711660,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":715500,"parent_record_offset":715476,"ancestor_record_offset":711660,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":711772,"parent_record_offset":711736,"ancestor_record_offset":711660,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":714376,"parent_record_offset":714348,"ancestor_record_offset":711660,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$12::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x829A
// 首条位置: {"module":1630,"inlinee":33434}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":626076,"parent_record_offset":626044,"ancestor_record_offset":626044,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":626100,"parent_record_offset":626076,"ancestor_record_offset":626044,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":626124,"parent_record_offset":626100,"ancestor_record_offset":626044,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":626148,"parent_record_offset":626124,"ancestor_record_offset":626044,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$13::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x122D8
// 首条位置: {"module":1630,"inlinee":74456}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":625900,"parent_record_offset":625872,"ancestor_record_offset":625872,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":626668,"parent_record_offset":626644,"ancestor_record_offset":625872,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":626644,"parent_record_offset":626564,"ancestor_record_offset":625872,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":626604,"parent_record_offset":626564,"ancestor_record_offset":625872,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":626564,"parent_record_offset":626528,"ancestor_record_offset":625872,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":626528,"parent_record_offset":626492,"ancestor_record_offset":625872,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":626100,"parent_record_offset":626076,"ancestor_record_offset":625872,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":626420,"parent_record_offset":625996,"ancestor_record_offset":625872,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":626124,"parent_record_offset":626100,"ancestor_record_offset":625872,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":626148,"parent_record_offset":626124,"ancestor_record_offset":625872,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":626076,"parent_record_offset":626044,"ancestor_record_offset":625872,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$12::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/roof_edit.rs
//     evidence={"module":1630,"record_offset":626044,"parent_record_offset":625996,"ancestor_record_offset":625872,"inlinee":33434,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":625996,"parent_record_offset":625964,"ancestor_record_offset":625872,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":625932,"parent_record_offset":625900,"ancestor_record_offset":625872,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":625964,"parent_record_offset":625932,"ancestor_record_offset":625872,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$14::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x122D6
// 首条位置: {"module":1630,"inlinee":74454}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":627260,"parent_record_offset":624320,"ancestor_record_offset":624320,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":627536,"parent_record_offset":624320,"ancestor_record_offset":624320,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":628268,"parent_record_offset":624320,"ancestor_record_offset":624320,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":624360,"parent_record_offset":624320,"ancestor_record_offset":624320,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":626984,"parent_record_offset":624320,"ancestor_record_offset":624320,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":625176,"parent_record_offset":625152,"ancestor_record_offset":624320,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":625152,"parent_record_offset":625096,"ancestor_record_offset":624320,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":624516,"parent_record_offset":624488,"ancestor_record_offset":624320,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":624572,"parent_record_offset":624544,"ancestor_record_offset":624320,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":624596,"parent_record_offset":624572,"ancestor_record_offset":624320,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":625532,"parent_record_offset":625508,"ancestor_record_offset":624320,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":625124,"parent_record_offset":625096,"ancestor_record_offset":624320,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":625096,"parent_record_offset":625072,"ancestor_record_offset":624320,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":625580,"parent_record_offset":625556,"ancestor_record_offset":624320,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":625556,"parent_record_offset":625532,"ancestor_record_offset":624320,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":625072,"parent_record_offset":625048,"ancestor_record_offset":624320,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":625508,"parent_record_offset":625484,"ancestor_record_offset":624320,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":625484,"parent_record_offset":625228,"ancestor_record_offset":624320,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":626100,"parent_record_offset":626076,"ancestor_record_offset":624320,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":627100,"parent_record_offset":627076,"ancestor_record_offset":624320,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":625440,"parent_record_offset":625412,"ancestor_record_offset":624320,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":624544,"parent_record_offset":624516,"ancestor_record_offset":624320,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":627076,"parent_record_offset":627040,"ancestor_record_offset":624320,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":624712,"parent_record_offset":624624,"ancestor_record_offset":624320,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":624940,"parent_record_offset":624432,"ancestor_record_offset":624320,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":624736,"parent_record_offset":624712,"ancestor_record_offset":624320,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":627156,"parent_record_offset":627100,"ancestor_record_offset":624320,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":626124,"parent_record_offset":626100,"ancestor_record_offset":624320,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":626148,"parent_record_offset":626124,"ancestor_record_offset":624320,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":624624,"parent_record_offset":624596,"ancestor_record_offset":624320,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":624680,"parent_record_offset":624656,"ancestor_record_offset":624320,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":624656,"parent_record_offset":624624,"ancestor_record_offset":624320,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":626076,"parent_record_offset":626044,"ancestor_record_offset":624320,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$12::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/roof_edit.rs
//     evidence={"module":1630,"record_offset":626044,"parent_record_offset":625996,"ancestor_record_offset":624320,"inlinee":33434,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$13::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/roof_edit.rs
//     evidence={"module":1630,"record_offset":625872,"parent_record_offset":625840,"ancestor_record_offset":624320,"inlinee":74456,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":1630,"record_offset":628292,"parent_record_offset":628268,"ancestor_record_offset":624320,"inlinee":29592,"source_line":336,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1630,"record_offset":628140,"parent_record_offset":628116,"ancestor_record_offset":624320,"inlinee":32780,"source_line":69,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1630,"record_offset":628188,"parent_record_offset":628164,"ancestor_record_offset":624320,"inlinee":32782,"source_line":69,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":628340,"parent_record_offset":628316,"ancestor_record_offset":624320,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":628316,"parent_record_offset":628292,"ancestor_record_offset":624320,"inlinee":6675,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":628428,"parent_record_offset":628404,"ancestor_record_offset":624320,"inlinee":6705,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":624396,"parent_record_offset":624360,"ancestor_record_offset":624320,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":627012,"parent_record_offset":626984,"ancestor_record_offset":624320,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":625996,"parent_record_offset":625964,"ancestor_record_offset":624320,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":627216,"parent_record_offset":627192,"ancestor_record_offset":624320,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":625840,"parent_record_offset":624432,"ancestor_record_offset":624320,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$75::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":628212,"parent_record_offset":628188,"ancestor_record_offset":624320,"inlinee":6539,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":624488,"parent_record_offset":624432,"ancestor_record_offset":624320,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":625932,"parent_record_offset":625900,"ancestor_record_offset":624320,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":625900,"parent_record_offset":625872,"ancestor_record_offset":624320,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":625964,"parent_record_offset":625932,"ancestor_record_offset":624320,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":628164,"parent_record_offset":628140,"ancestor_record_offset":624320,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":624432,"parent_record_offset":624396,"ancestor_record_offset":624320,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":627040,"parent_record_offset":627012,"ancestor_record_offset":624320,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x122C6
// 首条位置: {"module":1630,"inlinee":74438}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":606740,"parent_record_offset":606712,"ancestor_record_offset":606712,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":607032,"parent_record_offset":607008,"ancestor_record_offset":606712,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":607008,"parent_record_offset":606952,"ancestor_record_offset":606712,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":606980,"parent_record_offset":606952,"ancestor_record_offset":606712,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":606952,"parent_record_offset":606928,"ancestor_record_offset":606712,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":606928,"parent_record_offset":606904,"ancestor_record_offset":606712,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":606856,"parent_record_offset":606824,"ancestor_record_offset":606712,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":606824,"parent_record_offset":606796,"ancestor_record_offset":606712,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":606768,"parent_record_offset":606740,"ancestor_record_offset":606712,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":606796,"parent_record_offset":606768,"ancestor_record_offset":606712,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x122C4
// 首条位置: {"module":1630,"inlinee":74436}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":607884,"parent_record_offset":605168,"ancestor_record_offset":605168,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":608976,"parent_record_offset":605168,"ancestor_record_offset":605168,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":609344,"parent_record_offset":605168,"ancestor_record_offset":605168,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":605208,"parent_record_offset":605168,"ancestor_record_offset":605168,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":607608,"parent_record_offset":605168,"ancestor_record_offset":605168,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":606020,"parent_record_offset":605996,"ancestor_record_offset":605168,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":605996,"parent_record_offset":605940,"ancestor_record_offset":605168,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":605360,"parent_record_offset":605332,"ancestor_record_offset":605168,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":605416,"parent_record_offset":605388,"ancestor_record_offset":605168,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":605440,"parent_record_offset":605416,"ancestor_record_offset":605168,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":606376,"parent_record_offset":606352,"ancestor_record_offset":605168,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":605968,"parent_record_offset":605940,"ancestor_record_offset":605168,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":605940,"parent_record_offset":605916,"ancestor_record_offset":605168,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":606424,"parent_record_offset":606400,"ancestor_record_offset":605168,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":606400,"parent_record_offset":606376,"ancestor_record_offset":605168,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":605916,"parent_record_offset":605892,"ancestor_record_offset":605168,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":606352,"parent_record_offset":606328,"ancestor_record_offset":605168,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":606328,"parent_record_offset":606072,"ancestor_record_offset":605168,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":607724,"parent_record_offset":607700,"ancestor_record_offset":605168,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":606284,"parent_record_offset":606256,"ancestor_record_offset":605168,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":605388,"parent_record_offset":605360,"ancestor_record_offset":605168,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":607700,"parent_record_offset":607664,"ancestor_record_offset":605168,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":605556,"parent_record_offset":605468,"ancestor_record_offset":605168,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":605784,"parent_record_offset":605280,"ancestor_record_offset":605168,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":605580,"parent_record_offset":605556,"ancestor_record_offset":605168,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":607780,"parent_record_offset":607724,"ancestor_record_offset":605168,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":607336,"parent_record_offset":605280,"ancestor_record_offset":605168,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":608640,"parent_record_offset":608588,"ancestor_record_offset":605168,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":605468,"parent_record_offset":605440,"ancestor_record_offset":605168,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":605524,"parent_record_offset":605500,"ancestor_record_offset":605168,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":605500,"parent_record_offset":605468,"ancestor_record_offset":605168,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":608540,"parent_record_offset":608516,"ancestor_record_offset":605168,"inlinee":28073,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":608588,"parent_record_offset":608564,"ancestor_record_offset":605168,"inlinee":28014,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/roof_edit.rs
//     evidence={"module":1630,"record_offset":606712,"parent_record_offset":606684,"ancestor_record_offset":605168,"inlinee":74438,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":1630,"record_offset":609268,"parent_record_offset":609244,"ancestor_record_offset":605168,"inlinee":29592,"source_line":336,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1630,"record_offset":608148,"parent_record_offset":608124,"ancestor_record_offset":605168,"inlinee":32780,"source_line":69,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1630,"record_offset":608196,"parent_record_offset":608172,"ancestor_record_offset":605168,"inlinee":32782,"source_line":69,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":609416,"parent_record_offset":609392,"ancestor_record_offset":605168,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":609504,"parent_record_offset":609480,"ancestor_record_offset":605168,"inlinee":6705,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":609392,"parent_record_offset":609368,"ancestor_record_offset":605168,"inlinee":5746,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":605244,"parent_record_offset":605208,"ancestor_record_offset":605168,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":607636,"parent_record_offset":607608,"ancestor_record_offset":605168,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":606824,"parent_record_offset":606796,"ancestor_record_offset":605168,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":607840,"parent_record_offset":607816,"ancestor_record_offset":605168,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":606684,"parent_record_offset":605280,"ancestor_record_offset":605168,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":608612,"parent_record_offset":608588,"ancestor_record_offset":605168,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$75::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":608220,"parent_record_offset":608196,"ancestor_record_offset":605168,"inlinee":6539,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":605332,"parent_record_offset":605280,"ancestor_record_offset":605168,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":606768,"parent_record_offset":606740,"ancestor_record_offset":605168,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":606740,"parent_record_offset":606712,"ancestor_record_offset":605168,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":606796,"parent_record_offset":606768,"ancestor_record_offset":605168,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":608172,"parent_record_offset":608148,"ancestor_record_offset":605168,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":605280,"parent_record_offset":605244,"ancestor_record_offset":605168,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":607664,"parent_record_offset":607636,"ancestor_record_offset":605168,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":586,"record_offset":41292,"rva":11769072,"va":5380478192}
// 编译实例 RVAs: 0xb394f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":586,"record_offset":41508,"parent_record_offset":41292,"ancestor_record_offset":41292,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":586,"record_offset":41532,"parent_record_offset":41508,"ancestor_record_offset":41292,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":586,"record_offset":41556,"parent_record_offset":41532,"ancestor_record_offset":41292,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":586,"record_offset":41580,"parent_record_offset":41556,"ancestor_record_offset":41292,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$4::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12282
// 首条位置: {"module":1630,"inlinee":74370}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":529160,"parent_record_offset":529132,"ancestor_record_offset":529132,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":529452,"parent_record_offset":529428,"ancestor_record_offset":529132,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":529428,"parent_record_offset":529372,"ancestor_record_offset":529132,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":529400,"parent_record_offset":529372,"ancestor_record_offset":529132,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":529372,"parent_record_offset":529348,"ancestor_record_offset":529132,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":529348,"parent_record_offset":529324,"ancestor_record_offset":529132,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":529276,"parent_record_offset":529244,"ancestor_record_offset":529132,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":529244,"parent_record_offset":529216,"ancestor_record_offset":529132,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":529188,"parent_record_offset":529160,"ancestor_record_offset":529132,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":529216,"parent_record_offset":529188,"ancestor_record_offset":529132,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$5::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12280
// 首条位置: {"module":1630,"inlinee":74368}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":530304,"parent_record_offset":527588,"ancestor_record_offset":527588,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":531396,"parent_record_offset":527588,"ancestor_record_offset":527588,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":531764,"parent_record_offset":527588,"ancestor_record_offset":527588,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":527628,"parent_record_offset":527588,"ancestor_record_offset":527588,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":530028,"parent_record_offset":527588,"ancestor_record_offset":527588,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":528440,"parent_record_offset":528416,"ancestor_record_offset":527588,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":528416,"parent_record_offset":528360,"ancestor_record_offset":527588,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":527780,"parent_record_offset":527752,"ancestor_record_offset":527588,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":527836,"parent_record_offset":527808,"ancestor_record_offset":527588,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":527860,"parent_record_offset":527836,"ancestor_record_offset":527588,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":528796,"parent_record_offset":528772,"ancestor_record_offset":527588,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":528388,"parent_record_offset":528360,"ancestor_record_offset":527588,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":528360,"parent_record_offset":528336,"ancestor_record_offset":527588,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":528844,"parent_record_offset":528820,"ancestor_record_offset":527588,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":528820,"parent_record_offset":528796,"ancestor_record_offset":527588,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":528336,"parent_record_offset":528312,"ancestor_record_offset":527588,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":528772,"parent_record_offset":528748,"ancestor_record_offset":527588,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":528748,"parent_record_offset":528492,"ancestor_record_offset":527588,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":530144,"parent_record_offset":530120,"ancestor_record_offset":527588,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":528704,"parent_record_offset":528676,"ancestor_record_offset":527588,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":527808,"parent_record_offset":527780,"ancestor_record_offset":527588,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":530120,"parent_record_offset":530084,"ancestor_record_offset":527588,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":527976,"parent_record_offset":527888,"ancestor_record_offset":527588,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":528204,"parent_record_offset":527700,"ancestor_record_offset":527588,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":528000,"parent_record_offset":527976,"ancestor_record_offset":527588,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":530200,"parent_record_offset":530144,"ancestor_record_offset":527588,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":529756,"parent_record_offset":527700,"ancestor_record_offset":527588,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":531060,"parent_record_offset":531008,"ancestor_record_offset":527588,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":527888,"parent_record_offset":527860,"ancestor_record_offset":527588,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":527944,"parent_record_offset":527920,"ancestor_record_offset":527588,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":527920,"parent_record_offset":527888,"ancestor_record_offset":527588,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":530960,"parent_record_offset":530936,"ancestor_record_offset":527588,"inlinee":28073,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1630,"record_offset":531008,"parent_record_offset":530984,"ancestor_record_offset":527588,"inlinee":28014,"source_line":26,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$4::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/roof_edit.rs
//     evidence={"module":1630,"record_offset":529132,"parent_record_offset":529104,"ancestor_record_offset":527588,"inlinee":74370,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":1630,"record_offset":531688,"parent_record_offset":531664,"ancestor_record_offset":527588,"inlinee":29592,"source_line":336,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1630,"record_offset":530568,"parent_record_offset":530544,"ancestor_record_offset":527588,"inlinee":32780,"source_line":69,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1630,"record_offset":530616,"parent_record_offset":530592,"ancestor_record_offset":527588,"inlinee":32782,"source_line":69,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":531836,"parent_record_offset":531812,"ancestor_record_offset":527588,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":531924,"parent_record_offset":531900,"ancestor_record_offset":527588,"inlinee":6705,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":531812,"parent_record_offset":531788,"ancestor_record_offset":527588,"inlinee":5746,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":527664,"parent_record_offset":527628,"ancestor_record_offset":527588,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":530056,"parent_record_offset":530028,"ancestor_record_offset":527588,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":529244,"parent_record_offset":529216,"ancestor_record_offset":527588,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":530260,"parent_record_offset":530236,"ancestor_record_offset":527588,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":529104,"parent_record_offset":527700,"ancestor_record_offset":527588,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":531032,"parent_record_offset":531008,"ancestor_record_offset":527588,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$75::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":530640,"parent_record_offset":530616,"ancestor_record_offset":527588,"inlinee":6539,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":527752,"parent_record_offset":527700,"ancestor_record_offset":527588,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":529188,"parent_record_offset":529160,"ancestor_record_offset":527588,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":529160,"parent_record_offset":529132,"ancestor_record_offset":527588,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":529216,"parent_record_offset":529188,"ancestor_record_offset":527588,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":530592,"parent_record_offset":530568,"ancestor_record_offset":527588,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":527700,"parent_record_offset":527664,"ancestor_record_offset":527588,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":530084,"parent_record_offset":530056,"ancestor_record_offset":527588,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":586,"record_offset":42120,"rva":11769296,"va":5380478416}
// 编译实例 RVAs: 0xb395d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31368677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380478416,"raw_hex":"e8e5a5de01","size":5,"soft_fail":false,"target_va":5411847152,"va":5380478470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":586,"record_offset":42336,"parent_record_offset":42120,"ancestor_record_offset":42120,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":586,"record_offset":42364,"parent_record_offset":42336,"ancestor_record_offset":42120,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":586,"record_offset":42392,"parent_record_offset":42364,"ancestor_record_offset":42120,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":586,"record_offset":42420,"parent_record_offset":42392,"ancestor_record_offset":42120,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$7::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1230C
// 首条位置: {"module":1630,"inlinee":74508}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":700564,"parent_record_offset":700536,"ancestor_record_offset":700536,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":700872,"parent_record_offset":700848,"ancestor_record_offset":700536,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":700848,"parent_record_offset":700792,"ancestor_record_offset":700536,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":700820,"parent_record_offset":700792,"ancestor_record_offset":700536,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":700792,"parent_record_offset":700768,"ancestor_record_offset":700536,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":700768,"parent_record_offset":700744,"ancestor_record_offset":700536,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":700696,"parent_record_offset":700660,"ancestor_record_offset":700536,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":700660,"parent_record_offset":700628,"ancestor_record_offset":700536,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":700596,"parent_record_offset":700564,"ancestor_record_offset":700536,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":700628,"parent_record_offset":700596,"ancestor_record_offset":700536,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$8::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1230A
// 首条位置: {"module":1630,"inlinee":74506}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":701448,"parent_record_offset":698980,"ancestor_record_offset":698980,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":702424,"parent_record_offset":698980,"ancestor_record_offset":698980,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":705516,"parent_record_offset":698980,"ancestor_record_offset":698980,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":699024,"parent_record_offset":698980,"ancestor_record_offset":698980,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":701480,"parent_record_offset":698980,"ancestor_record_offset":698980,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":699840,"parent_record_offset":699816,"ancestor_record_offset":698980,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":699816,"parent_record_offset":699760,"ancestor_record_offset":698980,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":699180,"parent_record_offset":699152,"ancestor_record_offset":698980,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":699236,"parent_record_offset":699208,"ancestor_record_offset":698980,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":699260,"parent_record_offset":699236,"ancestor_record_offset":698980,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":700196,"parent_record_offset":700172,"ancestor_record_offset":698980,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":699788,"parent_record_offset":699760,"ancestor_record_offset":698980,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":699760,"parent_record_offset":699736,"ancestor_record_offset":698980,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":700244,"parent_record_offset":700220,"ancestor_record_offset":698980,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":700220,"parent_record_offset":700196,"ancestor_record_offset":698980,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":699736,"parent_record_offset":699712,"ancestor_record_offset":698980,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":700172,"parent_record_offset":700148,"ancestor_record_offset":698980,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":700148,"parent_record_offset":699892,"ancestor_record_offset":698980,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":701596,"parent_record_offset":701572,"ancestor_record_offset":698980,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":700104,"parent_record_offset":700076,"ancestor_record_offset":698980,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":699208,"parent_record_offset":699180,"ancestor_record_offset":698980,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":701572,"parent_record_offset":701536,"ancestor_record_offset":698980,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":699376,"parent_record_offset":699288,"ancestor_record_offset":698980,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":699604,"parent_record_offset":699096,"ancestor_record_offset":698980,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":699400,"parent_record_offset":699376,"ancestor_record_offset":698980,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":701652,"parent_record_offset":701596,"ancestor_record_offset":698980,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":701176,"parent_record_offset":699096,"ancestor_record_offset":698980,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":699288,"parent_record_offset":699260,"ancestor_record_offset":698980,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":699344,"parent_record_offset":699320,"ancestor_record_offset":698980,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":699320,"parent_record_offset":699288,"ancestor_record_offset":698980,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$7::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/roof_edit.rs
//     evidence={"module":1630,"record_offset":700536,"parent_record_offset":700504,"ancestor_record_offset":698980,"inlinee":74508,"source_line":15,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1630,"record_offset":701712,"parent_record_offset":701688,"ancestor_record_offset":698980,"inlinee":32780,"source_line":69,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1630,"record_offset":701760,"parent_record_offset":701736,"ancestor_record_offset":698980,"inlinee":32782,"source_line":69,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":705588,"parent_record_offset":705564,"ancestor_record_offset":698980,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":705676,"parent_record_offset":705652,"ancestor_record_offset":698980,"inlinee":6671,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":705564,"parent_record_offset":705540,"ancestor_record_offset":698980,"inlinee":6705,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_tuple
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":705900,"parent_record_offset":705876,"ancestor_record_offset":698980,"inlinee":6683,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":699060,"parent_record_offset":699024,"ancestor_record_offset":698980,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":701508,"parent_record_offset":701480,"ancestor_record_offset":698980,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":700660,"parent_record_offset":700628,"ancestor_record_offset":698980,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":702072,"parent_record_offset":702048,"ancestor_record_offset":698980,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":700504,"parent_record_offset":699096,"ancestor_record_offset":698980,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$75::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":701784,"parent_record_offset":701760,"ancestor_record_offset":698980,"inlinee":6539,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":702380,"parent_record_offset":702356,"ancestor_record_offset":698980,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$96::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":703400,"parent_record_offset":703376,"ancestor_record_offset":698980,"inlinee":13361,"source_line":1338,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":699152,"parent_record_offset":699096,"ancestor_record_offset":698980,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":700596,"parent_record_offset":700564,"ancestor_record_offset":698980,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":700564,"parent_record_offset":700536,"ancestor_record_offset":698980,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":700628,"parent_record_offset":700596,"ancestor_record_offset":698980,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":701736,"parent_record_offset":701712,"ancestor_record_offset":698980,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":699096,"parent_record_offset":699060,"ancestor_record_offset":698980,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":701536,"parent_record_offset":701508,"ancestor_record_offset":698980,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8298
// 首条位置: {"module":1630,"inlinee":33432}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":713412,"parent_record_offset":713380,"ancestor_record_offset":713380,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":713436,"parent_record_offset":713412,"ancestor_record_offset":713380,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":713460,"parent_record_offset":713436,"ancestor_record_offset":713380,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":713484,"parent_record_offset":713460,"ancestor_record_offset":713380,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::roof_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":586,"record_offset":35592,"rva":11766656,"va":5380475776}
// 编译实例 RVAs: 0xb38b80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t31001710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380475776,"raw_hex":"e86e0cd901","size":5,"soft_fail":false,"target_va":5411478016,"va":5380476301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5395188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380475776,"raw_hex":"e80cadadff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380476687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14085da90
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2996019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380475776,"raw_hex":"e8cd48d2ff","size":5,"soft_fail":false,"target_va":5377481360,"va":5380477374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140887df0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2821813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380475776,"raw_hex":"e84bf1d4ff","size":5,"soft_fail":false,"target_va":5377654256,"va":5380476064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888d70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2817996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380475776,"raw_hex":"e83400d5ff","size":5,"soft_fail":false,"target_va":5377658224,"va":5380476215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140889390
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2817141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380475776,"raw_hex":"e88b03d5ff","size":5,"soft_fail":false,"target_va":5377659792,"va":5380476928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140898f40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2752486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380475776,"raw_hex":"e81a00d6ff","size":5,"soft_fail":false,"target_va":5377724224,"va":5380476705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14143c450
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9451571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380475776,"raw_hex":"e833389000","size":5,"soft_fail":false,"target_va":5389927504,"va":5380475928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415d29d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11115738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380475776,"raw_hex":"e8da9ca900","size":5,"soft_fail":false,"target_va":5391591888,"va":5380476145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415d6a40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11131498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380475776,"raw_hex":"e86adaa900","size":5,"soft_fail":false,"target_va":5391608384,"va":5380476881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415e0240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11170818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380475776,"raw_hex":"e80274aa00","size":5,"soft_fail":false,"target_va":5391647296,"va":5380476473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415e2840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11180258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380475776,"raw_hex":"e8e298aa00","size":5,"soft_fail":false,"target_va":5391657024,"va":5380476761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415ebe90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11218698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380475776,"raw_hex":"e80a2fab00","size":5,"soft_fail":false,"target_va":5391695504,"va":5380476801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415ed9a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11225586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380475776,"raw_hex":"e8f249ab00","size":5,"soft_fail":false,"target_va":5391702432,"va":5380476841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f9b40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31198898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380475776,"raw_hex":"e8b20edc01","size":5,"soft_fail":false,"target_va":5411674944,"va":5380476041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::de::_$LT$impl$u20$serde_json..value..Value$GT$::unexpected
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31002261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380475776,"raw_hex":"e8950ed901","size":5,"soft_fail":false,"target_va":5411478544,"va":5380476278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::value::de::impl$2::deserialize_enum
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":586,"record_offset":35780,"parent_record_offset":35592,"ancestor_record_offset":35592,"inlinee":26604,"source_line":255,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":586,"record_offset":36804,"parent_record_offset":36780,"ancestor_record_offset":35592,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":586,"record_offset":36780,"parent_record_offset":36692,"ancestor_record_offset":35592,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":586,"record_offset":35956,"parent_record_offset":35932,"ancestor_record_offset":35592,"inlinee":6033,"source_line":1680,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":586,"record_offset":36012,"parent_record_offset":35988,"ancestor_record_offset":35592,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":586,"record_offset":36064,"parent_record_offset":36036,"ancestor_record_offset":35592,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":586,"record_offset":36088,"parent_record_offset":36064,"ancestor_record_offset":35592,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":586,"record_offset":37172,"parent_record_offset":37148,"ancestor_record_offset":35592,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":586,"record_offset":36720,"parent_record_offset":36692,"ancestor_record_offset":35592,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":586,"record_offset":36692,"parent_record_offset":36668,"ancestor_record_offset":35592,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":586,"record_offset":37220,"parent_record_offset":37196,"ancestor_record_offset":35592,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":586,"record_offset":37196,"parent_record_offset":37172,"ancestor_record_offset":35592,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":586,"record_offset":36668,"parent_record_offset":36644,"ancestor_record_offset":35592,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":586,"record_offset":37148,"parent_record_offset":37124,"ancestor_record_offset":35592,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":586,"record_offset":36616,"parent_record_offset":36588,"ancestor_record_offset":35592,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":586,"record_offset":36748,"parent_record_offset":36720,"ancestor_record_offset":35592,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":586,"record_offset":36036,"parent_record_offset":36012,"ancestor_record_offset":35592,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":586,"record_offset":36204,"parent_record_offset":36116,"ancestor_record_offset":35592,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":586,"record_offset":36560,"parent_record_offset":35780,"ancestor_record_offset":35592,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":586,"record_offset":36228,"parent_record_offset":36204,"ancestor_record_offset":35592,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":586,"record_offset":36492,"parent_record_offset":36432,"ancestor_record_offset":35592,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":586,"record_offset":36528,"parent_record_offset":36432,"ancestor_record_offset":35592,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":586,"record_offset":36116,"parent_record_offset":36088,"ancestor_record_offset":35592,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":586,"record_offset":36172,"parent_record_offset":36148,"ancestor_record_offset":35592,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":586,"record_offset":36148,"parent_record_offset":36116,"ancestor_record_offset":35592,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/roof_edit.rs
//     evidence={"module":586,"record_offset":36432,"parent_record_offset":35780,"ancestor_record_offset":35592,"inlinee":33412,"source_line":15,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":586,"record_offset":36464,"parent_record_offset":36432,"ancestor_record_offset":35592,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$17::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":586,"record_offset":35932,"parent_record_offset":35780,"ancestor_record_offset":35592,"inlinee":26606,"source_line":877,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":586,"record_offset":35988,"parent_record_offset":35780,"ancestor_record_offset":35592,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::roof_edit::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2B2E
// 首条位置: {"module":586,"inlinee":11054}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::ser::impl$10::serialize_field
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":586,"record_offset":64464,"parent_record_offset":63912,"ancestor_record_offset":63912,"inlinee":5725,"source_line":794,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct_variant
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":586,"record_offset":63944,"parent_record_offset":63912,"ancestor_record_offset":63912,"inlinee":5433,"source_line":420,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":586,"record_offset":66432,"parent_record_offset":66400,"ancestor_record_offset":63912,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":586,"record_offset":64232,"parent_record_offset":64208,"ancestor_record_offset":63912,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":586,"record_offset":63972,"parent_record_offset":63944,"ancestor_record_offset":63912,"inlinee":5420,"source_line":1849,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":586,"record_offset":64264,"parent_record_offset":64088,"ancestor_record_offset":63912,"inlinee":5429,"source_line":1894,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":586,"record_offset":64348,"parent_record_offset":64088,"ancestor_record_offset":63912,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":586,"record_offset":64056,"parent_record_offset":63944,"ancestor_record_offset":63912,"inlinee":5182,"source_line":219,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":586,"record_offset":64488,"parent_record_offset":64464,"ancestor_record_offset":63912,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":586,"record_offset":64020,"parent_record_offset":63996,"ancestor_record_offset":63912,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":586,"record_offset":63996,"parent_record_offset":63972,"ancestor_record_offset":63912,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::roof_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":86,"record_offset":14200,"rva":2465328,"va":5371174448}
// 编译实例 RVAs: 0x259e30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003e0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2212607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371174448,"raw_hex":"e8013ddeff","size":5,"soft_fail":false,"target_va":5368963232,"va":5371175834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b6100
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2474567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371174448,"raw_hex":"e847c22500","size":5,"soft_fail":false,"target_va":5373649152,"va":5371174580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b98d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2488395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371174448,"raw_hex":"e84bf82500","size":5,"soft_fail":false,"target_va":5373663440,"va":5371175040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404ba9b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2492255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371174448,"raw_hex":"e85f072600","size":5,"soft_fail":false,"target_va":5373667760,"va":5371175500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404baab0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2492235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371174448,"raw_hex":"e84b072600","size":5,"soft_fail":false,"target_va":5373668016,"va":5371175776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bb130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2493931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371174448,"raw_hex":"e8eb0d2600","size":5,"soft_fail":false,"target_va":5373669680,"va":5371175744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bdc80
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2505795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371174448,"raw_hex":"e8433c2600","size":5,"soft_fail":false,"target_va":5373680768,"va":5371174968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404be300
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2506616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371174448,"raw_hex":"e8783f2600","size":5,"soft_fail":false,"target_va":5373682432,"va":5371175811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404be720
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2508759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371174448,"raw_hex":"e8d7472600","size":5,"soft_fail":false,"target_va":5373683488,"va":5371174724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404c11a0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: roof_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2519675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371174448,"raw_hex":"e87b722600","size":5,"soft_fail":false,"target_va":5373694368,"va":5371174688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t40142349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371174448,"raw_hex":"e80d866402","size":5,"soft_fail":false,"target_va":5411318240,"va":5371175886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::error::Error::io
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t40301813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371174448,"raw_hex":"e8f5f46602","size":5,"soft_fail":false,"target_va":5411477664,"va":5371175846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$10::serialize_field
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":86,"record_offset":16616,"parent_record_offset":14200,"ancestor_record_offset":14200,"inlinee":5725,"source_line":794,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct_variant
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":86,"record_offset":14484,"parent_record_offset":14200,"ancestor_record_offset":14200,"inlinee":5433,"source_line":420,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":86,"record_offset":14860,"parent_record_offset":14828,"ancestor_record_offset":14200,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":86,"record_offset":14728,"parent_record_offset":14700,"ancestor_record_offset":14200,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":86,"record_offset":14788,"parent_record_offset":14756,"ancestor_record_offset":14200,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":86,"record_offset":14908,"parent_record_offset":14884,"ancestor_record_offset":14200,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":86,"record_offset":14884,"parent_record_offset":14860,"ancestor_record_offset":14200,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":86,"record_offset":14756,"parent_record_offset":14728,"ancestor_record_offset":14200,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":86,"record_offset":14660,"parent_record_offset":14632,"ancestor_record_offset":14200,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":86,"record_offset":14828,"parent_record_offset":14660,"ancestor_record_offset":14200,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":86,"record_offset":14604,"parent_record_offset":14576,"ancestor_record_offset":14200,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":86,"record_offset":14948,"parent_record_offset":14660,"ancestor_record_offset":14200,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":86,"record_offset":14700,"parent_record_offset":14660,"ancestor_record_offset":14200,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":86,"record_offset":14632,"parent_record_offset":14604,"ancestor_record_offset":14200,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":86,"record_offset":21420,"parent_record_offset":21388,"ancestor_record_offset":14200,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":86,"record_offset":15588,"parent_record_offset":15564,"ancestor_record_offset":14200,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":86,"record_offset":14520,"parent_record_offset":14484,"ancestor_record_offset":14200,"inlinee":5420,"source_line":1849,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":86,"record_offset":15620,"parent_record_offset":15032,"ancestor_record_offset":14200,"inlinee":5429,"source_line":1894,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":86,"record_offset":16100,"parent_record_offset":15032,"ancestor_record_offset":14200,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":86,"record_offset":15000,"parent_record_offset":14484,"ancestor_record_offset":14200,"inlinee":5182,"source_line":219,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":86,"record_offset":16640,"parent_record_offset":16616,"ancestor_record_offset":14200,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":86,"record_offset":14576,"parent_record_offset":14548,"ancestor_record_offset":14200,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":86,"record_offset":14548,"parent_record_offset":14520,"ancestor_record_offset":14200,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::roof_edit::impl$2::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8353
// 首条位置: {"module":593,"inlinee":33619}
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
// 完整结构及原始类型记录: roof_edit.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::roof_edit::RoofEdit
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::resources::history::roof_edit::RoofEdit>", "constructor": "enum2$", "arguments": ["country_core::resources::history::roof_edit::RoofEdit"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::new<enum2$<country_core::resources::history::roof_edit::RoofEdit> >", "constructor": "alloc::sync::Arc::new", "arguments": ["enum2$<country_core::resources::history::roof_edit::RoofEdit>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<enum2$<country_core::resources::history::roof_edit::RoofEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["enum2$<country_core::resources::history::roof_edit::RoofEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::Arc<enum2$<country_core::resources::history::roof_edit::RoofEdit>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::Arc<enum2$<country_core::resources::history::roof_edit::RoofEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<enum2$<country_core::resources::history::roof_edit::RoofEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["enum2$<country_core::resources::history::roof_edit::RoofEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$216::deserialize<enum2$<country_core::resources::history::roof_edit::RoofEdit>,enum2$<serde_json::value::Value> >", "constructor": "serde::de::impls::impl$216::deserialize", "arguments": ["enum2$<country_core::resources::history::roof_edit::RoofEdit>", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$54::fmt<enum2$<country_core::resources::history::roof_edit::RoofEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::impl$54::fmt", "arguments": ["enum2$<country_core::resources::history::roof_edit::RoofEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<alloc::sync::Arc<enum2$<country_core::resources::history::roof_edit::RoofEdit>,alloc::alloc::Global> >", "constructor": "core::marker::PhantomData", "arguments": ["alloc::sync::Arc<enum2$<country_core::resources::history::roof_edit::RoofEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$5::newtype_variant_seed<core::marker::PhantomData<alloc::sync::Arc<enum2$<country_core::resources::history::roof_edit::RoofEdit>,alloc::alloc::Global> > >", "constructor": "serde_json::value::de::impl$5::newtype_variant_seed", "arguments": ["core::marker::PhantomData<alloc::sync::Arc<enum2$<country_core::resources::history::roof_edit::RoofEdit>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::roof_edit::RoofReplayCmd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [已有逆向证据 B / native_observed_partial_layout; not full original struct declaration] RoofEdit native replay variants
// name: "RoofEdit native replay variants"
// baseline: "B"
// status: "native_observed_partial_layout; not full original struct declaration"
// alignment: "unknown"
// complete_size: "unknown"
// unobserved_fields: "unknown"
// original_field_names: "unknown unless explicitly established"
// evidence: [{"path": "evidence/multi-version/B-parameter-findings.md", "sha256": "42d2c8b17de750427f1e97eb36892fdabb832a009313b743512dfad399e76322"}, {"path": "reconstruction/history/observed-history-schema.json", "sha256": "79e5822e6beaffa3563392828a5160993e533c88e3633c5ffa278db2365fc7ce"}, {"path": "evidence/multi-version/B-discrete-replay-verification.json", "sha256": "e6179e56b46b6f6cff1fba4de2a23006e9ff735eca2badd0f1b7e3b57320d388"}]
// native_tags: [{"value": 0, "serialized_name": "CreateRoof"}, {"value": 1, "serialized_name": "DeleteRoof"}, {"value": 2, "serialized_name": "VisualEdit"}, {"value": 3, "serialized_name": "SwitchRoofRidgeDir"}, {"value": 4, "serialized_name": "unknown", "observed_branch_semantics": "flat rim style change"}]
// discriminant_representation: "B replay payload tags; complete Rust enum ABI unknown"
// ------------------------------------------------------------------
// [已有逆向证据 B / native_observed_partial_layout; not full original struct declaration] Roof history VisualEdit serialized fields
// name: "Roof history VisualEdit serialized fields"
// baseline: "B"
// status: "native_observed_partial_layout; not full original struct declaration"
// alignment: "unknown"
// complete_size: "unknown"
// unobserved_fields: "unknown"
// original_field_names: "unknown unless explicitly established"
// evidence: [{"path": "evidence/multi-version/B-roof-replay-schema/visual-field-map.json", "sha256": "a2dbb9459cd9bb7fea6ff2d3ace3afe450ad9fee38333b3962b0b6e54468cc85"}]
// offset_base: "VisualEdit enum record; history offsets retained separately"
// enum_discriminant: "unknown"
// [观察字段；未命名者 original_name=unknown] {"offset": 40, "original_name": "wall_index", "type": "unknown", "width": "unknown", "name_basis": "native serializer key", "binding": {"field": "wall_index", "offset_from_enum": 40, "key_va": "0x1429a4cae", "source_va": "0x14025a00e", "offset_in_history_record": 56}}
// [观察字段；未命名者 original_name=unknown] {"offset": 4, "original_name": "prev_height", "type": "unknown", "width": "unknown", "name_basis": "native serializer key", "binding": {"field": "prev_height", "offset_from_enum": 4, "key_va": "0x1429a4cdb", "source_va": "0x14025a032", "offset_in_history_record": 20}}
// [观察字段；未命名者 original_name=unknown] {"offset": 8, "original_name": "prev_profile", "type": "unknown", "width": "unknown", "name_basis": "native serializer key", "binding": {"field": "prev_profile", "offset_from_enum": 8, "key_va": "0x1429a4ce6", "source_va": "0x14025a056", "offset_in_history_record": 24}}
// [观察字段；未命名者 original_name=unknown] {"offset": 48, "original_name": "prev_tip_offset", "type": "unknown", "width": "unknown", "name_basis": "native serializer key", "binding": {"field": "prev_tip_offset", "offset_from_enum": 48, "key_va": "0x1429a4cf2", "source_va": "0x14025a07a", "offset_in_history_record": 64}}
// [观察字段；未命名者 original_name=unknown] {"offset": 12, "original_name": "prev_ridge", "type": "unknown", "width": "unknown", "name_basis": "native serializer key", "binding": {"field": "prev_ridge", "offset_from_enum": 12, "key_va": "0x1429a4d01", "source_va": "0x14025a09e", "offset_in_history_record": 28}}
// [观察字段；未命名者 original_name=unknown] {"offset": 16, "original_name": "prev_eave", "type": "unknown", "width": "unknown", "name_basis": "native serializer key", "binding": {"field": "prev_eave", "offset_from_enum": 16, "key_va": "0x1429a4d0b", "source_va": "0x14025a0c2", "offset_in_history_record": 32}}
// [观察字段；未命名者 original_name=unknown] {"offset": 20, "original_name": "new_height", "type": "unknown", "width": "unknown", "name_basis": "native serializer key", "binding": {"field": "new_height", "offset_from_enum": 20, "key_va": "0x1429a4d14", "source_va": "0x14025a0e6", "offset_in_history_record": 36}}
// [观察字段；未命名者 original_name=unknown] {"offset": 24, "original_name": "new_profile", "type": "unknown", "width": "unknown", "name_basis": "native serializer key", "binding": {"field": "new_profile", "offset_from_enum": 24, "key_va": "0x1429a4d1e", "source_va": "0x14025a10a", "offset_in_history_record": 40}}
// [观察字段；未命名者 original_name=unknown] {"offset": 56, "original_name": "new_tip_offset", "type": "unknown", "width": "unknown", "name_basis": "native serializer key", "binding": {"field": "new_tip_offset", "offset_from_enum": 56, "key_va": "0x1429a4d29", "source_va": "0x14025a12e", "offset_in_history_record": 72}}
// [观察字段；未命名者 original_name=unknown] {"offset": 28, "original_name": "new_ridge", "type": "unknown", "width": "unknown", "name_basis": "native serializer key", "binding": {"field": "new_ridge", "offset_from_enum": 28, "key_va": "0x1429a4d37", "source_va": "0x14025a152", "offset_in_history_record": 44}}
// [观察字段；未命名者 original_name=unknown] {"offset": 32, "original_name": "new_eave", "type": "unknown", "width": "unknown", "name_basis": "native serializer key", "binding": {"field": "new_eave", "offset_from_enum": 32, "key_va": "0x1429a4d40", "source_va": "0x14025a176", "offset_in_history_record": 48}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: roof_edit.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [资源/历史 JSON schema，不是内存布局] Roof history serialized observations
// 全部实际键和数据形态: {"/commands/Roof": {"observed_types": {"object": 241}, "object_instances": 241, "field_occurrences": {"CreateRoof": 92, "VisualEdit": 118, "DeleteRoof": 26, "SwitchRoofRidgeDir": 5}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/0/edits/1/Roof"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/1/edits/0/Roof"}]}, "/commands/Roof/CreateRoof": {"observed_types": {"object": 92}, "object_instances": 92, "field_occurrences": {"color_id": 92, "params": 92, "wall_index": 92, "flat_roof_style": 34}, "present_in_every_observed_object": ["color_id", "params", "wall_index"], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/0/edits/1/Roof/CreateRoof"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/2/edits/1/Roof/CreateRoof"}]}, "/commands/Roof/CreateRoof/color_id": {"observed_types": {"integer": 92}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0, 7], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/0/edits/1/Roof/CreateRoof/color_id"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/2/edits/1/Roof/CreateRoof/color_id"}]}, "/commands/Roof/CreateRoof/flat_roof_style": {"observed_types": {"string": 34}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": null, "observed_string_values": ["Matching", "MatchingStoneOnly"], "sources": [{"file": "assets/starting-builds/full/03-halftimber-roof/history.json", "pointer": "/History/edit_chain/1/edits/1/Roof/CreateRoof/flat_roof_style"}, {"file": "assets/starting-builds/full/03-halftimber-roof/history.json", "pointer": "/History/edit_chain/6/edits/1/Roof/CreateRoof/flat_roof_style"}]}, "/commands/Roof/CreateRoof/params": {"observed_types": {"object": 92}, "object_instances": 92, "field_occurrences": {"eave_length_01": 92, "height_01": 92, "profile_01": 92, "ridge_dir": 92, "ridge_length_01": 92, "tip_offset_01": 92, "flat_rim_style": 34}, "present_in_every_observed_object": ["eave_length_01", "height_01", "profile_01", "ridge_dir", "ridge_length_01", "tip_offset_01"], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/0/edits/1/Roof/CreateRoof/params"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/2/edits/1/Roof/CreateRoof/params"}]}, "/commands/Roof/CreateRoof/params/eave_length_01": {"observed_types": {"number": 92}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.2, 0.20000000298023224], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/0/edits/1/Roof/CreateRoof/params/eave_length_01"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/2/edits/1/Roof/CreateRoof/params/eave_length_01"}]}, "/commands/Roof/CreateRoof/params/flat_rim_style": {"observed_types": {"string": 34}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": null, "observed_string_values": ["Rim"], "sources": [{"file": "assets/starting-builds/full/03-halftimber-roof/history.json", "pointer": "/History/edit_chain/1/edits/1/Roof/CreateRoof/params/flat_rim_style"}, {"file": "assets/starting-builds/full/03-halftimber-roof/history.json", "pointer": "/History/edit_chain/6/edits/1/Roof/CreateRoof/params/flat_rim_style"}]}, "/commands/Roof/CreateRoof/params/height_01": {"observed_types": {"number": 92}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.1, 0.8333333134651184], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/0/edits/1/Roof/CreateRoof/params/height_01"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/2/edits/1/Roof/CreateRoof/params/height_01"}]}, "/commands/Roof/CreateRoof/params/profile_01": {"observed_types": {"number": 92}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.5, 0.5], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/0/edits/1/Roof/CreateRoof/params/profile_01"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/2/edits/1/Roof/CreateRoof/params/profile_01"}]}, "/commands/Roof/CreateRoof/params/ridge_dir": {"observed_types": {"string": 92}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": null, "observed_string_values": ["Length", "Width"], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/0/edits/1/Roof/CreateRoof/params/ridge_dir"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/2/edits/1/Roof/CreateRoof/params/ridge_dir"}]}, "/commands/Roof/CreateRoof/params/ridge_length_01": {"observed_types": {"number": 92}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.0, 1.0], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/0/edits/1/Roof/CreateRoof/params/ridge_length_01"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/2/edits/1/Roof/CreateRoof/params/ridge_length_01"}]}, "/commands/Roof/CreateRoof/params/tip_offset_01": {"observed_types": {"array": 92}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {"2": 92}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/0/edits/1/Roof/CreateRoof/params/tip_offset_01"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/2/edits/1/Roof/CreateRoof/params/tip_offset_01"}]}, "/commands/Roof/CreateRoof/params/tip_offset_01/[]": {"observed_types": {"number": 184}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.0, 0.0], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/0/edits/1/Roof/CreateRoof/params/tip_offset_01/0"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/0/edits/1/Roof/CreateRoof/params/tip_offset_01/1"}]}, "/commands/Roof/CreateRoof/wall_index": {"observed_types": {"integer": 92}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [1, 109], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/0/edits/1/Roof/CreateRoof/wall_index"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/2/edits/1/Roof/CreateRoof/wall_index"}]}, "/commands/Roof/DeleteRoof": {"observed_types": {"object": 26}, "object_instances": 26, "field_occurrences": {"color_id": 26, "params": 26, "wall_index": 26, "flat_roof_style": 8}, "present_in_every_observed_object": ["color_id", "params", "wall_index"], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/62/edits/0/Roof/DeleteRoof"}, {"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/63/edits/0/Roof/DeleteRoof"}]}, "/commands/Roof/DeleteRoof/color_id": {"observed_types": {"integer": 26}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0, 5], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/62/edits/0/Roof/DeleteRoof/color_id"}, {"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/63/edits/0/Roof/DeleteRoof/color_id"}]}, "/commands/Roof/DeleteRoof/flat_roof_style": {"observed_types": {"string": 8}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": null, "observed_string_values": ["Matching", "MatchingStoneOnly"], "sources": [{"file": "assets/starting-builds/full/04-small-bridge/history.json", "pointer": "/History/edit_chain/22/edits/0/Roof/DeleteRoof/flat_roof_style"}, {"file": "assets/starting-builds/full/04-small-bridge/history.json", "pointer": "/History/edit_chain/67/edits/0/Roof/DeleteRoof/flat_roof_style"}]}, "/commands/Roof/DeleteRoof/params": {"observed_types": {"object": 26}, "object_instances": 26, "field_occurrences": {"eave_length_01": 26, "height_01": 26, "profile_01": 26, "ridge_dir": 26, "ridge_length_01": 26, "tip_offset_01": 26, "flat_rim_style": 8}, "present_in_every_observed_object": ["eave_length_01", "height_01", "profile_01", "ridge_dir", "ridge_length_01", "tip_offset_01"], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/62/edits/0/Roof/DeleteRoof/params"}, {"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/63/edits/0/Roof/DeleteRoof/params"}]}, "/commands/Roof/DeleteRoof/params/eave_length_01": {"observed_types": {"number": 26}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.2, 0.20000000298023224], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/62/edits/0/Roof/DeleteRoof/params/eave_length_01"}, {"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/63/edits/0/Roof/DeleteRoof/params/eave_length_01"}]}, "/commands/Roof/DeleteRoof/params/flat_rim_style": {"observed_types": {"string": 8}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": null, "observed_string_values": ["Rim"], "sources": [{"file": "assets/starting-builds/full/04-small-bridge/history.json", "pointer": "/History/edit_chain/22/edits/0/Roof/DeleteRoof/params/flat_rim_style"}, {"file": "assets/starting-builds/full/04-small-bridge/history.json", "pointer": "/History/edit_chain/67/edits/0/Roof/DeleteRoof/params/flat_rim_style"}]}, "/commands/Roof/DeleteRoof/params/height_01": {"observed_types": {"number": 26}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.0, 0.8333333134651184], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/62/edits/0/Roof/DeleteRoof/params/height_01"}, {"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/63/edits/0/Roof/DeleteRoof/params/height_01"}]}, "/commands/Roof/DeleteRoof/params/profile_01": {"observed_types": {"number": 26}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.1700000911951065, 0.5], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/62/edits/0/Roof/DeleteRoof/params/profile_01"}, {"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/63/edits/0/Roof/DeleteRoof/params/profile_01"}]}, "/commands/Roof/DeleteRoof/params/ridge_dir": {"observed_types": {"string": 26}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": null, "observed_string_values": ["Length", "Width"], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/62/edits/0/Roof/DeleteRoof/params/ridge_dir"}, {"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/63/edits/0/Roof/DeleteRoof/params/ridge_dir"}]}, "/commands/Roof/DeleteRoof/params/ridge_length_01": {"observed_types": {"number": 26}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.0, 1.0], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/62/edits/0/Roof/DeleteRoof/params/ridge_length_01"}, {"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/63/edits/0/Roof/DeleteRoof/params/ridge_length_01"}]}, "/commands/Roof/DeleteRoof/params/tip_offset_01": {"observed_types": {"array": 26}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {"2": 26}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/62/edits/0/Roof/DeleteRoof/params/tip_offset_01"}, {"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/63/edits/0/Roof/DeleteRoof/params/tip_offset_01"}]}, "/commands/Roof/DeleteRoof/params/tip_offset_01/[]": {"observed_types": {"number": 52}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.0, 0.0], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/62/edits/0/Roof/DeleteRoof/params/tip_offset_01/0"}, {"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/62/edits/0/Roof/DeleteRoof/params/tip_offset_01/1"}]}, "/commands/Roof/DeleteRoof/wall_index": {"observed_types": {"integer": 26}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [3, 47], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/62/edits/0/Roof/DeleteRoof/wall_index"}, {"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/63/edits/0/Roof/DeleteRoof/wall_index"}]}, "/commands/Roof/SwitchRoofRidgeDir": {"observed_types": {"object": 5}, "object_instances": 5, "field_occurrences": {"wall_index": 5, "from": 5, "to": 5}, "present_in_every_observed_object": ["wall_index", "from", "to"], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/03-halftimber-roof/history.json", "pointer": "/History/edit_chain/28/edits/0/Roof/SwitchRoofRidgeDir"}, {"file": "assets/starting-builds/full/03-halftimber-roof/history.json", "pointer": "/History/edit_chain/45/edits/0/Roof/SwitchRoofRidgeDir"}]}, "/commands/Roof/SwitchRoofRidgeDir/from": {"observed_types": {"string": 5}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": null, "observed_string_values": ["Length", "Width"], "sources": [{"file": "assets/starting-builds/full/03-halftimber-roof/history.json", "pointer": "/History/edit_chain/28/edits/0/Roof/SwitchRoofRidgeDir/from"}, {"file": "assets/starting-builds/full/03-halftimber-roof/history.json", "pointer": "/History/edit_chain/45/edits/0/Roof/SwitchRoofRidgeDir/from"}]}, "/commands/Roof/SwitchRoofRidgeDir/to": {"observed_types": {"string": 5}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": null, "observed_string_values": ["Length", "Width"], "sources": [{"file": "assets/starting-builds/full/03-halftimber-roof/history.json", "pointer": "/History/edit_chain/28/edits/0/Roof/SwitchRoofRidgeDir/to"}, {"file": "assets/starting-builds/full/03-halftimber-roof/history.json", "pointer": "/History/edit_chain/45/edits/0/Roof/SwitchRoofRidgeDir/to"}]}, "/commands/Roof/SwitchRoofRidgeDir/wall_index": {"observed_types": {"integer": 5}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [9, 35], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/full/03-halftimber-roof/history.json", "pointer": "/History/edit_chain/28/edits/0/Roof/SwitchRoofRidgeDir/wall_index"}, {"file": "assets/starting-builds/full/03-halftimber-roof/history.json", "pointer": "/History/edit_chain/45/edits/0/Roof/SwitchRoofRidgeDir/wall_index"}]}, "/commands/Roof/VisualEdit": {"observed_types": {"object": 118}, "object_instances": 118, "field_occurrences": {"new_eave": 118, "new_height": 118, "new_profile": 118, "new_ridge": 118, "new_tip_offset": 118, "prev_eave": 118, "prev_height": 118, "prev_profile": 118, "prev_ridge": 118, "prev_tip_offset": 118, "wall_index": 118}, "present_in_every_observed_object": ["new_eave", "new_height", "new_profile", "new_ridge", "new_tip_offset", "prev_eave", "prev_height", "prev_profile", "prev_ridge", "prev_tip_offset", "wall_index"], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/1/edits/0/Roof/VisualEdit"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/5/edits/0/Roof/VisualEdit"}]}, "/commands/Roof/VisualEdit/new_eave": {"observed_types": {"number": 118}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.2, 0.20000000298023224], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/1/edits/0/Roof/VisualEdit/new_eave"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/5/edits/0/Roof/VisualEdit/new_eave"}]}, "/commands/Roof/VisualEdit/new_height": {"observed_types": {"number": 118}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.0, 0.5349163], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/1/edits/0/Roof/VisualEdit/new_height"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/5/edits/0/Roof/VisualEdit/new_height"}]}, "/commands/Roof/VisualEdit/new_profile": {"observed_types": {"number": 118}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.0, 1.0], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/1/edits/0/Roof/VisualEdit/new_profile"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/5/edits/0/Roof/VisualEdit/new_profile"}]}, "/commands/Roof/VisualEdit/new_ridge": {"observed_types": {"number": 118}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.0, 1.0], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/1/edits/0/Roof/VisualEdit/new_ridge"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/5/edits/0/Roof/VisualEdit/new_ridge"}]}, "/commands/Roof/VisualEdit/new_tip_offset": {"observed_types": {"array": 118}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {"2": 118}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/1/edits/0/Roof/VisualEdit/new_tip_offset"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/5/edits/0/Roof/VisualEdit/new_tip_offset"}]}, "/commands/Roof/VisualEdit/new_tip_offset/[]": {"observed_types": {"number": 236}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.0, 0.0], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/1/edits/0/Roof/VisualEdit/new_tip_offset/0"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/1/edits/0/Roof/VisualEdit/new_tip_offset/1"}]}, "/commands/Roof/VisualEdit/prev_eave": {"observed_types": {"number": 118}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.2, 0.20000000298023224], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/1/edits/0/Roof/VisualEdit/prev_eave"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/5/edits/0/Roof/VisualEdit/prev_eave"}]}, "/commands/Roof/VisualEdit/prev_height": {"observed_types": {"number": 118}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.0, 0.8333333134651184], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/1/edits/0/Roof/VisualEdit/prev_height"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/5/edits/0/Roof/VisualEdit/prev_height"}]}, "/commands/Roof/VisualEdit/prev_profile": {"observed_types": {"number": 118}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.0, 1.0], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/1/edits/0/Roof/VisualEdit/prev_profile"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/5/edits/0/Roof/VisualEdit/prev_profile"}]}, "/commands/Roof/VisualEdit/prev_ridge": {"observed_types": {"number": 118}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.0, 1.0], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/1/edits/0/Roof/VisualEdit/prev_ridge"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/5/edits/0/Roof/VisualEdit/prev_ridge"}]}, "/commands/Roof/VisualEdit/prev_tip_offset": {"observed_types": {"array": 118}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {"2": 118}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/1/edits/0/Roof/VisualEdit/prev_tip_offset"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/5/edits/0/Roof/VisualEdit/prev_tip_offset"}]}, "/commands/Roof/VisualEdit/prev_tip_offset/[]": {"observed_types": {"number": 236}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.0, 0.0], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/1/edits/0/Roof/VisualEdit/prev_tip_offset/0"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/1/edits/0/Roof/VisualEdit/prev_tip_offset/1"}]}, "/commands/Roof/VisualEdit/wall_index": {"observed_types": {"integer": 118}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [1, 109], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/1/edits/0/Roof/VisualEdit/wall_index"}, {"file": "assets/starting-builds/demo/01-demo3/history.json", "pointer": "/History/edit_chain/5/edits/0/Roof/VisualEdit/wall_index"}]}}
// offsets / Rust numeric types / missing defaults = unknown。
// [已有模块证据关联；不自动成为字段事实] evidence/building-history-owners/three-version-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/history/roof_edit.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/history/roof_edit.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/history/roof_edit.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// [已有模块证据关联；不自动成为字段事实] evidence/pdb-structure/type-layout-coverage.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::history::roof_edit::RoofEdit", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: roof_edit.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::event::writer::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>", "bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >

// ------------------------------------------------------------------
// 函数: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: 15797e3bded995b5a989b6fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::history::roof_edit::_::impl$0::deserialize::__Field> > @ 0x15d2a16 -> 0xb392d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: b5a0009b0d79be71bd95ee9f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::history::roof_edit::_::impl$0::deserialize::__Field> >; module=1630, inline_record=485104; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: c73a375642245f5c12e88d45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::roof_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=586, inline_record=36432; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: ce7bdf206d3a8b807e77aeb0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x15e03d0 -> 0xb39400 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$10::deserialize
// 信息 ID: d03ae0c670f6dcdd77cd9f25
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=713212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$11::visit_map
// 信息 ID: f3e0ecbc96af49c2b08aa17f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=711660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$12::visit_str
// 信息 ID: 7cb55e1b4d13825ef9ee0b5b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=626044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$13::deserialize
// 信息 ID: 893389f5645e4c998c0c69cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=625872; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$14::visit_map
// 信息 ID: 3ba35ebdd35204fea5421be4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=624320; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 信息 ID: a441bae3642e7059bde23af6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=606712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 信息 ID: 00bd4a7227603873b23e08d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=605168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str<serde_json::error::Error>
// 信息 ID: ee713c14349b6de5b71bbed4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x15d6bd0 -> 0xb394f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$4::deserialize
// 信息 ID: 784b9716e8866ef59d88fe0d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=529132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$5::visit_map
// 信息 ID: 84f522b19c59714b1de172db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=527588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str<serde_json::error::Error>
// 信息 ID: 97cafaea073ab5af5e8443d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x15ec0b1 -> 0xb395d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$7::deserialize
// 信息 ID: 2c345f3531e11073ae2a55e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=700536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$8::visit_map
// 信息 ID: 48f4d6d61aef6426b346f471
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=698980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str
// 信息 ID: 0e00f3860713088d82d130b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=713380; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::roof_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 信息 ID: d4f6189d8027e89e854aae33
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::resources::history::roof_edit::_::impl$0::deserialize -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impls::impl$216::deserialize<enum2$<country_core::resources::history::roof_edit::RoofEdit>,enum2$<serde_json::value::Value> > @ 0xb5d9dc -> 0xb38b80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::roof_edit::_::impl$0::serialize
// 信息 ID: a772f7ea1d043e793f2e9b17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=586, inline_record=63912; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::roof_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: ae6a73ce39a5de28ef5fc0c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::history::roof_edit::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::roof_edit::impl$2::fmt
// 信息 ID: 0c0a3997cd32b164f834538f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::sync::impl$54::fmt<enum2$<country_core::resources::history::roof_edit::RoofEdit>,alloc::alloc::Global>; module=593, inline_record=58900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
