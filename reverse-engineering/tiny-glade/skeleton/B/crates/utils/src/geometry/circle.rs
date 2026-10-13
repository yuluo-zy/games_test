// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/utils/src/geometry/circle.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 69db16956f57159dc3851baf9117729b45f441da5596b9ae2f4bf915ec88cd71
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: circle.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 172, 184, 498, 519, 525, 527, 567, 767, 768, 1532, 1546, 1610, 1640, 1673, 2599, 3375, 3465, 3485, 4349, 4351, 4658
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
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
// 函数符号: utils::geometry::circle::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1182C
// 首条位置: {"module":1532,"inlinee":71724}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$0::expecting
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":767,"record_offset":8120,"rva":13206336,"va":5381915456}
// 编译实例 RVAs: 0xc98340
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t976955","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381915456,"raw_hex":"e93be80e00","size":5,"soft_fail":false,"target_va":5382892432,"va":5381915472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_bytes<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1673,"record_offset":260,"rva":24014432,"va":5392723552}
// 编译实例 RVAs: 0x16e6e60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11809
// 首条位置: {"module":1532,"inlinee":71689}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1532,"record_offset":8412,"parent_record_offset":8388,"ancestor_record_offset":8388,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1532,"record_offset":8436,"parent_record_offset":8412,"ancestor_record_offset":8388,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":8460,"parent_record_offset":8436,"ancestor_record_offset":8388,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":8484,"parent_record_offset":8460,"ancestor_record_offset":8388,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11808
// 首条位置: {"module":1532,"inlinee":71688}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11807
// 首条位置: {"module":1532,"inlinee":71687}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::__private::de::content::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":8072,"parent_record_offset":8036,"ancestor_record_offset":8036,"inlinee":71674,"source_line":2039,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":8288,"parent_record_offset":8264,"ancestor_record_offset":8036,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":8336,"parent_record_offset":8312,"ancestor_record_offset":8036,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":8312,"parent_record_offset":8288,"ancestor_record_offset":8036,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1532,"record_offset":8216,"parent_record_offset":8192,"ancestor_record_offset":8036,"inlinee":5602,"source_line":1081,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1532,"record_offset":8192,"parent_record_offset":8072,"ancestor_record_offset":8036,"inlinee":5600,"source_line":2689,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":8264,"parent_record_offset":8240,"ancestor_record_offset":8036,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":8240,"parent_record_offset":8216,"ancestor_record_offset":8036,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":8556,"parent_record_offset":8072,"ancestor_record_offset":8036,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1532,"record_offset":8436,"parent_record_offset":8412,"ancestor_record_offset":8036,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":8460,"parent_record_offset":8436,"ancestor_record_offset":8036,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":8484,"parent_record_offset":8460,"ancestor_record_offset":8036,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1532,"record_offset":8412,"parent_record_offset":8388,"ancestor_record_offset":8036,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":8724,"parent_record_offset":8072,"ancestor_record_offset":8036,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":8136,"parent_record_offset":8072,"ancestor_record_offset":8036,"inlinee":11530,"source_line":1383,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":1532,"record_offset":8388,"parent_record_offset":8072,"ancestor_record_offset":8036,"inlinee":71689,"source_line":16,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":1532,"record_offset":8160,"parent_record_offset":8136,"ancestor_record_offset":8036,"inlinee":71688,"source_line":16,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$2::expecting
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":767,"record_offset":8264,"rva":13206368,"va":5381915488}
// 编译实例 RVAs: 0xc98360
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t976923","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381915488,"raw_hex":"e91be80e00","size":5,"soft_fail":false,"target_va":5382892432,"va":5381915504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11802
// 首条位置: {"module":1532,"inlinee":71682}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1532,"record_offset":9604,"parent_record_offset":7660,"ancestor_record_offset":7660,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1532,"record_offset":9776,"parent_record_offset":7660,"ancestor_record_offset":7660,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":10044,"parent_record_offset":7660,"ancestor_record_offset":7660,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":7720,"parent_record_offset":7660,"ancestor_record_offset":7660,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":8968,"parent_record_offset":7660,"ancestor_record_offset":7660,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":8288,"parent_record_offset":8264,"ancestor_record_offset":7660,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":8336,"parent_record_offset":8312,"ancestor_record_offset":7660,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":8312,"parent_record_offset":8288,"ancestor_record_offset":7660,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1532,"record_offset":8216,"parent_record_offset":8192,"ancestor_record_offset":7660,"inlinee":5602,"source_line":1081,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1532,"record_offset":8192,"parent_record_offset":8072,"ancestor_record_offset":7660,"inlinee":5600,"source_line":2689,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":8264,"parent_record_offset":8240,"ancestor_record_offset":7660,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":8240,"parent_record_offset":8216,"ancestor_record_offset":7660,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":8556,"parent_record_offset":8072,"ancestor_record_offset":7660,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1532,"record_offset":8436,"parent_record_offset":8412,"ancestor_record_offset":7660,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::copysign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1532,"record_offset":9372,"parent_record_offset":9288,"ancestor_record_offset":7660,"inlinee":6570,"source_line":1505,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":1532,"record_offset":9340,"parent_record_offset":9316,"ancestor_record_offset":7660,"inlinee":6857,"source_line":716,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":1532,"record_offset":9316,"parent_record_offset":9288,"ancestor_record_offset":7660,"inlinee":6856,"source_line":682,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1532,"record_offset":7916,"parent_record_offset":7884,"ancestor_record_offset":7660,"inlinee":6982,"source_line":358,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1532,"record_offset":7884,"parent_record_offset":7852,"ancestor_record_offset":7660,"inlinee":6980,"source_line":49,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1532,"record_offset":7948,"parent_record_offset":7916,"ancestor_record_offset":7660,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1532,"record_offset":8912,"parent_record_offset":7808,"ancestor_record_offset":7660,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":8460,"parent_record_offset":8436,"ancestor_record_offset":7660,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":8484,"parent_record_offset":8460,"ancestor_record_offset":7660,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1532,"record_offset":8412,"parent_record_offset":8388,"ancestor_record_offset":7660,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/features/impl_serde.rs
//     evidence={"module":1532,"record_offset":9732,"parent_record_offset":9708,"ancestor_record_offset":7660,"inlinee":29596,"source_line":18,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":9208,"parent_record_offset":9168,"ancestor_record_offset":7660,"inlinee":71685,"source_line":1665,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":9168,"parent_record_offset":9128,"ancestor_record_offset":7660,"inlinee":71684,"source_line":1826,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":8072,"parent_record_offset":8036,"ancestor_record_offset":7660,"inlinee":71674,"source_line":2039,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":10116,"parent_record_offset":10092,"ancestor_record_offset":7660,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1532,"record_offset":10204,"parent_record_offset":10180,"ancestor_record_offset":7660,"inlinee":6671,"source_line":126,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_tuple_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1532,"record_offset":10092,"parent_record_offset":10068,"ancestor_record_offset":7660,"inlinee":6648,"source_line":126,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":7764,"parent_record_offset":7720,"ancestor_record_offset":7660,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":9008,"parent_record_offset":8968,"ancestor_record_offset":7660,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":8724,"parent_record_offset":8072,"ancestor_record_offset":7660,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":8136,"parent_record_offset":8072,"ancestor_record_offset":7660,"inlinee":11530,"source_line":1383,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":7996,"parent_record_offset":7808,"ancestor_record_offset":7660,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":9128,"parent_record_offset":9088,"ancestor_record_offset":7660,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":9288,"parent_record_offset":9208,"ancestor_record_offset":7660,"inlinee":6569,"source_line":227,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i16
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":9520,"parent_record_offset":9208,"ancestor_record_offset":7660,"inlinee":11516,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":9548,"parent_record_offset":9208,"ancestor_record_offset":7660,"inlinee":11521,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":9464,"parent_record_offset":9208,"ancestor_record_offset":7660,"inlinee":11519,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u16
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":9492,"parent_record_offset":9208,"ancestor_record_offset":7660,"inlinee":11520,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":9404,"parent_record_offset":9208,"ancestor_record_offset":7660,"inlinee":11518,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":9432,"parent_record_offset":9208,"ancestor_record_offset":7660,"inlinee":6568,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::MapDeserializer::next_pair
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1532,"record_offset":7852,"parent_record_offset":7808,"ancestor_record_offset":7660,"inlinee":6979,"source_line":1146,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$57::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1532,"record_offset":7808,"parent_record_offset":7764,"ancestor_record_offset":7660,"inlinee":7004,"source_line":1210,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$57::next_value_seed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1532,"record_offset":9048,"parent_record_offset":9008,"ancestor_record_offset":7660,"inlinee":7006,"source_line":1223,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1532,"record_offset":9968,"parent_record_offset":9944,"ancestor_record_offset":7660,"inlinee":31386,"source_line":7,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":1532,"record_offset":8388,"parent_record_offset":8072,"ancestor_record_offset":7660,"inlinee":71689,"source_line":16,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":1532,"record_offset":8160,"parent_record_offset":8136,"ancestor_record_offset":7660,"inlinee":71688,"source_line":16,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":1532,"record_offset":8036,"parent_record_offset":7996,"ancestor_record_offset":7660,"inlinee":71687,"source_line":16,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11803
// 首条位置: {"module":1532,"inlinee":71683}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::impl$6::next_element
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":10444,"parent_record_offset":10388,"ancestor_record_offset":10388,"inlinee":11489,"source_line":1753,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::copysign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1532,"record_offset":11244,"parent_record_offset":11160,"ancestor_record_offset":10388,"inlinee":6570,"source_line":1505,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":1532,"record_offset":11212,"parent_record_offset":11188,"ancestor_record_offset":10388,"inlinee":6857,"source_line":716,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":1532,"record_offset":11188,"parent_record_offset":11160,"ancestor_record_offset":10388,"inlinee":6856,"source_line":682,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1532,"record_offset":10564,"parent_record_offset":10540,"ancestor_record_offset":10388,"inlinee":6982,"source_line":358,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1532,"record_offset":10540,"parent_record_offset":10508,"ancestor_record_offset":10388,"inlinee":6980,"source_line":49,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1532,"record_offset":11616,"parent_record_offset":11560,"ancestor_record_offset":10388,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1532,"record_offset":10588,"parent_record_offset":10564,"ancestor_record_offset":10388,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1532,"record_offset":11712,"parent_record_offset":11688,"ancestor_record_offset":10388,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1532,"record_offset":10680,"parent_record_offset":10508,"ancestor_record_offset":10388,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1532,"record_offset":11688,"parent_record_offset":11664,"ancestor_record_offset":10388,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1532,"record_offset":11640,"parent_record_offset":11616,"ancestor_record_offset":10388,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1532,"record_offset":11664,"parent_record_offset":11640,"ancestor_record_offset":10388,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/features/impl_serde.rs
//     evidence={"module":1532,"record_offset":10648,"parent_record_offset":10624,"ancestor_record_offset":10388,"inlinee":29596,"source_line":18,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":11052,"parent_record_offset":11012,"ancestor_record_offset":10388,"inlinee":71685,"source_line":1665,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":11012,"parent_record_offset":10972,"ancestor_record_offset":10388,"inlinee":71684,"source_line":1826,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":10476,"parent_record_offset":10444,"ancestor_record_offset":10388,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":10624,"parent_record_offset":10508,"ancestor_record_offset":10388,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":10972,"parent_record_offset":10932,"ancestor_record_offset":10388,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":11160,"parent_record_offset":11052,"ancestor_record_offset":10388,"inlinee":6569,"source_line":227,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":11392,"parent_record_offset":11052,"ancestor_record_offset":10388,"inlinee":11521,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":11336,"parent_record_offset":11052,"ancestor_record_offset":10388,"inlinee":11519,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":11132,"parent_record_offset":11052,"ancestor_record_offset":10388,"inlinee":11517,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u16
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":11364,"parent_record_offset":11052,"ancestor_record_offset":10388,"inlinee":11520,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":11276,"parent_record_offset":11052,"ancestor_record_offset":10388,"inlinee":11518,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":11304,"parent_record_offset":11052,"ancestor_record_offset":10388,"inlinee":6568,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$45::next_element_seed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1532,"record_offset":10508,"parent_record_offset":10476,"ancestor_record_offset":10388,"inlinee":7008,"source_line":963,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1532,"record_offset":11788,"parent_record_offset":11764,"ancestor_record_offset":10388,"inlinee":31386,"source_line":7,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3673
// 首条位置: {"module":1673,"inlinee":13939}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1673,"record_offset":64708,"parent_record_offset":64576,"ancestor_record_offset":64576,"inlinee":5727,"source_line":409,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1673,"record_offset":65308,"parent_record_offset":64576,"ancestor_record_offset":64576,"inlinee":5728,"source_line":774,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1673,"record_offset":65252,"parent_record_offset":64576,"ancestor_record_offset":64576,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::is_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":1673,"record_offset":64616,"parent_record_offset":64576,"ancestor_record_offset":64576,"inlinee":13932,"source_line":27,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":65104,"parent_record_offset":65072,"ancestor_record_offset":64576,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":64972,"parent_record_offset":64944,"ancestor_record_offset":64576,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":65032,"parent_record_offset":65000,"ancestor_record_offset":64576,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":65152,"parent_record_offset":65128,"ancestor_record_offset":64576,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":65128,"parent_record_offset":65104,"ancestor_record_offset":64576,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":65000,"parent_record_offset":64972,"ancestor_record_offset":64576,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1673,"record_offset":64904,"parent_record_offset":64876,"ancestor_record_offset":64576,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1673,"record_offset":65072,"parent_record_offset":64904,"ancestor_record_offset":64576,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1673,"record_offset":64848,"parent_record_offset":64820,"ancestor_record_offset":64576,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1673,"record_offset":65192,"parent_record_offset":64904,"ancestor_record_offset":64576,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1673,"record_offset":64944,"parent_record_offset":64904,"ancestor_record_offset":64576,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1673,"record_offset":64876,"parent_record_offset":64848,"ancestor_record_offset":64576,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1673,"record_offset":64644,"parent_record_offset":64616,"ancestor_record_offset":64576,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1673,"record_offset":64764,"parent_record_offset":64736,"ancestor_record_offset":64576,"inlinee":5420,"source_line":1849,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1673,"record_offset":65372,"parent_record_offset":65340,"ancestor_record_offset":64576,"inlinee":5435,"source_line":1859,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1673,"record_offset":64736,"parent_record_offset":64708,"ancestor_record_offset":64576,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1673,"record_offset":65340,"parent_record_offset":65308,"ancestor_record_offset":64576,"inlinee":5730,"source_line":720,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":1673,"record_offset":64820,"parent_record_offset":64792,"ancestor_record_offset":64576,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":1673,"record_offset":64792,"parent_record_offset":64764,"ancestor_record_offset":64576,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1673,"record_offset":64672,"parent_record_offset":64644,"ancestor_record_offset":64576,"inlinee":13934,"source_line":7,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":172,"record_offset":62844,"rva":4033136,"va":5372742256}
// 编译实例 RVAs: 0x3d8a70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bab90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t925777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372742256,"raw_hex":"e851200e00","size":5,"soft_fail":false,"target_va":5373668240,"va":5372742458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bd170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t935593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372742256,"raw_hex":"e8a9460e00","size":5,"soft_fail":false,"target_va":5373677936,"va":5372742338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bdc80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t938393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372742256,"raw_hex":"e899510e00","size":5,"soft_fail":false,"target_va":5373680768,"va":5372742370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38575724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372742256,"raw_hex":"e86c9e4c02","size":5,"soft_fail":false,"target_va":5411318240,"va":5372742511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":172,"record_offset":63200,"parent_record_offset":62844,"ancestor_record_offset":62844,"inlinee":5727,"source_line":409,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":172,"record_offset":63800,"parent_record_offset":62844,"ancestor_record_offset":62844,"inlinee":5728,"source_line":774,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":172,"record_offset":63744,"parent_record_offset":62844,"ancestor_record_offset":62844,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::is_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":172,"record_offset":63116,"parent_record_offset":62844,"ancestor_record_offset":62844,"inlinee":13932,"source_line":27,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":172,"record_offset":63596,"parent_record_offset":63564,"ancestor_record_offset":62844,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":172,"record_offset":63464,"parent_record_offset":63436,"ancestor_record_offset":62844,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":172,"record_offset":63524,"parent_record_offset":63492,"ancestor_record_offset":62844,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":172,"record_offset":63644,"parent_record_offset":63620,"ancestor_record_offset":62844,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":172,"record_offset":63620,"parent_record_offset":63596,"ancestor_record_offset":62844,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":172,"record_offset":63492,"parent_record_offset":63464,"ancestor_record_offset":62844,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":172,"record_offset":63396,"parent_record_offset":63368,"ancestor_record_offset":62844,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":172,"record_offset":63564,"parent_record_offset":63396,"ancestor_record_offset":62844,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":172,"record_offset":63340,"parent_record_offset":63312,"ancestor_record_offset":62844,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":172,"record_offset":63684,"parent_record_offset":63396,"ancestor_record_offset":62844,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":172,"record_offset":63436,"parent_record_offset":63396,"ancestor_record_offset":62844,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":172,"record_offset":63368,"parent_record_offset":63340,"ancestor_record_offset":62844,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":172,"record_offset":63140,"parent_record_offset":63116,"ancestor_record_offset":62844,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":172,"record_offset":63256,"parent_record_offset":63228,"ancestor_record_offset":62844,"inlinee":5420,"source_line":1849,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":172,"record_offset":63864,"parent_record_offset":63832,"ancestor_record_offset":62844,"inlinee":5435,"source_line":1859,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":172,"record_offset":63228,"parent_record_offset":63200,"ancestor_record_offset":62844,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":172,"record_offset":63832,"parent_record_offset":63800,"ancestor_record_offset":62844,"inlinee":5730,"source_line":720,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":172,"record_offset":63312,"parent_record_offset":63284,"ancestor_record_offset":62844,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":172,"record_offset":63284,"parent_record_offset":63256,"ancestor_record_offset":62844,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":172,"record_offset":63164,"parent_record_offset":63140,"ancestor_record_offset":62844,"inlinee":13934,"source_line":7,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1673,"record_offset":10784,"rva":24017216,"va":5392726336}
// 编译实例 RVAs: 0x16e7940
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17645017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392726336,"raw_hex":"e827c2f2fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392726516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14085db60
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15245204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392726336,"raw_hex":"e86c6017ff","size":5,"soft_fail":false,"target_va":5377481568,"va":5392726767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140898f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15002268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392726336,"raw_hex":"e864151bff","size":5,"soft_fail":false,"target_va":5377724224,"va":5392726487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b864c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11933037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392726336,"raw_hex":"e893ea49ff","size":5,"soft_fail":false,"target_va":5380793536,"va":5392726568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141592910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1397072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392726336,"raw_hex":"e8b0aeeaff","size":5,"soft_fail":false,"target_va":5391329552,"va":5392726619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141595a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1384359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392726336,"raw_hex":"e859e0eaff","size":5,"soft_fail":false,"target_va":5391342112,"va":5392726466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141596670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1381172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392726336,"raw_hex":"e8cceceaff","size":5,"soft_fail":false,"target_va":5391345264,"va":5392726431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1673,"record_offset":11080,"parent_record_offset":10784,"ancestor_record_offset":10784,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1673,"record_offset":11052,"parent_record_offset":10784,"ancestor_record_offset":10784,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::is_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":1673,"record_offset":10960,"parent_record_offset":10784,"ancestor_record_offset":10784,"inlinee":13932,"source_line":27,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":11308,"parent_record_offset":11284,"ancestor_record_offset":10784,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":11284,"parent_record_offset":11228,"ancestor_record_offset":10784,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":11256,"parent_record_offset":11228,"ancestor_record_offset":10784,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":11228,"parent_record_offset":11204,"ancestor_record_offset":10784,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":11204,"parent_record_offset":11180,"ancestor_record_offset":10784,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1673,"record_offset":10988,"parent_record_offset":10960,"ancestor_record_offset":10784,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1673,"record_offset":11108,"parent_record_offset":11080,"ancestor_record_offset":10784,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1673,"record_offset":11016,"parent_record_offset":10988,"ancestor_record_offset":10784,"inlinee":13934,"source_line":7,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::impl$3::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x75B6
// 首条位置: {"module":527,"record_offset":15100,"rva":10109920,"va":5378819040}
// 编译实例 RVAs: 0x9a43e0, 0x165bd40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field3_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4073740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378819040,"raw_hex":"e80c293e00","size":5,"soft_fail":false,"target_va":5382892944,"va":5378819199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::impl$6::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17E6A
// 首条位置: {"module":2599,"inlinee":97898}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$102::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2599,"record_offset":22612,"parent_record_offset":22588,"ancestor_record_offset":22588,"inlinee":8499,"source_line":17,"archive":"_pdb/B/modules/02599.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::is_default
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x366C
// 首条位置: {"module":172,"inlinee":13932}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":172,"record_offset":63140,"parent_record_offset":63116,"ancestor_record_offset":63116,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::impl$6::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":172,"record_offset":63164,"parent_record_offset":63140,"ancestor_record_offset":63116,"inlinee":13934,"source_line":7,"archive":"_pdb/B/modules/00172.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":767,"record_offset":252,"rva":13202752,"va":5381911872}
// 编译实例 RVAs: 0xc97540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::new_rotated
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF9E
// 首条位置: {"module":767,"inlinee":44958}
// 编译实例 RVAs: 0xc97560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::area
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":767,"record_offset":492,"rva":13202816,"va":5381911936}
// 编译实例 RVAs: 0xc97580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":767,"record_offset":616,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::yaw_radians
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6D5A
// 首条位置: {"module":498,"inlinee":27994}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::get_pos_at_u
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFA1
// 首条位置: {"module":767,"record_offset":644,"rva":13202848,"va":5381911968}
// 编译实例 RVAs: 0xc975a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29935722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381911968,"raw_hex":"e86ac8c801","size":5,"soft_fail":false,"target_va":5411847808,"va":5381912081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29935909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381911968,"raw_hex":"e825c9c801","size":5,"soft_fail":false,"target_va":5411847952,"va":5381912038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29936046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381911968,"raw_hex":"e8aec9c801","size":5,"soft_fail":false,"target_va":5411848144,"va":5381912093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":932,"parent_record_offset":644,"ancestor_record_offset":644,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$38::add_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":1020,"parent_record_offset":644,"ancestor_record_offset":644,"inlinee":14288,"source_line":1317,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":767,"record_offset":784,"parent_record_offset":644,"ancestor_record_offset":644,"inlinee":44948,"source_line":80,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":767,"record_offset":960,"parent_record_offset":932,"ancestor_record_offset":644,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":767,"record_offset":1048,"parent_record_offset":1020,"ancestor_record_offset":644,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::cos
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":767,"record_offset":872,"parent_record_offset":784,"ancestor_record_offset":644,"inlinee":14893,"source_line":675,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":767,"record_offset":900,"parent_record_offset":784,"ancestor_record_offset":644,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::CircularF32::as_f32
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":767,"record_offset":816,"parent_record_offset":784,"ancestor_record_offset":644,"inlinee":44950,"source_line":36,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":767,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":644,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::get_tangent_at_u
// 记录形式: compiled_procedure; instances=1
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":767,"record_offset":1112,"rva":13203040,"va":5381912160}
// 编译实例 RVAs: 0xc97660
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29935551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381912160,"raw_hex":"e8bfc7c801","size":5,"soft_fail":false,"target_va":5411847808,"va":5381912252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29935732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381912160,"raw_hex":"e874c8c801","size":5,"soft_fail":false,"target_va":5411847952,"va":5381912215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29935876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381912160,"raw_hex":"e804c9c801","size":5,"soft_fail":false,"target_va":5411848144,"va":5381912263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":1396,"parent_record_offset":1112,"ancestor_record_offset":1112,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":767,"record_offset":1252,"parent_record_offset":1112,"ancestor_record_offset":1112,"inlinee":44948,"source_line":80,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":767,"record_offset":1336,"parent_record_offset":1252,"ancestor_record_offset":1112,"inlinee":14893,"source_line":675,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":767,"record_offset":1364,"parent_record_offset":1252,"ancestor_record_offset":1112,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::CircularF32::as_f32
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":767,"record_offset":1280,"parent_record_offset":1252,"ancestor_record_offset":1112,"inlinee":44950,"source_line":36,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":767,"record_offset":1304,"parent_record_offset":1280,"ancestor_record_offset":1112,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::get_closest_u_from_pos
// 记录形式: compiled_procedure; instances=1
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":767,"record_offset":1428,"rva":13203184,"va":5381912304}
// 编译实例 RVAs: 0xc976f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29934934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381912304,"raw_hex":"e856c5c801","size":5,"soft_fail":false,"target_va":5411847408,"va":5381912469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29935226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381912304,"raw_hex":"e87ac6c801","size":5,"soft_fail":false,"target_va":5411847712,"va":5381912481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29935440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381912304,"raw_hex":"e850c7c801","size":5,"soft_fail":false,"target_va":5411847952,"va":5381912507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29629378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381912304,"raw_hex":"e8c21bc401","size":5,"soft_fail":false,"target_va":5411542064,"va":5381912681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29629130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381912304,"raw_hex":"e8ca1ac401","size":5,"soft_fail":false,"target_va":5411541904,"va":5381912769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::try_normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":1644,"parent_record_offset":1428,"ancestor_record_offset":1428,"inlinee":25426,"source_line":481,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":1592,"parent_record_offset":1428,"ancestor_record_offset":1428,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] utils::cartesian_to_polar
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":767,"record_offset":1808,"parent_record_offset":1428,"ancestor_record_offset":1428,"inlinee":44752,"source_line":418,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] utils::circular_f32::CircularF32::as_f32
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":767,"record_offset":1896,"parent_record_offset":1428,"ancestor_record_offset":1428,"inlinee":44950,"source_line":36,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":767,"record_offset":2084,"parent_record_offset":1428,"ancestor_record_offset":1428,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":767,"record_offset":1956,"parent_record_offset":1428,"ancestor_record_offset":1428,"inlinee":24685,"source_line":31,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":767,"record_offset":2172,"parent_record_offset":2148,"ancestor_record_offset":1428,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":767,"record_offset":2196,"parent_record_offset":2172,"ancestor_record_offset":1428,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":767,"record_offset":1992,"parent_record_offset":1956,"ancestor_record_offset":1428,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":767,"record_offset":2264,"parent_record_offset":2084,"ancestor_record_offset":1428,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":767,"record_offset":1616,"parent_record_offset":1592,"ancestor_record_offset":1428,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":767,"record_offset":2148,"parent_record_offset":2124,"ancestor_record_offset":1428,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":767,"record_offset":2124,"parent_record_offset":2084,"ancestor_record_offset":1428,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":1732,"parent_record_offset":1700,"ancestor_record_offset":1428,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":1700,"parent_record_offset":1676,"ancestor_record_offset":1428,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":1676,"parent_record_offset":1644,"ancestor_record_offset":1428,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":767,"record_offset":1864,"parent_record_offset":1808,"ancestor_record_offset":1428,"inlinee":14897,"source_line":830,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":767,"record_offset":1836,"parent_record_offset":1808,"ancestor_record_offset":1428,"inlinee":10999,"source_line":627,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":767,"record_offset":1764,"parent_record_offset":1732,"ancestor_record_offset":1428,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":767,"record_offset":1920,"parent_record_offset":1896,"ancestor_record_offset":1428,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":767,"record_offset":2020,"parent_record_offset":1956,"ancestor_record_offset":1428,"inlinee":24686,"source_line":90,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::get_normal_at_u
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=8
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF94
// 首条位置: {"module":767,"inlinee":44948}
// 编译实例 RVAs: 0xc978e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29934911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381912800,"raw_hex":"e83fc5c801","size":5,"soft_fail":false,"target_va":5411847808,"va":5381912892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29935092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381912800,"raw_hex":"e8f4c5c801","size":5,"soft_fail":false,"target_va":5411847952,"va":5381912855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29935236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381912800,"raw_hex":"e884c6c801","size":5,"soft_fail":false,"target_va":5411848144,"va":5381912903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":767,"record_offset":872,"parent_record_offset":784,"ancestor_record_offset":784,"inlinee":14893,"source_line":675,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":767,"record_offset":900,"parent_record_offset":784,"ancestor_record_offset":784,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] utils::circular_f32::CircularF32::as_f32
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":767,"record_offset":816,"parent_record_offset":784,"ancestor_record_offset":784,"inlinee":44950,"source_line":36,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":767,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":784,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::perimeter
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 111, 112
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF97
// 首条位置: {"module":767,"inlinee":44951}
// 编译实例 RVAs: 0xc97970
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::intersect_ray
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 150
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFA5
// 首条位置: {"module":767,"record_offset":2680,"rva":13203840,"va":5381912960}
// 编译实例 RVAs: 0xc97980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":2816,"parent_record_offset":2680,"ancestor_record_offset":2680,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":767,"record_offset":2844,"parent_record_offset":2680,"ancestor_record_offset":2680,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::intersect_circle
// 记录形式: compiled_procedure; instances=1
// 源码行: 167
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":767,"record_offset":2876,"rva":13204032,"va":5381913152}
// 编译实例 RVAs: 0xc97a40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":767,"record_offset":3180,"parent_record_offset":2876,"ancestor_record_offset":2876,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":3036,"parent_record_offset":2876,"ancestor_record_offset":2876,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":3292,"parent_record_offset":2876,"ancestor_record_offset":2876,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":3320,"parent_record_offset":2876,"ancestor_record_offset":2876,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":3376,"parent_record_offset":2876,"ancestor_record_offset":2876,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":3236,"parent_record_offset":2876,"ancestor_record_offset":2876,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":767,"record_offset":3208,"parent_record_offset":2876,"ancestor_record_offset":2876,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":767,"record_offset":3400,"parent_record_offset":3376,"ancestor_record_offset":2876,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":767,"record_offset":3344,"parent_record_offset":3320,"ancestor_record_offset":2876,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":767,"record_offset":3084,"parent_record_offset":3056,"ancestor_record_offset":2876,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":3108,"parent_record_offset":3056,"ancestor_record_offset":2876,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":3056,"parent_record_offset":3036,"ancestor_record_offset":2876,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":767,"record_offset":3140,"parent_record_offset":3108,"ancestor_record_offset":2876,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::is_point_inside
// 记录形式: compiled_procedure; instances=1
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":767,"record_offset":3548,"rva":13204240,"va":5381913360}
// 编译实例 RVAs: 0xc97b10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":3684,"parent_record_offset":3548,"ancestor_record_offset":3548,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":767,"record_offset":3732,"parent_record_offset":3704,"ancestor_record_offset":3548,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":3756,"parent_record_offset":3704,"ancestor_record_offset":3548,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":3704,"parent_record_offset":3684,"ancestor_record_offset":3548,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":767,"record_offset":3784,"parent_record_offset":3756,"ancestor_record_offset":3548,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::closest_point_on_perimeter
// 记录形式: compiled_procedure; instances=1
// 源码行: 191
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":767,"record_offset":3828,"rva":13204288,"va":5381913408}
// 编译实例 RVAs: 0xc97b40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":4036,"parent_record_offset":3828,"ancestor_record_offset":3828,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":4240,"parent_record_offset":3828,"ancestor_record_offset":3828,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":4316,"parent_record_offset":3828,"ancestor_record_offset":3828,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":3988,"parent_record_offset":3828,"ancestor_record_offset":3828,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":767,"record_offset":4340,"parent_record_offset":4316,"ancestor_record_offset":3828,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":767,"record_offset":4180,"parent_record_offset":4056,"ancestor_record_offset":3828,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":767,"record_offset":4008,"parent_record_offset":3988,"ancestor_record_offset":3828,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":4116,"parent_record_offset":4088,"ancestor_record_offset":3828,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":4088,"parent_record_offset":4056,"ancestor_record_offset":3828,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":4056,"parent_record_offset":4036,"ancestor_record_offset":3828,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":767,"record_offset":4144,"parent_record_offset":4116,"ancestor_record_offset":3828,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::as_points2_ptcount
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 195
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF98
// 首条位置: {"module":767,"inlinee":44952}
// 编译实例 RVAs: 0xc97ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":767,"record_offset":4536,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":767,"record_offset":4560,"parent_record_offset":4536,"ancestor_record_offset":4396,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::impl$0::as_points2_ptcount::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=22
// 源码行: 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3788
// 首条位置: {"module":184,"inlinee":14216}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=13
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":184,"record_offset":14256,"parent_record_offset":14224,"ancestor_record_offset":14224,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":3375,"record_offset":4656,"parent_record_offset":4408,"ancestor_record_offset":4408,"inlinee":44961,"source_line":59,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":184,"record_offset":14344,"parent_record_offset":14320,"ancestor_record_offset":14224,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":184,"record_offset":14368,"parent_record_offset":14344,"ancestor_record_offset":14224,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":184,"record_offset":14408,"parent_record_offset":14256,"ancestor_record_offset":14224,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":184,"record_offset":14436,"parent_record_offset":14256,"ancestor_record_offset":14224,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3375,"record_offset":4856,"parent_record_offset":4832,"ancestor_record_offset":4408,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3375,"record_offset":4912,"parent_record_offset":4888,"ancestor_record_offset":4408,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":184,"record_offset":14320,"parent_record_offset":14296,"ancestor_record_offset":14224,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":184,"record_offset":14296,"parent_record_offset":14256,"ancestor_record_offset":14224,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3375,"record_offset":4832,"parent_record_offset":4656,"ancestor_record_offset":4408,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$38::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3375,"record_offset":4888,"parent_record_offset":4656,"ancestor_record_offset":4408,"inlinee":14288,"source_line":1317,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::cos
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3375,"record_offset":4772,"parent_record_offset":4684,"ancestor_record_offset":4408,"inlinee":14893,"source_line":675,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3375,"record_offset":4800,"parent_record_offset":4684,"ancestor_record_offset":4408,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::CircularF32::as_f32
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":3375,"record_offset":4712,"parent_record_offset":4684,"ancestor_record_offset":4408,"inlinee":44950,"source_line":36,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3375,"record_offset":4736,"parent_record_offset":4712,"ancestor_record_offset":4408,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":3375,"record_offset":4684,"parent_record_offset":4656,"ancestor_record_offset":4408,"inlinee":44948,"source_line":80,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::as_points2
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF99
// 首条位置: {"module":767,"inlinee":44953}
// 编译实例 RVAs: 0xc97bc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29934206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381913536,"raw_hex":"e87ec2c801","size":5,"soft_fail":false,"target_va":5411847776,"va":5381913565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":767,"record_offset":4788,"parent_record_offset":4592,"ancestor_record_offset":4592,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":767,"record_offset":4760,"parent_record_offset":4592,"ancestor_record_offset":4592,"inlinee":13632,"source_line":70,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::as_points2_ptcount
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":767,"record_offset":4816,"parent_record_offset":4592,"ancestor_record_offset":4592,"inlinee":44952,"source_line":195,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::perimeter
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":767,"record_offset":4736,"parent_record_offset":4592,"ancestor_record_offset":4592,"inlinee":44951,"source_line":111,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":767,"record_offset":4864,"parent_record_offset":4840,"ancestor_record_offset":4592,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":767,"record_offset":4840,"parent_record_offset":4816,"ancestor_record_offset":4592,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::as_points3
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 207
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF9A
// 首条位置: {"module":767,"inlinee":44954}
// 编译实例 RVAs: 0xc97c50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29934054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381913680,"raw_hex":"e8e6c1c801","size":5,"soft_fail":false,"target_va":5411847776,"va":5381913717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":767,"record_offset":5156,"parent_record_offset":4904,"ancestor_record_offset":4904,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::as_points2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":767,"record_offset":5040,"parent_record_offset":4904,"ancestor_record_offset":4904,"inlinee":44953,"source_line":202,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":767,"record_offset":5124,"parent_record_offset":5040,"ancestor_record_offset":4904,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":767,"record_offset":5180,"parent_record_offset":5156,"ancestor_record_offset":4904,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":767,"record_offset":5096,"parent_record_offset":5040,"ancestor_record_offset":4904,"inlinee":13632,"source_line":70,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::Circle2d::perimeter
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":767,"record_offset":5068,"parent_record_offset":5040,"ancestor_record_offset":4904,"inlinee":44951,"source_line":111,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::as_curve2
// 记录形式: compiled_procedure; instances=1
// 源码行: 212
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":767,"record_offset":5216,"rva":13204720,"va":5381913840}
// 编译实例 RVAs: 0xc97cf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$::from_iter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t13279788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381913840,"raw_hex":"e82ca2ca00","size":5,"soft_fail":false,"target_va":5395193792,"va":5381913999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419401b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13272076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381913840,"raw_hex":"e80c84ca00","size":5,"soft_fail":false,"target_va":5395186096,"va":5381914015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29933898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381913840,"raw_hex":"e84ac1c801","size":5,"soft_fail":false,"target_va":5411847776,"va":5381913873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29628753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381913840,"raw_hex":"e85119c401","size":5,"soft_fail":false,"target_va":5411542880,"va":5381914122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":767,"record_offset":5468,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":767,"record_offset":5552,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":767,"record_offset":5352,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":44953,"source_line":202,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":767,"record_offset":5492,"parent_record_offset":5468,"ancestor_record_offset":5216,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":767,"record_offset":5436,"parent_record_offset":5352,"ancestor_record_offset":5216,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":767,"record_offset":5516,"parent_record_offset":5492,"ancestor_record_offset":5216,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":767,"record_offset":5580,"parent_record_offset":5552,"ancestor_record_offset":5216,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":767,"record_offset":5408,"parent_record_offset":5352,"ancestor_record_offset":5216,"inlinee":13632,"source_line":70,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::Circle2d::perimeter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":767,"record_offset":5380,"parent_record_offset":5352,"ancestor_record_offset":5216,"inlinee":44951,"source_line":111,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::as_curve3
// 记录形式: compiled_procedure; instances=1
// 源码行: 223
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":767,"record_offset":5624,"rva":13205008,"va":5381914128}
// 编译实例 RVAs: 0xc97e10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$::from_iter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t13279131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914128,"raw_hex":"e89b9fca00","size":5,"soft_fail":false,"target_va":5395193440,"va":5381914304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141940470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13272475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914128,"raw_hex":"e89b85ca00","size":5,"soft_fail":false,"target_va":5395186800,"va":5381914320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29933599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914128,"raw_hex":"e81fc0c801","size":5,"soft_fail":false,"target_va":5411847776,"va":5381914172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29628440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914128,"raw_hex":"e81818c401","size":5,"soft_fail":false,"target_va":5411542880,"va":5381914435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":767,"record_offset":5904,"parent_record_offset":5624,"ancestor_record_offset":5624,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":767,"record_offset":5988,"parent_record_offset":5624,"ancestor_record_offset":5624,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::as_points3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":767,"record_offset":5760,"parent_record_offset":5624,"ancestor_record_offset":5624,"inlinee":44954,"source_line":207,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":767,"record_offset":5928,"parent_record_offset":5904,"ancestor_record_offset":5624,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":767,"record_offset":5868,"parent_record_offset":5784,"ancestor_record_offset":5624,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":767,"record_offset":5952,"parent_record_offset":5928,"ancestor_record_offset":5624,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":767,"record_offset":6016,"parent_record_offset":5988,"ancestor_record_offset":5624,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":767,"record_offset":5840,"parent_record_offset":5784,"ancestor_record_offset":5624,"inlinee":13632,"source_line":70,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::Circle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":767,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":5624,"inlinee":44953,"source_line":202,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::Circle2d::perimeter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":767,"record_offset":5812,"parent_record_offset":5784,"ancestor_record_offset":5624,"inlinee":44951,"source_line":111,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::as_curve2_range
// 记录形式: compiled_procedure; instances=1
// 源码行: 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":767,"record_offset":6060,"rva":13205328,"va":5381914448}
// 编译实例 RVAs: 0xc97f50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$::from_iter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t13279481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914448,"raw_hex":"e8f9a0ca00","size":5,"soft_fail":false,"target_va":5395194144,"va":5381914658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419401b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13271417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914448,"raw_hex":"e87981ca00","size":5,"soft_fail":false,"target_va":5395186096,"va":5381914674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29933251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914448,"raw_hex":"e8c3bec801","size":5,"soft_fail":false,"target_va":5411847776,"va":5381914520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29628078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914448,"raw_hex":"e8ae16c401","size":5,"soft_fail":false,"target_va":5411542880,"va":5381914797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":767,"record_offset":6268,"parent_record_offset":6060,"ancestor_record_offset":6060,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":767,"record_offset":6296,"parent_record_offset":6060,"ancestor_record_offset":6060,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":767,"record_offset":6240,"parent_record_offset":6060,"ancestor_record_offset":6060,"inlinee":13632,"source_line":70,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":767,"record_offset":6380,"parent_record_offset":6060,"ancestor_record_offset":6060,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::perimeter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":767,"record_offset":6212,"parent_record_offset":6060,"ancestor_record_offset":6060,"inlinee":44951,"source_line":111,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":767,"record_offset":6320,"parent_record_offset":6296,"ancestor_record_offset":6060,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":767,"record_offset":6344,"parent_record_offset":6320,"ancestor_record_offset":6060,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":767,"record_offset":6408,"parent_record_offset":6380,"ancestor_record_offset":6060,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::impl$0::as_curve2_range::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 241
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C550
// 首条位置: {"module":3375,"inlinee":116048}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3375,"record_offset":1068,"parent_record_offset":1028,"ancestor_record_offset":1028,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":3375,"record_offset":1180,"parent_record_offset":1028,"ancestor_record_offset":1028,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":3375,"record_offset":1392,"parent_record_offset":1028,"ancestor_record_offset":1028,"inlinee":44961,"source_line":59,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3375,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":1028,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3375,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":1028,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3375,"record_offset":1332,"parent_record_offset":1180,"ancestor_record_offset":1028,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3375,"record_offset":1360,"parent_record_offset":1180,"ancestor_record_offset":1028,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3375,"record_offset":1148,"parent_record_offset":1068,"ancestor_record_offset":1028,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3375,"record_offset":1120,"parent_record_offset":1068,"ancestor_record_offset":1028,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3375,"record_offset":1648,"parent_record_offset":1624,"ancestor_record_offset":1028,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3375,"record_offset":1092,"parent_record_offset":1068,"ancestor_record_offset":1028,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3375,"record_offset":1244,"parent_record_offset":1220,"ancestor_record_offset":1028,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3375,"record_offset":1220,"parent_record_offset":1180,"ancestor_record_offset":1028,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3375,"record_offset":1568,"parent_record_offset":1392,"ancestor_record_offset":1028,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3375,"record_offset":1624,"parent_record_offset":1392,"ancestor_record_offset":1028,"inlinee":14288,"source_line":1317,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3375,"record_offset":1508,"parent_record_offset":1420,"ancestor_record_offset":1028,"inlinee":14893,"source_line":675,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3375,"record_offset":1536,"parent_record_offset":1420,"ancestor_record_offset":1028,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::CircularF32::as_f32
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":3375,"record_offset":1448,"parent_record_offset":1420,"ancestor_record_offset":1028,"inlinee":44950,"source_line":36,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3375,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":1028,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":3375,"record_offset":1420,"parent_record_offset":1392,"ancestor_record_offset":1028,"inlinee":44948,"source_line":80,"archive":"_pdb/B/modules/03375.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::with_expanded_diameter
// 记录形式: compiled_procedure; instances=1
// 源码行: 250
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":767,"record_offset":6452,"rva":13205696,"va":5381914816}
// 编译实例 RVAs: 0xc980c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::contains_point
// 记录形式: compiled_procedure; instances=1
// 源码行: 259
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":767,"record_offset":6588,"rva":13205744,"va":5381914864}
// 编译实例 RVAs: 0xc980f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":6776,"parent_record_offset":6588,"ancestor_record_offset":6588,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":6728,"parent_record_offset":6588,"ancestor_record_offset":6588,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":767,"record_offset":6748,"parent_record_offset":6728,"ancestor_record_offset":6588,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":6796,"parent_record_offset":6776,"ancestor_record_offset":6588,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::signed_distance
// 记录形式: compiled_procedure; instances=1
// 源码行: 264
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":767,"record_offset":6828,"rva":13205792,"va":5381914912}
// 编译实例 RVAs: 0xc98120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":6964,"parent_record_offset":6828,"ancestor_record_offset":6828,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":767,"record_offset":7012,"parent_record_offset":6984,"ancestor_record_offset":6828,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":7036,"parent_record_offset":6984,"ancestor_record_offset":6828,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":6984,"parent_record_offset":6964,"ancestor_record_offset":6828,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":767,"record_offset":7064,"parent_record_offset":7036,"ancestor_record_offset":6828,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::aabb
// 记录形式: compiled_procedure; instances=1
// 源码行: 268
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":767,"record_offset":7108,"rva":13205824,"va":5381914944}
// 编译实例 RVAs: 0xc98140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$56::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":7232,"parent_record_offset":7108,"ancestor_record_offset":7108,"inlinee":10959,"source_line":1468,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":767,"record_offset":7252,"parent_record_offset":7232,"ancestor_record_offset":7108,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::lerp
// 记录形式: compiled_procedure; instances=1
// 源码行: 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":767,"record_offset":7284,"rva":13205872,"va":5381914992}
// 编译实例 RVAs: 0xc98170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29932836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914992,"raw_hex":"e824bdc801","size":5,"soft_fail":false,"target_va":5411847952,"va":5381915111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":7908,"parent_record_offset":7284,"ancestor_record_offset":7284,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":767,"record_offset":7792,"parent_record_offset":7284,"ancestor_record_offset":7284,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] utils::circular_f32::CircularF32::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":767,"record_offset":7420,"parent_record_offset":7284,"ancestor_record_offset":7284,"inlinee":44955,"source_line":60,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::new_rotated
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":767,"record_offset":8088,"parent_record_offset":7284,"ancestor_record_offset":7284,"inlinee":44958,"source_line":40,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":767,"record_offset":7876,"parent_record_offset":7792,"ancestor_record_offset":7284,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":767,"record_offset":7820,"parent_record_offset":7792,"ancestor_record_offset":7284,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":7940,"parent_record_offset":7908,"ancestor_record_offset":7284,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":767,"record_offset":8028,"parent_record_offset":7908,"ancestor_record_offset":7284,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::CircularF32::as_f32
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":767,"record_offset":7452,"parent_record_offset":7420,"ancestor_record_offset":7284,"inlinee":44950,"source_line":36,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::CircularF32::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":767,"record_offset":7700,"parent_record_offset":7420,"ancestor_record_offset":7284,"inlinee":44956,"source_line":22,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::CircularF32::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":767,"record_offset":7724,"parent_record_offset":7700,"ancestor_record_offset":7284,"inlinee":44957,"source_line":32,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":767,"record_offset":7568,"parent_record_offset":7420,"ancestor_record_offset":7284,"inlinee":14137,"source_line":25,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":767,"record_offset":7636,"parent_record_offset":7568,"ancestor_record_offset":7284,"inlinee":44786,"source_line":149,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":767,"record_offset":7476,"parent_record_offset":7452,"ancestor_record_offset":7284,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/00767.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::subtract
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":767,"record_offset":7596,"parent_record_offset":7568,"ancestor_record_offset":7284,"inlinee":31608,"source_line":161,"archive":"_pdb/B/modules/00767.jsonl.gz"}
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
// 完整结构及原始类型记录: circle.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::circle::Circle2d
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 28; 过程引用数: 26
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$1::serialize_newtype_variant<utils::geometry::circle::Circle2d>", "constructor": "serde_json::value::ser::impl$1::serialize_newtype_variant", "arguments": ["utils::geometry::circle::Circle2d"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<utils::geometry::circle::Circle2d>", "constructor": "core::fmt::impl$74::fmt", "arguments": ["utils::geometry::circle::Circle2d"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: circle.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-freeform-cache/B-shape-height-helpers/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "utils::geometry::circle::Circle2d", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/B-platform-geometry/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "utils::geometry::circle::Circle2d", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-critters/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "utils::geometry::circle::Circle2d", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: circle.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 15; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>", "bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>
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
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::aabb
// 信息 ID: 7c4cd05fd28d5167d2253e4c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x78756d -> 0xc98140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_circle<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_circle::closure_env$0> @ 0x9f0961 -> 0xc98140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_circle<country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug::closure_env$2> @ 0x9f0ee9 -> 0xc98140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<forest::manage_tree_prefabs::manage_tree_prefabs::closure_env$0> @ 0x227863f -> 0xc98140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<system_tree::convert_preplaced_trees::convert_preplaced_trees::closure_env$0> @ 0x25df03f -> 0xc98140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<system_tree::hide_trees::hide_trees::closure_env$0> @ 0x25df38f -> 0xc98140 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::area
// 信息 ID: 7bba5b777a773410a869fc80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_create_circle::ui_create_circle @ 0x165cc59 -> 0xc97580 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_create_circle::ui_create_circle @ 0x165c749 -> 0xc97580 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio @ 0x17d2efd -> 0xc97580 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio @ 0x17d32e2 -> 0xc97580 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::get_building_size_01 @ 0x17d3f41 -> 0xc97580 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::as_curve2
// 信息 ID: 807629abd2747d94746ba0fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::structure_outline::structure_outline @ 0x95af91 -> 0xc97cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0x97c23f -> 0xc97cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0x9ad368 -> 0xc97cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::calc_shape_height::calc_shape_height @ 0xac8167 -> 0xc97cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2 @ 0x11ff2d9 -> 0xc97cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0x1248c26 -> 0xc97cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0x1249d8d -> 0xc97cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0x124d5b5 -> 0xc97cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_full_glade_size::demo_show_full_glade_size @ 0x1eacbd4 -> 0xc97cf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::as_curve2_range
// 信息 ID: a5e7a68426503a6275349080
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line @ 0xa6509a -> 0xc97f50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::as_curve3
// 信息 ID: ef8352c83c0edcc5f579e58c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line @ 0x8dff25 -> 0xc97e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0x8ec454 -> 0xc97e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::structure_outline::structure_outline::closure$2 @ 0x96080e -> 0xc97e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode::closure$1::closure_env$0,tuple$<> > | country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode::closure$1::closure$0 | core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::circle::ui_edit_circle::show_full_circle_signifier::closure$0::closure_env$0,tuple$<> > | country_core::systems::wall::shapes::circle::ui_edit_circle::show_full_circle_signifier::closure$0::closure$0 @ 0x9a363a -> 0xc97e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode @ 0x9a53ab -> 0xc97e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve3 @ 0x11ff31f -> 0xc97e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::circle::ui_create_circle::circle_signifier::closure$0::closure_env$0,tuple$<> > | country_core::systems::wall::shapes::circle::ui_create_circle::circle_signifier::closure$0::closure$0 @ 0x165b348 -> 0xc97e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle @ 0x165e051 -> 0xc97e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::construct_inter_roof_merlons::construct_inter_roof_merlons @ 0x22d4e27 -> 0xc97e10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::as_points2
// 信息 ID: bf15ac029d1b0e5bc63ecce9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x8d35da -> 0xc97bc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::create_circle::create_circle @ 0x97e9d8 -> 0xc97bc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::is_shape_inside @ 0xad2b3c -> 0xc97bc0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_points3; module=767, inline_record=5040; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_curve2; module=767, inline_record=5352; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_curve3; module=767, inline_record=5784; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::as_points2_ptcount
// 信息 ID: 2ccd6a006d5ef81b91030624
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] timberframe::timberframe_circle::construct_timberframe_circle @ 0x21b6af4 -> 0xc97ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_circle::construct_timberframe_circle @ 0x21b6db8 -> 0xc97ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_circle::construct_timberframe_circle @ 0x21b7288 -> 0xc97ba0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_points2; module=767, inline_record=4816; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::as_points3
// 信息 ID: b6eed040049c06bf5c821634
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0xe563 -> 0xc97c50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x1d9a3 -> 0xc97c50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a4f7b -> 0xc97c50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x8d35a5 -> 0xc97c50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::ui_autoclutter::ui_convert_autoclutter_to_decoclutter @ 0x99e168 -> 0xc97c50 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_curve3; module=767, inline_record=5760; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::closest_point_on_perimeter
// 信息 ID: fc75f0555243f3d1537d013f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds @ 0x81166c -> 0xc97b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6> @ 0x1290c97 -> 0xc97b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_birdnests::add_birdnests::closure$0 @ 0x22caa13 -> 0xc97b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_birdnests::add_birdnests::closure$0 @ 0x22cab89 -> 0xc97b40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::contains_point
// 信息 ID: 20a8e06832b3dff5c9708b04
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 25 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_move_freehand_walls::ui_move_freehand_walls @ 0x978fdd -> 0xc980f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_freehand_walls::ui_move_freehand_walls @ 0x979015 -> 0xc980f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_pruner::ivy_pruner @ 0x997ade -> 0xc980f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_pruner::ivy_pruner @ 0x997a8e -> 0xc980f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::ui_continue_wall::ui_continue_wall @ 0xaaebdf -> 0xc980f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::ui_draw_wall::ui_draw_wall @ 0xab0557 -> 0xc980f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::ui_draw_wall::ui_draw_wall @ 0xab0034 -> 0xc980f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::is_pos_inside | country_core::systems::wall::valid_enclosure::ShapeParameters::contains_point @ 0xad2960 -> 0xc980f0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::is_shape_inside @ 0xad2aae -> 0xc980f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::is_shape_inside @ 0xad2ac8 -> 0xc980f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::is_shape_inside @ 0xad2ae2 -> 0xc980f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::is_shape_inside @ 0xad2afc -> 0xc980f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x11ff3f7 -> 0xc980f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x11ff41a -> 0xc980f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x11ff43d -> 0xc980f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x11ff466 -> 0xc980f0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::get_closest_u_from_pos
// 信息 ID: 3127cfc8915d729bdc5c779a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x776191 -> 0xc976f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x7761ae -> 0xc976f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::get_closest_u_from_pos @ 0x11ff670 -> 0xc976f0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::get_normal_at_u
// 信息 ID: efc77de6dcac2578c865e21c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x776a69 -> 0xc978e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::get_normal_at_u @ 0x11ff610 -> 0xc978e0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121dfd4 -> 0xc978e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> > @ 0x122780f -> 0xc978e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0x124bd58 -> 0xc978e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0x124be78 -> 0xc978e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b377e -> 0xc978e0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::get_pos_at_u; module=767, inline_record=784; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::get_tangent_at_u; module=767, inline_record=1252; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0> > > >; module=3375, inline_record=1420; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0> > > >; module=3375, inline_record=4684; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> > > >; module=3375, inline_record=6624; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::get_pos_at_u
// 信息 ID: e9a30f33a8a1a268e468e30e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 44 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> > > > @ 0x48d2cf -> 0xc975a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line @ 0xa64b0b -> 0xc975a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line @ 0xa65054 -> 0xc975a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::is_shape_inside @ 0xad2c1c -> 0xc975a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x11ff471 -> 0xc975a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x11ff497 -> 0xc975a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x11ff4bd -> 0xc975a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x11ff4e3 -> 0xc975a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x11ff509 -> 0xc975a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x11ff52f -> 0xc975a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x11ff53f -> 0xc975a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x11ff565 -> 0xc975a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x11ff587 -> 0xc975a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x11ff5a9 -> 0xc975a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x11ff5cb -> 0xc975a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x11ff5ed -> 0xc975a0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0> > > >; module=3375, inline_record=1392; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0> > > >; module=3375, inline_record=4656; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> > > >; module=3375, inline_record=6596; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::get_tangent_at_u
// 信息 ID: 8e1d97a0585e1378eecc0541
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::systems::nani_update_stone_floor @ 0x2079198 -> 0xc97660 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::intersect_circle
// 信息 ID: d2e459d8822f261c5aa302ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::trap_doors::generate_trap_doors @ 0x7bef1c -> 0xc97a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0xedc66a -> 0xc97a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0xedd708 -> 0xc97a40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::intersect_ray
// 信息 ID: a9c32b9e0954efdda60e328d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0x97be93 -> 0xc97980 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0x97bf78 -> 0xc97980 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::eraser::wall_eraser::wall_eraser @ 0xacda07 -> 0xc97980 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0xedc43c -> 0xc97980 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0xedd603 -> 0xc97980 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::utils_intersect::line_x_shape_overlap @ 0x175af8b -> 0xc97980 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::intersects_circle; module=768, inline_record=38500; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::is_point_inside
// 信息 ID: e4a62c460d68989d33a8e133
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_categorize::circle_categorize @ 0x124b1c5 -> 0xc97b10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::lerp
// 信息 ID: 535c0736031476b86012c3c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::search_lerped_shape_that_fits_into_glade @ 0xad2ef4 -> 0xc98170 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::search_lerped_shape_that_fits_into_glade @ 0xad2f9d -> 0xc98170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::new
// 信息 ID: 6cbe39006767ebf4ae584b20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 7d9da2e094c5e3aaa2b1ca08: void (*)(bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 24 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a4f68 -> 0xc97540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0x8ec405 -> 0xc97540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::ui_autoclutter::ui_convert_autoclutter_to_decoclutter @ 0x99e14e -> 0xc97540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode @ 0x9a5384 -> 0xc97540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line @ 0xa64afb -> 0xc97540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line @ 0xa65049 -> 0xc97540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0xa7edcc -> 0xc97540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::terrain::ui_draw_terrain::ui_draw_terrain @ 0xa9ff96 -> 0xc97540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::eraser::wall_eraser::wall_eraser @ 0xacd351 -> 0xc97540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::garden::draw_garden::draw_garden @ 0xae49eb -> 0xc97540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::garden::draw_garden::draw_garden @ 0xae4831 -> 0xc97540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::paths::draw_path::draw_path @ 0xae4f3c -> 0xc97540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::paths::draw_path::draw_path @ 0xae514c -> 0xc97540 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::commands::Commands)> > @ 0xaf2248 -> 0xc97540 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::commands::Commands)> > > @ 0x14d99aa -> 0xc97540 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::commands::Commands)> > > @ 0x150622d -> 0xc97540 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::new_rotated
// 信息 ID: 13d4e38ce7a76341bec4e8ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 15 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::trap_doors::generate_trap_doors @ 0x7bef05 -> 0xc97560 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y @ 0x8e3a51 -> 0xc97560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0x124d4ad -> 0xc97560 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle @ 0x165e014 -> 0xc97560 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle @ 0x165e205 -> 0xc97560 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle @ 0x165f16d -> 0xc97560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x21a3920 -> 0xc97560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x21a3a61 -> 0xc97560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x21a42ed -> 0xc97560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x21a4312 -> 0xc97560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b3403 -> 0xc97560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::shared::critters_avoid_collisions @ 0x2318384 -> 0xc97560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::shared::critters_avoid_collisions @ 0x23187be -> 0xc97560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::shared::move_critters @ 0x2319c10 -> 0xc97560 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::shared::move_critters @ 0x2319d69 -> 0xc97560 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::lerp; module=767, inline_record=8088; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::perimeter
// 信息 ID: 4c6b955070277a6c2c6cdfb1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 10 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::structure_outline::structure_outline @ 0x95b049 -> 0xc97970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter @ 0x11ff650 -> 0xc97970 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121df90 -> 0xc97970 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0x1249d59 -> 0xc97970 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer @ 0x124b366 -> 0xc97970 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0x124baf5 -> 0xc97970 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0x124d5bd -> 0xc97970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_create_circle::circle_signifier::closure$0 @ 0x165d21c -> 0xc97970 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b3444 -> 0xc97970 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_circle::construct_timberframe_circle @ 0x21b6a84 -> 0xc97970 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_points2; module=767, inline_record=4736; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_points3; module=767, inline_record=5068; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_curve2; module=767, inline_record=5380; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_curve3; module=767, inline_record=5812; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_curve2_range; module=767, inline_record=6212; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::signed_distance
// 信息 ID: 3d9c4f4e24e6f64c33dcc590
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0x124bf62 -> 0xc98120 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door @ 0x127f73f -> 0xc98120 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door @ 0x127f762 -> 0xc98120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::with_expanded_diameter
// 信息 ID: 78dc0ef3f177a0cfe3092745
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_tree::hide_trees::hide_trees @ 0x226ad6f -> 0xc980c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::yaw_radians
// 信息 ID: 3d72155b78718ae7b592247b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::Roof::rotation_yaw_radians; module=498, inline_record=27812; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_clutter::rmb_autoclutter::handle_rmb_autoclutter; module=519, inline_record=13156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize
// 信息 ID: 17118025097cbc5c9e90d0e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum<serde::__private::de::content::EnumRefDeserializer<serde_json::error::Error> >; module=1673, inline_record=69828; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: 9cb70a38a8aa4576341efe1c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impl$1::fmt<utils::geometry::circle::_::impl$0::deserialize::__FieldVisitor> @ 0x16e7030 -> 0xc98340 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_bytes<serde_json::error::Error>
// 信息 ID: 1410531d602a85b4f6af3e77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_bytes -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 8d7e06bf8369a1e606cda286
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=8388; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=8752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_u64
// 信息 ID: 629ba15f0db9f12958dafa7a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=8160; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=8528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 175c04b4d3d24bfbf1d76a5b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=8036; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 10cbdc763b43303f55a462e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impl$1::fmt<utils::geometry::circle::_::impl$0::deserialize::__Visitor> @ 0x16e6fb0 -> 0xc98360 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 0d3e88505d2881d1a0072054
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=7660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 9a47e775b9a296993d7c3cff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=10388; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::serialize
// 信息 ID: e1cad29a9967038c3a65adac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1673, inline_record=64576; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: e675fbef868ebf11f23d5745
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] utils::geometry::circle::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::valid_enclosure::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x259640 -> 0x3d8a70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: ae25342bc398d59f92dc5932
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::geometry::circle::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$1::serialize_newtype_variant<utils::geometry::circle::Circle2d> @ 0x1591d4f -> 0x16e7940 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::impl$0::as_curve2_range::closure$0
// 信息 ID: 4984c73d1051c4ec44029a40
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::geometry::circle::impl$0::as_curve2_range", "captures": "unknown"}
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_curve2_range::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_curve2_range::closure_env$0 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_curve2_range::closure_env$0 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0> > > >; module=3375, inline_record=1028; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::impl$0::as_points2_ptcount::closure$0
// 信息 ID: 09c810eb7a45d2f81932339c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::geometry::circle::impl$0::as_points2_ptcount", "captures": "unknown"}
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 13 条；闭包环境类型引用 19 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> > > >; module=184, inline_record=14224; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::glade_border::GladeBorder::is_shape_inside; module=567, inline_record=10620; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> > > >; module=1546, inline_record=1248; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> > > >; module=1610, inline_record=19200; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0> > > >; module=1610, inline_record=86924; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0> > > >; module=3375, inline_record=4408; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> > > >; module=3375, inline_record=6348; 不是独立调用边。
// [PDB 内联宿主] timberframe::timberframe_circle::construct_timberframe_circle; module=3485, inline_record=2584; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::impl$3::fmt
// 信息 ID: 3319c330951f1d94504d884e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<utils::geometry::circle::Circle2d>; module=1673, inline_record=3672; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$74::fmt<utils::geometry::circle::Circle2d>; module=4658, inline_record=404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::impl$6::eq
// 信息 ID: d876f16d77481ade747a9ba6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners; module=2599, inline_record=22588; 不是独立调用边。
// [PDB 内联宿主] system_roof::update_roof_spatial::update_roof_spatial; module=3465, inline_record=4320; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::is_default
// 信息 ID: 794796a2d9e37b9b69e7b4b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::circle::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=172, inline_record=63116; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=1673, inline_record=10960; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1673, inline_record=64616; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
