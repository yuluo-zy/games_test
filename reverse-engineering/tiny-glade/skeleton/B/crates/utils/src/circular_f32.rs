// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/utils/src/circular_f32.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 56157488c9f61057e590ed78f955b304d3fee6b04d90741291398290ba7a0ac0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: circular_f32.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 39, 172, 527, 546, 564, 767, 1532, 1673, 1675, 3375, 4770
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7A9A
// 首条位置: {"module":1532,"inlinee":31386}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::_::impl$0::deserialize::impl$0::expecting
// 记录形式: compiled_procedure; instances=1
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1675,"record_offset":1608,"rva":24065120,"va":5392774240}
// 编译实例 RVAs: 0x16f3460
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-9881829","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5392774240,"raw_hex":"e91b3769ff","size":5,"soft_fail":false,"target_va":5382892432,"va":5392774256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::_::impl$0::deserialize::impl$0::visit_bytes
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7A96
// 首条位置: {"module":1532,"inlinee":31382}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7A97
// 首条位置: {"module":1532,"inlinee":31383}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1532,"record_offset":13144,"parent_record_offset":13116,"ancestor_record_offset":13116,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1532,"record_offset":13172,"parent_record_offset":13144,"ancestor_record_offset":13116,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":13200,"parent_record_offset":13172,"ancestor_record_offset":13116,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":13228,"parent_record_offset":13200,"ancestor_record_offset":13116,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::_::impl$0::deserialize::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7A8D
// 首条位置: {"module":546,"inlinee":31373}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1180D
// 首条位置: {"module":1532,"inlinee":71693}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::__private::de::content::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":12888,"parent_record_offset":12844,"ancestor_record_offset":12844,"inlinee":71674,"source_line":2039,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":13520,"parent_record_offset":13496,"ancestor_record_offset":12844,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":13568,"parent_record_offset":13544,"ancestor_record_offset":12844,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":13544,"parent_record_offset":13520,"ancestor_record_offset":12844,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1532,"record_offset":13448,"parent_record_offset":13424,"ancestor_record_offset":12844,"inlinee":5602,"source_line":1081,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1532,"record_offset":13424,"parent_record_offset":12888,"ancestor_record_offset":12844,"inlinee":5600,"source_line":2689,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":13496,"parent_record_offset":13472,"ancestor_record_offset":12844,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":13472,"parent_record_offset":13448,"ancestor_record_offset":12844,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1532,"record_offset":13172,"parent_record_offset":13144,"ancestor_record_offset":12844,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":13200,"parent_record_offset":13172,"ancestor_record_offset":12844,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":13228,"parent_record_offset":13200,"ancestor_record_offset":12844,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1532,"record_offset":13144,"parent_record_offset":13116,"ancestor_record_offset":12844,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":13032,"parent_record_offset":12888,"ancestor_record_offset":12844,"inlinee":11492,"source_line":1563,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":13088,"parent_record_offset":12888,"ancestor_record_offset":12844,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":12948,"parent_record_offset":12888,"ancestor_record_offset":12844,"inlinee":11530,"source_line":1383,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::_::impl$0::deserialize::impl$0::visit_bytes
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1532,"record_offset":13004,"parent_record_offset":12888,"ancestor_record_offset":12844,"inlinee":31382,"source_line":7,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1532,"record_offset":13116,"parent_record_offset":13088,"ancestor_record_offset":12844,"inlinee":31383,"source_line":7,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1532,"record_offset":12972,"parent_record_offset":12948,"ancestor_record_offset":12844,"inlinee":31373,"source_line":7,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::_::impl$0::deserialize::impl$2::expecting
// 记录形式: compiled_procedure; instances=1
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1675,"record_offset":1748,"rva":24065152,"va":5392774272}
// 编译实例 RVAs: 0x16f3480
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-9881861","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5392774272,"raw_hex":"e9fb3669ff","size":5,"soft_fail":false,"target_va":5382892432,"va":5392774288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1180B
// 首条位置: {"module":1532,"inlinee":71691}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1532,"record_offset":14160,"parent_record_offset":12480,"ancestor_record_offset":12480,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":14192,"parent_record_offset":12480,"ancestor_record_offset":12480,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":12524,"parent_record_offset":12480,"ancestor_record_offset":12480,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":13672,"parent_record_offset":12480,"ancestor_record_offset":12480,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":13520,"parent_record_offset":13496,"ancestor_record_offset":12480,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":13568,"parent_record_offset":13544,"ancestor_record_offset":12480,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":13544,"parent_record_offset":13520,"ancestor_record_offset":12480,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1532,"record_offset":13448,"parent_record_offset":13424,"ancestor_record_offset":12480,"inlinee":5602,"source_line":1081,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1532,"record_offset":13424,"parent_record_offset":12888,"ancestor_record_offset":12480,"inlinee":5600,"source_line":2689,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":13496,"parent_record_offset":13472,"ancestor_record_offset":12480,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":13472,"parent_record_offset":13448,"ancestor_record_offset":12480,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1532,"record_offset":13172,"parent_record_offset":13144,"ancestor_record_offset":12480,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::copysign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1532,"record_offset":14012,"parent_record_offset":13928,"ancestor_record_offset":12480,"inlinee":6570,"source_line":1505,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":1532,"record_offset":13980,"parent_record_offset":13956,"ancestor_record_offset":12480,"inlinee":6857,"source_line":716,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":1532,"record_offset":13956,"parent_record_offset":13928,"ancestor_record_offset":12480,"inlinee":6856,"source_line":682,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1532,"record_offset":12728,"parent_record_offset":12700,"ancestor_record_offset":12480,"inlinee":6982,"source_line":358,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1532,"record_offset":12700,"parent_record_offset":12672,"ancestor_record_offset":12480,"inlinee":6980,"source_line":49,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1532,"record_offset":12756,"parent_record_offset":12728,"ancestor_record_offset":12480,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":13200,"parent_record_offset":13172,"ancestor_record_offset":12480,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":13228,"parent_record_offset":13200,"ancestor_record_offset":12480,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1532,"record_offset":13144,"parent_record_offset":13116,"ancestor_record_offset":12480,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":13864,"parent_record_offset":13832,"ancestor_record_offset":12480,"inlinee":71685,"source_line":1665,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":13832,"parent_record_offset":13800,"ancestor_record_offset":12480,"inlinee":71684,"source_line":1826,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":12888,"parent_record_offset":12844,"ancestor_record_offset":12480,"inlinee":71674,"source_line":2039,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":14264,"parent_record_offset":14240,"ancestor_record_offset":12480,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1532,"record_offset":14240,"parent_record_offset":14216,"ancestor_record_offset":12480,"inlinee":6671,"source_line":126,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":12572,"parent_record_offset":12524,"ancestor_record_offset":12480,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":13704,"parent_record_offset":13672,"ancestor_record_offset":12480,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":13032,"parent_record_offset":12888,"ancestor_record_offset":12480,"inlinee":11492,"source_line":1563,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":13088,"parent_record_offset":12888,"ancestor_record_offset":12480,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":12948,"parent_record_offset":12888,"ancestor_record_offset":12480,"inlinee":11530,"source_line":1383,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":12800,"parent_record_offset":12620,"ancestor_record_offset":12480,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":13800,"parent_record_offset":13768,"ancestor_record_offset":12480,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":13928,"parent_record_offset":13864,"ancestor_record_offset":12480,"inlinee":6569,"source_line":227,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":14104,"parent_record_offset":13864,"ancestor_record_offset":12480,"inlinee":11521,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":14076,"parent_record_offset":13864,"ancestor_record_offset":12480,"inlinee":11519,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":14044,"parent_record_offset":13864,"ancestor_record_offset":12480,"inlinee":6568,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::MapDeserializer::next_pair
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1532,"record_offset":12672,"parent_record_offset":12620,"ancestor_record_offset":12480,"inlinee":6979,"source_line":1146,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$57::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1532,"record_offset":12620,"parent_record_offset":12572,"ancestor_record_offset":12480,"inlinee":7004,"source_line":1210,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$57::next_value_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1532,"record_offset":13736,"parent_record_offset":13704,"ancestor_record_offset":12480,"inlinee":7006,"source_line":1223,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::_::impl$0::deserialize::impl$0::visit_bytes
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1532,"record_offset":13004,"parent_record_offset":12888,"ancestor_record_offset":12480,"inlinee":31382,"source_line":7,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1532,"record_offset":13116,"parent_record_offset":13088,"ancestor_record_offset":12480,"inlinee":31383,"source_line":7,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1532,"record_offset":12972,"parent_record_offset":12948,"ancestor_record_offset":12480,"inlinee":31373,"source_line":7,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1532,"record_offset":12844,"parent_record_offset":12800,"ancestor_record_offset":12480,"inlinee":71693,"source_line":7,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1180E
// 首条位置: {"module":1532,"inlinee":71694}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::impl$6::next_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":14504,"parent_record_offset":14460,"ancestor_record_offset":14460,"inlinee":11489,"source_line":1753,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::copysign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1532,"record_offset":15000,"parent_record_offset":14916,"ancestor_record_offset":14460,"inlinee":6570,"source_line":1505,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":1532,"record_offset":14968,"parent_record_offset":14944,"ancestor_record_offset":14460,"inlinee":6857,"source_line":716,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":1532,"record_offset":14944,"parent_record_offset":14916,"ancestor_record_offset":14460,"inlinee":6856,"source_line":682,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1532,"record_offset":14648,"parent_record_offset":14624,"ancestor_record_offset":14460,"inlinee":6982,"source_line":358,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1532,"record_offset":14624,"parent_record_offset":14584,"ancestor_record_offset":14460,"inlinee":6980,"source_line":49,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1532,"record_offset":14672,"parent_record_offset":14648,"ancestor_record_offset":14460,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":14816,"parent_record_offset":14780,"ancestor_record_offset":14460,"inlinee":71685,"source_line":1665,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":14780,"parent_record_offset":14744,"ancestor_record_offset":14460,"inlinee":71684,"source_line":1826,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":14544,"parent_record_offset":14504,"ancestor_record_offset":14460,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":14708,"parent_record_offset":14584,"ancestor_record_offset":14460,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":14744,"parent_record_offset":14708,"ancestor_record_offset":14460,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":14916,"parent_record_offset":14816,"ancestor_record_offset":14460,"inlinee":6569,"source_line":227,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":15148,"parent_record_offset":14816,"ancestor_record_offset":14460,"inlinee":11521,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":15092,"parent_record_offset":14816,"ancestor_record_offset":14460,"inlinee":11519,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":14888,"parent_record_offset":14816,"ancestor_record_offset":14460,"inlinee":11517,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u16
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":15120,"parent_record_offset":14816,"ancestor_record_offset":14460,"inlinee":11520,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":15032,"parent_record_offset":14816,"ancestor_record_offset":14460,"inlinee":11518,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":15060,"parent_record_offset":14816,"ancestor_record_offset":14460,"inlinee":6568,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$45::next_element_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1532,"record_offset":14584,"parent_record_offset":14544,"ancestor_record_offset":14460,"inlinee":7008,"source_line":963,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":39,"record_offset":22876,"rva":982272,"va":5369691392}
// 编译实例 RVAs: 0xefd00, 0xa44920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bdc80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3989303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369691392,"raw_hex":"e837df3c00","size":5,"soft_fail":false,"target_va":5373680768,"va":5369691460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141586090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11802407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379475744,"raw_hex":"e82717b400","size":5,"soft_fail":false,"target_va":5391278224,"va":5379475812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t41626688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369691392,"raw_hex":"e8402c7b02","size":5,"soft_fail":false,"target_va":5411318240,"va":5369691547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":39,"record_offset":23140,"parent_record_offset":22876,"ancestor_record_offset":22876,"inlinee":5727,"source_line":409,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":39,"record_offset":23680,"parent_record_offset":22876,"ancestor_record_offset":22876,"inlinee":5728,"source_line":774,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":39,"record_offset":23652,"parent_record_offset":22876,"ancestor_record_offset":22876,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":23504,"parent_record_offset":23472,"ancestor_record_offset":22876,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":23376,"parent_record_offset":23352,"ancestor_record_offset":22876,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":23432,"parent_record_offset":23404,"ancestor_record_offset":22876,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":23552,"parent_record_offset":23528,"ancestor_record_offset":22876,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":23528,"parent_record_offset":23504,"ancestor_record_offset":22876,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":23404,"parent_record_offset":23376,"ancestor_record_offset":22876,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":23316,"parent_record_offset":23292,"ancestor_record_offset":22876,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":23472,"parent_record_offset":23316,"ancestor_record_offset":22876,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":23268,"parent_record_offset":23244,"ancestor_record_offset":22876,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":23592,"parent_record_offset":23316,"ancestor_record_offset":22876,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":23352,"parent_record_offset":23316,"ancestor_record_offset":22876,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":39,"record_offset":23292,"parent_record_offset":23268,"ancestor_record_offset":22876,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":39,"record_offset":23192,"parent_record_offset":23168,"ancestor_record_offset":22876,"inlinee":5420,"source_line":1849,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":39,"record_offset":23744,"parent_record_offset":23712,"ancestor_record_offset":22876,"inlinee":5435,"source_line":1859,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":39,"record_offset":23168,"parent_record_offset":23140,"ancestor_record_offset":22876,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":39,"record_offset":23712,"parent_record_offset":23680,"ancestor_record_offset":22876,"inlinee":5730,"source_line":720,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":39,"record_offset":23244,"parent_record_offset":23220,"ancestor_record_offset":22876,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":39,"record_offset":23220,"parent_record_offset":23192,"ancestor_record_offset":22876,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=1
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":546,"record_offset":12736,"rva":10766848,"va":5379475968}
// 编译实例 RVAs: 0xa44a00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4394601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379475968,"raw_hex":"e897f1bcff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379476100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14085db60
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1994696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379475968,"raw_hex":"e83890e1ff","size":5,"soft_fail":false,"target_va":5377481568,"va":5379476259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140898f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1751852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379475968,"raw_hex":"e8d444e5ff","size":5,"soft_fail":false,"target_va":5377724224,"va":5379476071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b864c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1317389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379475968,"raw_hex":"e80d1a1400","size":5,"soft_fail":false,"target_va":5380793536,"va":5379476142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141595a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11866057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379475968,"raw_hex":"e8c90fb500","size":5,"soft_fail":false,"target_va":5391342112,"va":5379476050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":546,"record_offset":12932,"parent_record_offset":12736,"ancestor_record_offset":12736,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":546,"record_offset":12904,"parent_record_offset":12736,"ancestor_record_offset":12736,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":13136,"parent_record_offset":13112,"ancestor_record_offset":12736,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":13112,"parent_record_offset":13060,"ancestor_record_offset":12736,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":546,"record_offset":13084,"parent_record_offset":13060,"ancestor_record_offset":12736,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":546,"record_offset":13060,"parent_record_offset":13036,"ancestor_record_offset":12736,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":546,"record_offset":13036,"parent_record_offset":13016,"ancestor_record_offset":12736,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":546,"record_offset":12956,"parent_record_offset":12932,"ancestor_record_offset":12736,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::impl$3::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7A7D
// 首条位置: {"module":546,"inlinee":31357}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::impl$6::eq
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x366E
// 首条位置: {"module":172,"inlinee":13934}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::CircularF32::set_value
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1675,"record_offset":252,"rva":24064384,"va":5392773504}
// 编译实例 RVAs: 0x16f3180
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19074416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392773504,"raw_hex":"e8700d2301","size":5,"soft_fail":false,"target_va":5411847952,"va":5392773531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::circular_f32::CircularF32::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1675,"record_offset":380,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":44957,"source_line":32,"archive":"_pdb/B/modules/01675.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1675,"record_offset":404,"parent_record_offset":380,"ancestor_record_offset":252,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/01675.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::CircularF32::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF9C
// 首条位置: {"module":767,"inlinee":44956}
// 编译实例 RVAs: 0x16f31d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19074343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392773584,"raw_hex":"e8270d2301","size":5,"soft_fail":false,"target_va":5411847952,"va":5392773604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::circular_f32::CircularF32::project_unbounded
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":767,"record_offset":7724,"parent_record_offset":7700,"ancestor_record_offset":7700,"inlinee":44957,"source_line":32,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":767,"record_offset":7748,"parent_record_offset":7724,"ancestor_record_offset":7700,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::CircularF32::project_unbounded
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF9D
// 首条位置: {"module":767,"inlinee":44957}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":767,"record_offset":7748,"parent_record_offset":7724,"ancestor_record_offset":7724,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::CircularF32::as_f32
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=17
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF96
// 首条位置: {"module":767,"inlinee":44950}
// 编译实例 RVAs: 0x16f3210
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19074275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392773648,"raw_hex":"e8e30c2301","size":5,"soft_fail":false,"target_va":5411847952,"va":5392773672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=14
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":767,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":816,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::CircularF32::distance
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1675,"record_offset":780,"rva":24064608,"va":5392773728}
// 编译实例 RVAs: 0x16f3260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19074175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392773728,"raw_hex":"e87f0c2301","size":5,"soft_fail":false,"target_va":5411847952,"va":5392773772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::circular_f32::CircularF32::as_f32
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1675,"record_offset":912,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":44950,"source_line":36,"archive":"_pdb/B/modules/01675.jsonl.gz"}
//   [pdb_inline_direct] utils::comparative_space::impl$3::distance
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1675,"record_offset":1024,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":44725,"source_line":127,"archive":"_pdb/B/modules/01675.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1675,"record_offset":1052,"parent_record_offset":1024,"ancestor_record_offset":780,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01675.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1675,"record_offset":1108,"parent_record_offset":1024,"ancestor_record_offset":780,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01675.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1675,"record_offset":936,"parent_record_offset":912,"ancestor_record_offset":780,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/01675.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::CircularF32::lerp
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF9B
// 首条位置: {"module":767,"inlinee":44955}
// 编译实例 RVAs: 0x16f3330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19073957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392773936,"raw_hex":"e8a50b2301","size":5,"soft_fail":false,"target_va":5411847952,"va":5392773990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::circular_f32::CircularF32::as_f32
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":767,"record_offset":7452,"parent_record_offset":7420,"ancestor_record_offset":7420,"inlinee":44950,"source_line":36,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] utils::circular_f32::CircularF32::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":767,"record_offset":7700,"parent_record_offset":7420,"ancestor_record_offset":7420,"inlinee":44956,"source_line":22,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":767,"record_offset":7568,"parent_record_offset":7420,"ancestor_record_offset":7420,"inlinee":14137,"source_line":25,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::CircularF32::project_unbounded
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":767,"record_offset":7724,"parent_record_offset":7700,"ancestor_record_offset":7420,"inlinee":44957,"source_line":32,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_clamp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":767,"record_offset":7636,"parent_record_offset":7568,"ancestor_record_offset":7420,"inlinee":44786,"source_line":149,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":767,"record_offset":7476,"parent_record_offset":7452,"ancestor_record_offset":7420,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::subtract
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":767,"record_offset":7596,"parent_record_offset":7568,"ancestor_record_offset":7420,"inlinee":31608,"source_line":161,"archive":"_pdb/B/modules/00767.jsonl.gz"}
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
// 完整结构及原始类型记录: circular_f32.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::circular_f32::CircularF32
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 10; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,utils::circular_f32::CircularF32>", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "utils::circular_f32::CircularF32"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<utils::circular_f32::CircularF32>", "constructor": "core::fmt::impl$74::fmt", "arguments": ["utils::circular_f32::CircularF32"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<utils::circular_f32::CircularF32>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["utils::circular_f32::CircularF32"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: circular_f32.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: circular_f32.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_tod::TodButtonState>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>", "bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>", "bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_tod::TodButtonState>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_tod::TodButtonState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
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
// 函数: utils::circular_f32::CircularF32::as_f32
// 信息 ID: 12df19e77085f45d535bf0d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 13 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::roofs::roof_shape::Roof::rotation_yaw_radians @ 0x8e3b42 -> 0x16f3210 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::rmb_autoclutter::handle_rmb_autoclutter @ 0x976f3e -> 0x16f3210 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0x9ad14c -> 0x16f3210 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0x9ad95e -> 0x16f3210 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121cea0 -> 0x16f3210 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121db27 -> 0x16f3210 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::curved_halftimber::wooden_floor_borders::wooden_floor_borders @ 0x124461f -> 0x16f3210 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x21a3c9a -> 0x16f3210 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::get_pos_at_u; module=767, inline_record=816; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::get_tangent_at_u; module=767, inline_record=1280; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::get_closest_u_from_pos; module=767, inline_record=1896; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::get_normal_at_u; module=767, inline_record=2444; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::lerp; module=767, inline_record=7452; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::lerp; module=767, inline_record=7508; 不是独立调用边。
// [PDB 内联宿主] utils::circular_f32::CircularF32::distance; module=1675, inline_record=912; 不是独立调用边。
// [PDB 内联宿主] utils::circular_f32::CircularF32::distance; module=1675, inline_record=968; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::CircularF32::distance
// 信息 ID: 4e309a81a702619b7e77d65f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0x9ad9c9 -> 0x16f3260 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0x9ad266 -> 0x16f3260 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::CircularF32::lerp
// 信息 ID: 9cdcdc130ebaa8c96b92737d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0x9ad6c3 -> 0x16f3330 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::lerp; module=767, inline_record=7420; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::CircularF32::new
// 信息 ID: f19f8560ddba4d448bdb1c13
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0x9ad66e -> 0x16f31d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0x9ad21b -> 0x16f31d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0x9ad91a -> 0x16f31d0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::lerp; module=767, inline_record=7700; 不是独立调用边。
// [PDB 内联宿主] utils::circular_f32::CircularF32::lerp; module=1675, inline_record=1516; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::CircularF32::project_unbounded
// 信息 ID: eb8d07b4418e3ec7c4ee2a19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::lerp; module=767, inline_record=7724; 不是独立调用边。
// [PDB 内联宿主] utils::circular_f32::CircularF32::set_value; module=1675, inline_record=380; 不是独立调用边。
// [PDB 内联宿主] utils::circular_f32::CircularF32::new; module=1675, inline_record=564; 不是独立调用边。
// [PDB 内联宿主] utils::circular_f32::CircularF32::lerp; module=1675, inline_record=1540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::CircularF32::set_value
// 信息 ID: 040e0fc9fd63f05d2938f727
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::_::impl$0::deserialize
// 信息 ID: 4aaf5581b6f5c9a648591676
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=9968; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=11788; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: 89f0010360c41c58df646618
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impl$1::fmt<utils::circular_f32::_::impl$0::deserialize::__FieldVisitor> @ 0xa43e60 -> 0x16f3460 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::_::impl$0::deserialize::impl$0::visit_bytes
// 信息 ID: 6627df6d0cfa7e37bb0dc7f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::circular_f32::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=13004; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::circular_f32::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=13056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 3792d3271cfce3844d9db21d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::circular_f32::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=13116; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::circular_f32::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=13284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::_::impl$0::deserialize::impl$0::visit_u64
// 信息 ID: a5972cc288c79ce1e03f68ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::circular_f32::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=12972; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::circular_f32::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=13620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 8e77d96a36e355a9b83b3ca6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::circular_f32::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=12844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 28370d47130e120c2f934cb4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impl$1::fmt<utils::circular_f32::_::impl$0::deserialize::__Visitor> @ 0xa43e10 -> 0x16f3480 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 36bbf6e0b45f7188f7872bb0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::circular_f32::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=12480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: fc15556c32f262b4c16d496f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::circular_f32::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=14460; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: fc9eaad8754688cf5eed86e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] utils::circular_f32::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,utils::circular_f32::CircularF32> @ 0x4bac08 -> 0xefd00 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,utils::circular_f32::CircularF32> @ 0x1585478 -> 0xa44920 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: aa7f73fe0d7dccbca2052532
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::circular_f32::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<utils::circular_f32::CircularF32> @ 0x15929de -> 0xa44a00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::impl$3::fmt
// 信息 ID: 13ebe77f626375638b0e5720
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<utils::circular_f32::CircularF32>; module=546, inline_record=6820; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$74::fmt<utils::circular_f32::CircularF32>; module=4770, inline_record=404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::impl$6::eq
// 信息 ID: 223dcb631ac96dfa44cf2845
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::circle::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=172, inline_record=63164; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle; module=527, inline_record=83004; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; module=564, inline_record=117904; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=1673, inline_record=11016; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1673, inline_record=64672; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
