// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/theme-fetcher/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: f2f84f1ae48471442feb246ee14c7db8d939d3516a7c4650800d012a68c4595c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 228, 258, 259, 261, 845, 847, 863, 866, 2163, 2191, 2192
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use arc_swap; // namespace candidate
//   package identities: [{"name":"arc-swap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.7.1"}]
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.7.4"}]
// use blake3; // namespace candidate
//   package identities: [{"name":"blake3","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.5.3"}]
// use byteorder; // namespace candidate
//   package identities: [{"name":"byteorder","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.5.0"}]
// use bytes; // namespace candidate
//   package identities: [{"name":"bytes","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.6.0"}]
// use cache_control; // namespace candidate
//   package identities: [{"name":"cache_control","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.2.0"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use http; // namespace candidate
//   package identities: [{"name":"http","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.2.9"}]
// use httpdate; // namespace candidate
//   package identities: [{"name":"httpdate","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.3"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use memmap2; // namespace candidate
//   package identities: [{"name":"memmap2","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.8"},{"name":"memmap2","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.4"}]
// use reqwest; // namespace candidate
//   package identities: [{"name":"reqwest","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.22"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use theme_defs; // namespace candidate
//   package identities: [{"name":"theme-defs","source":"local_or_path","version":"0.1.0"}]
// use theme_fetcher; // namespace candidate
//   package identities: [{"name":"theme-fetcher","source":"local_or_path","version":"0.1.0"}]
// use zip; // namespace candidate
//   package identities: [{"name":"zip","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.6.6"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: serde_json::de::Deserializer::parse_exponent<serde_json::read::SliceRead>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":258,"record_offset":7188,"rva":360976,"va":5369070096}
// 编译实例 RVAs: 0x58210
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a0fa0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42240917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369070096,"raw_hex":"e8958b8402","size":5,"soft_fail":false,"target_va":5411311520,"va":5369070598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a4c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42256808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369070096,"raw_hex":"e8a8c98402","size":5,"soft_fail":false,"target_va":5411327120,"va":5369070307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$2::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":258,"record_offset":7776,"parent_record_offset":7188,"ancestor_record_offset":7188,"inlinee":6492,"source_line":1668,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$2::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":258,"record_offset":7804,"parent_record_offset":7188,"ancestor_record_offset":7188,"inlinee":6493,"source_line":1714,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":7456,"parent_record_offset":7188,"ancestor_record_offset":7188,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::f64_from_parts
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":7832,"parent_record_offset":7188,"ancestor_record_offset":7188,"inlinee":6482,"source_line":562,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::next_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":7560,"parent_record_offset":7188,"ancestor_record_offset":7188,"inlinee":6476,"source_line":224,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::peek_or_null
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":7368,"parent_record_offset":7188,"ancestor_record_offset":7188,"inlinee":6480,"source_line":216,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_infinite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":258,"record_offset":8024,"parent_record_offset":7832,"ancestor_record_offset":7188,"inlinee":6486,"source_line":546,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::wrapping_abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":258,"record_offset":7900,"parent_record_offset":7832,"ancestor_record_offset":7188,"inlinee":6484,"source_line":2192,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":258,"record_offset":7988,"parent_record_offset":7956,"ancestor_record_offset":7188,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":258,"record_offset":7956,"parent_record_offset":7900,"ancestor_record_offset":7188,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":7392,"parent_record_offset":7368,"ancestor_record_offset":7188,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":258,"record_offset":7476,"parent_record_offset":7456,"ancestor_record_offset":7188,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":258,"record_offset":7584,"parent_record_offset":7560,"ancestor_record_offset":7188,"inlinee":6477,"source_line":489,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":258,"record_offset":7416,"parent_record_offset":7392,"ancestor_record_offset":7188,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":258,"record_offset":4660,"rva":6374144,"va":5375083264}
// 编译实例 RVAs: 0x614300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a0920
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36225908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375083264,"raw_hex":"e874c32802","size":5,"soft_fail":false,"target_va":5411309856,"va":5375083943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a0fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36227724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375083264,"raw_hex":"e88cca2802","size":5,"soft_fail":false,"target_va":5411311520,"va":5375083791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a4990
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36242800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375083264,"raw_hex":"e870052902","size":5,"soft_fail":false,"target_va":5411326352,"va":5375083547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-1134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375083264,"raw_hex":"e892fbffff","size":5,"soft_fail":false,"target_va":5375082688,"va":5375083817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::de::Deserializer::parse_exponent<serde_json::read::SliceRead>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6013291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375083264,"raw_hex":"e8953ea4ff","size":5,"soft_fail":false,"target_va":5369070096,"va":5375083382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":5200,"parent_record_offset":4660,"ancestor_record_offset":4660,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::next_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":4828,"parent_record_offset":4660,"ancestor_record_offset":4660,"inlinee":6476,"source_line":224,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::parse_number
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":4968,"parent_record_offset":4660,"ancestor_record_offset":4660,"inlinee":6728,"source_line":435,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::peek_or_null
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":4884,"parent_record_offset":4660,"ancestor_record_offset":4660,"inlinee":6480,"source_line":216,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":4908,"parent_record_offset":4884,"ancestor_record_offset":4660,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":258,"record_offset":5224,"parent_record_offset":5200,"ancestor_record_offset":4660,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":258,"record_offset":4848,"parent_record_offset":4828,"ancestor_record_offset":4660,"inlinee":6477,"source_line":489,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":258,"record_offset":4932,"parent_record_offset":4908,"ancestor_record_offset":4660,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: std::io::cursor::impl$5::read_to_string<ref$<bytes::bytes::Bytes> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2191,"record_offset":3632,"rva":27065024,"va":5395774144}
// 编译实例 RVAs: 0x19cfac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16072874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395774144,"raw_hex":"e8aa40f500","size":5,"soft_fail":false,"target_va":5411847168,"va":5395774289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15553067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395774144,"raw_hex":"e82b52ed00","size":5,"soft_fail":false,"target_va":5411327440,"va":5395774368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::string::String::try_reserve
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12926838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395774144,"raw_hex":"e88ac03aff","size":5,"soft_fail":false,"target_va":5382847408,"va":5395774241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12872604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395774144,"raw_hex":"e864943bff","size":5,"soft_fail":false,"target_va":5382901600,"va":5395774199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::String::push_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2191,"record_offset":4056,"parent_record_offset":3632,"ancestor_record_offset":3632,"inlinee":6418,"source_line":1126,"archive":"_pdb/B/modules/02191.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2191,"record_offset":3996,"parent_record_offset":3632,"ancestor_record_offset":3632,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/02191.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2191,"record_offset":4028,"parent_record_offset":3632,"ancestor_record_offset":3632,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/02191.jsonl.gz"}
//   [pdb_inline_direct] std::io::cursor::Cursor::split
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":2191,"record_offset":3796,"parent_record_offset":3632,"ancestor_record_offset":3632,"inlinee":21422,"source_line":233,"archive":"_pdb/B/modules/02191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2191,"record_offset":4208,"parent_record_offset":4180,"ancestor_record_offset":3632,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/02191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2191,"record_offset":4300,"parent_record_offset":4272,"ancestor_record_offset":3632,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/02191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2191,"record_offset":4272,"parent_record_offset":4240,"ancestor_record_offset":3632,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/02191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2191,"record_offset":4240,"parent_record_offset":4208,"ancestor_record_offset":3632,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/02191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2191,"record_offset":4140,"parent_record_offset":4112,"ancestor_record_offset":3632,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/02191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2191,"record_offset":4084,"parent_record_offset":4056,"ancestor_record_offset":3632,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/02191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2191,"record_offset":4388,"parent_record_offset":4140,"ancestor_record_offset":3632,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/02191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2191,"record_offset":4180,"parent_record_offset":4140,"ancestor_record_offset":3632,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/02191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":2191,"record_offset":4112,"parent_record_offset":4084,"ancestor_record_offset":3632,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/02191.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2191,"record_offset":3852,"parent_record_offset":3796,"ancestor_record_offset":3632,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/02191.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":2191,"record_offset":3824,"parent_record_offset":3796,"ancestor_record_offset":3632,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/02191.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2191,"record_offset":3952,"parent_record_offset":3928,"ancestor_record_offset":3632,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/02191.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2191,"record_offset":4352,"parent_record_offset":4140,"ancestor_record_offset":3632,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/02191.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2191,"record_offset":3880,"parent_record_offset":3796,"ancestor_record_offset":3632,"inlinee":6334,"source_line":1916,"archive":"_pdb/B/modules/02191.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2191,"record_offset":3904,"parent_record_offset":3880,"ancestor_record_offset":3632,"inlinee":6335,"source_line":2115,"archive":"_pdb/B/modules/02191.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2191,"record_offset":3928,"parent_record_offset":3904,"ancestor_record_offset":3632,"inlinee":6336,"source_line":2001,"archive":"_pdb/B/modules/02191.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: std::io::default_read_to_end<zip::read::ZipFile>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2163,"record_offset":1856,"rva":26931568,"va":5395640688}
// 编译实例 RVAs: 0x19af170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$zip..read..ZipFile$u20$as$u20$std..io..Read$GT$::read
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395640688,"raw_hex":"e858f1ffff","size":5,"soft_fail":false,"target_va":5395637408,"va":5395641155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20559767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395640688,"raw_hex":"e86948c6fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5395641266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a2970
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15677057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395640688,"raw_hex":"e88136ef00","size":5,"soft_fail":false,"target_va":5411318128,"va":5395641066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16206068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395640688,"raw_hex":"e8f448f700","size":5,"soft_fail":false,"target_va":5411847200,"va":5395641127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15900552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395640688,"raw_hex":"e8889ff200","size":5,"soft_fail":false,"target_va":5411542064,"va":5395641507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::default_read_to_end::small_probe_read<zip::read::ZipFile>
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395640688,"raw_hex":"e88cfdffff","size":5,"soft_fail":false,"target_va":5395640224,"va":5395640847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5395640688,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5395641243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2163,"record_offset":2036,"parent_record_offset":1856,"ancestor_record_offset":1856,"inlinee":7415,"source_line":1246,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2163,"record_offset":2120,"parent_record_offset":1856,"ancestor_record_offset":1856,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2163,"record_offset":4424,"parent_record_offset":1856,"ancestor_record_offset":1856,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::spare_capacity_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2163,"record_offset":3024,"parent_record_offset":1856,"ancestor_record_offset":1856,"inlinee":8644,"source_line":2876,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::try_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2163,"record_offset":2560,"parent_record_offset":1856,"ancestor_record_offset":1856,"inlinee":9937,"source_line":1339,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2163,"record_offset":3084,"parent_record_offset":1856,"ancestor_record_offset":1856,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2163,"record_offset":4452,"parent_record_offset":1856,"ancestor_record_offset":1856,"inlinee":11838,"source_line":1908,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2163,"record_offset":2148,"parent_record_offset":1856,"ancestor_record_offset":1856,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2163,"record_offset":2296,"parent_record_offset":1856,"ancestor_record_offset":1856,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2163,"record_offset":2324,"parent_record_offset":1856,"ancestor_record_offset":1856,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2163,"record_offset":3744,"parent_record_offset":1856,"ancestor_record_offset":1856,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_direct] std::io::Read::read_buf
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\mod.rs
//     evidence={"module":2163,"record_offset":3140,"parent_record_offset":1856,"ancestor_record_offset":1856,"inlinee":21853,"source_line":990,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_direct] std::io::error::Error::is_interrupted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error.rs
//     evidence={"module":2163,"record_offset":3548,"parent_record_offset":1856,"ancestor_record_offset":1856,"inlinee":8607,"source_line":1002,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2163,"record_offset":4048,"parent_record_offset":4024,"ancestor_record_offset":1856,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2163,"record_offset":4024,"parent_record_offset":3940,"ancestor_record_offset":1856,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":2163,"record_offset":3940,"parent_record_offset":3912,"ancestor_record_offset":1856,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2163,"record_offset":2060,"parent_record_offset":2036,"ancestor_record_offset":1856,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2163,"record_offset":2588,"parent_record_offset":2560,"ancestor_record_offset":1856,"inlinee":9938,"source_line":340,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2163,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":1856,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2163,"record_offset":2908,"parent_record_offset":2644,"ancestor_record_offset":1856,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::grow_amortized
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2163,"record_offset":2644,"parent_record_offset":2616,"ancestor_record_offset":1856,"inlinee":14190,"source_line":635,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::set_ptr_and_cap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2163,"record_offset":2976,"parent_record_offset":2644,"ancestor_record_offset":1856,"inlinee":13455,"source_line":627,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2163,"record_offset":2616,"parent_record_offset":2588,"ancestor_record_offset":1856,"inlinee":14189,"source_line":572,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2163,"record_offset":2768,"parent_record_offset":2644,"ancestor_record_offset":1856,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":2163,"record_offset":3968,"parent_record_offset":3940,"ancestor_record_offset":1856,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":2163,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":1856,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":2163,"record_offset":2816,"parent_record_offset":2792,"ancestor_record_offset":1856,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2163,"record_offset":2736,"parent_record_offset":2712,"ancestor_record_offset":1856,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2163,"record_offset":3108,"parent_record_offset":3084,"ancestor_record_offset":1856,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$21::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2163,"record_offset":2348,"parent_record_offset":2324,"ancestor_record_offset":1856,"inlinee":4913,"source_line":1740,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2163,"record_offset":2712,"parent_record_offset":2644,"ancestor_record_offset":1856,"inlinee":11873,"source_line":1567,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2163,"record_offset":2228,"parent_record_offset":2192,"ancestor_record_offset":1856,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2163,"record_offset":3420,"parent_record_offset":3220,"ancestor_record_offset":1856,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::io::borrowed_buf::BorrowedCursor::advance
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\io\borrowed_buf.rs
//     evidence={"module":2163,"record_offset":3508,"parent_record_offset":3180,"ancestor_record_offset":1856,"inlinee":21858,"source_line":283,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::io::borrowed_buf::BorrowedCursor::ensure_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\io\borrowed_buf.rs
//     evidence={"module":2163,"record_offset":3220,"parent_record_offset":3180,"ancestor_record_offset":1856,"inlinee":21855,"source_line":310,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::io::borrowed_buf::BorrowedCursor::uninit_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\io\borrowed_buf.rs
//     evidence={"module":2163,"record_offset":3248,"parent_record_offset":3220,"ancestor_record_offset":1856,"inlinee":40599,"source_line":254,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":2163,"record_offset":3992,"parent_record_offset":3968,"ancestor_record_offset":1856,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2163,"record_offset":2192,"parent_record_offset":2172,"ancestor_record_offset":1856,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2163,"record_offset":2840,"parent_record_offset":2816,"ancestor_record_offset":1856,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2163,"record_offset":3772,"parent_record_offset":3744,"ancestor_record_offset":1856,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2163,"record_offset":3052,"parent_record_offset":3024,"ancestor_record_offset":1856,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2163,"record_offset":2944,"parent_record_offset":2644,"ancestor_record_offset":1856,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2163,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":1856,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2163,"record_offset":3372,"parent_record_offset":3348,"ancestor_record_offset":1856,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2163,"record_offset":3348,"parent_record_offset":3320,"ancestor_record_offset":1856,"inlinee":5242,"source_line":112,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2163,"record_offset":3320,"parent_record_offset":3296,"ancestor_record_offset":1856,"inlinee":5241,"source_line":413,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2163,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":1856,"inlinee":5240,"source_line":553,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] std::io::Read::read_buf::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\mod.rs
//     evidence={"module":2163,"record_offset":3452,"parent_record_offset":3180,"ancestor_record_offset":1856,"inlinee":21857,"source_line":991,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] std::io::default_read_buf
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\mod.rs
//     evidence={"module":2163,"record_offset":3180,"parent_record_offset":3140,"ancestor_record_offset":1856,"inlinee":21854,"source_line":586,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] std::io::default_read_to_end::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\mod.rs
//     evidence={"module":2163,"record_offset":2172,"parent_record_offset":2148,"ancestor_record_offset":1856,"inlinee":51410,"source_line":419,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::impl$20::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error.rs
//     evidence={"module":2163,"record_offset":3588,"parent_record_offset":3548,"ancestor_record_offset":1856,"inlinee":8610,"source_line":219,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::Repr::data
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":2163,"record_offset":3616,"parent_record_offset":3548,"ancestor_record_offset":1856,"inlinee":8608,"source_line":217,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":2163,"record_offset":3644,"parent_record_offset":3616,"ancestor_record_offset":1856,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":2163,"record_offset":3800,"parent_record_offset":3772,"ancestor_record_offset":1856,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::kind_from_prim
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":2163,"record_offset":3676,"parent_record_offset":3644,"ancestor_record_offset":1856,"inlinee":8613,"source_line":299,"archive":"_pdb/B/modules/02163.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::extract_with_checksums<std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":845,"record_offset":2588,"rva":14229952,"va":5382939072}
// 编译实例 RVAs: 0xd921c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..vec..Vec$LT$u8$GT$$u20$as$u20$std..io..copy..BufferedWriterSpec$GT$::buffer_size
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-227498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e85687fcff","size":5,"soft_fail":false,"target_va":5382712512,"va":5382940005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$blake3..Hash$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t142405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8452c0200","size":5,"soft_fail":false,"target_va":5383082784,"va":5382940374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d1240
//     owner=unresolved_namespace; count=2
//     target_aliases=8 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8134134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e80ae283ff","size":5,"soft_fail":false,"target_va":5374808640,"va":5382942769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8042232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8084985ff","size":5,"soft_fail":false,"target_va":5374900736,"va":5382942963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8020296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8b89e85ff","size":5,"soft_fail":false,"target_va":5374923872,"va":5382944163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7858455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8e91688ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5382939938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=28
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7858321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e86f1788ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5382939820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d58e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-235358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8a268fcff","size":5,"soft_fail":false,"target_va":5382704752,"va":5382940105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=5
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-111907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8dd4afeff","size":5,"soft_fail":false,"target_va":5382827728,"va":5382939630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140db7df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t154359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8f75a0200","size":5,"soft_fail":false,"target_va":5383093744,"va":5382939380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419af170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12700656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8f0cbc100","size":5,"soft_fail":false,"target_va":5395640688,"va":5382940027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419b02e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12702865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e891d4c100","size":5,"soft_fail":false,"target_va":5395645152,"va":5382942282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28907713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8c118b901","size":5,"soft_fail":false,"target_va":5411847168,"va":5382939450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8584632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e848027dff","size":5,"soft_fail":false,"target_va":5374355952,"va":5382940579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28598453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8b560b401","size":5,"soft_fail":false,"target_va":5411540963,"va":5382942505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7816990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8e2b888ff","size":5,"soft_fail":false,"target_va":5375124464,"va":5382941449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::extend_from_slice
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t153620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e814580200","size":5,"soft_fail":false,"target_va":5383095824,"va":5382942199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h6ddf3353abc1c5bf (.llvm.12582498285691889218)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t28604304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e89077b401","size":5,"soft_fail":false,"target_va":5411545360,"va":5382941051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t28605448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8087cb401","size":5,"soft_fail":false,"target_va":5411546576,"va":5382941123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hasher::finalize
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t149384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e888470200","size":5,"soft_fail":false,"target_va":5383090912,"va":5382941523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hasher::update
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t147146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8ca3e0200","size":5,"soft_fail":false,"target_va":5383087600,"va":5382940449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t145957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8253a0200","size":5,"soft_fail":false,"target_va":5383086224,"va":5382940262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28599308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e80c64b401","size":5,"soft_fail":false,"target_va":5411541776,"va":5382942463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$theme_fetcher..checked_zip..FolderChecksumInfo$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8c8e1ffff","size":5,"soft_fail":false,"target_va":5382934640,"va":5382942371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$zip..read..ZipFile$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-10995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e80dd5ffff","size":5,"soft_fail":false,"target_va":5382929728,"va":5382940718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28600434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e87268b401","size":5,"soft_fail":false,"target_va":5411542880,"va":5382942441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde::ser::SerializeMap::serialize_entry
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12700530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e872cbc100","size":5,"soft_fail":false,"target_va":5395642368,"va":5382941833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-246300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8e43dfcff","size":5,"soft_fail":false,"target_va":5382694656,"va":5382940951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-230065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e84f7dfcff","size":5,"soft_fail":false,"target_va":5382709584,"va":5382939644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-232212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8ec74fcff","size":5,"soft_fail":false,"target_va":5382708672,"va":5382940879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-233851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8856efcff","size":5,"soft_fail":false,"target_va":5382706224,"va":5382940070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::Write::write_all
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12702118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8a6d1c100","size":5,"soft_fail":false,"target_va":5395644384,"va":5382942261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-195465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e87704fdff","size":5,"soft_fail":false,"target_va":5382744080,"va":5382939540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::parent
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-198484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8acf8fcff","size":5,"soft_fail":false,"target_va":5382741200,"va":5382939679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::stat
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-163121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8cf82fdff","size":5,"soft_fail":false,"target_va":5382776592,"va":5382939708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zip::read::ZipFile::enclosed_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12697874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e812c1c100","size":5,"soft_fail":false,"target_va":5395637376,"va":5382939497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5382939072,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5382939791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":845,"record_offset":6484,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":845,"record_offset":7156,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":7572,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":5284,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$26::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":2876,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":10022,"source_line":3797,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":845,"record_offset":9260,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] blake3::Hasher::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":845,"record_offset":2932,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":54467,"source_line":1010,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":845,"record_offset":3080,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":845,"record_offset":3208,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":9461,"source_line":1252,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":845,"record_offset":6448,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":845,"record_offset":7768,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":845,"record_offset":3176,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":845,"record_offset":9372,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::ends_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":845,"record_offset":3576,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":8791,"source_line":1391,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":11468,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":8157,"source_line":2185,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::create_new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":845,"record_offset":8308,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":54474,"source_line":523,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":845,"record_offset":3788,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":8207,"source_line":2766,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":845,"record_offset":5796,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":9318,"source_line":347,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] std::io::copy::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\copy.rs
//     evidence={"module":845,"record_offset":5680,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":20255,"source_line":61,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":845,"record_offset":4212,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":13003,"source_line":3025,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":845,"record_offset":3236,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":845,"record_offset":6384,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":5177,"source_line":2250,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":845,"record_offset":4072,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] zip::read::ZipFile::name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/read.rs
//     evidence={"module":845,"record_offset":3264,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":20252,"source_line":858,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] zip::read::ZipFile::size
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/read.rs
//     evidence={"module":845,"record_offset":5224,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":54473,"source_line":926,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] zip::read::zip_archive::ZipArchive::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/read.rs
//     evidence={"module":845,"record_offset":3024,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":54472,"source_line":483,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":845,"record_offset":5600,"parent_record_offset":5576,"ancestor_record_offset":2588,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":845,"record_offset":5624,"parent_record_offset":5600,"ancestor_record_offset":2588,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":845,"record_offset":4756,"parent_record_offset":4732,"ancestor_record_offset":2588,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":845,"record_offset":5576,"parent_record_offset":5400,"ancestor_record_offset":2588,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":845,"record_offset":4732,"parent_record_offset":4644,"ancestor_record_offset":2588,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$4::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\borrow.rs
//     evidence={"module":845,"record_offset":3292,"parent_record_offset":3264,"ancestor_record_offset":2588,"inlinee":10898,"source_line":335,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":845,"record_offset":4644,"parent_record_offset":4616,"ancestor_record_offset":2588,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":845,"record_offset":3460,"parent_record_offset":3432,"ancestor_record_offset":2588,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":845,"record_offset":5340,"parent_record_offset":5312,"ancestor_record_offset":2588,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":845,"record_offset":7460,"parent_record_offset":7432,"ancestor_record_offset":2588,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":845,"record_offset":7432,"parent_record_offset":7408,"ancestor_record_offset":2588,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":845,"record_offset":3516,"parent_record_offset":3488,"ancestor_record_offset":2588,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":845,"record_offset":3488,"parent_record_offset":3460,"ancestor_record_offset":2588,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":845,"record_offset":5400,"parent_record_offset":5368,"ancestor_record_offset":2588,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":845,"record_offset":5368,"parent_record_offset":5340,"ancestor_record_offset":2588,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":845,"record_offset":7408,"parent_record_offset":7384,"ancestor_record_offset":2588,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":845,"record_offset":5436,"parent_record_offset":5400,"ancestor_record_offset":2588,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":845,"record_offset":6644,"parent_record_offset":6612,"ancestor_record_offset":2588,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":845,"record_offset":6612,"parent_record_offset":6580,"ancestor_record_offset":2588,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":845,"record_offset":6548,"parent_record_offset":6516,"ancestor_record_offset":2588,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":845,"record_offset":6580,"parent_record_offset":6548,"ancestor_record_offset":2588,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":845,"record_offset":6516,"parent_record_offset":6484,"ancestor_record_offset":2588,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":845,"record_offset":3360,"parent_record_offset":3292,"ancestor_record_offset":2588,"inlinee":5602,"source_line":1081,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":845,"record_offset":7244,"parent_record_offset":7192,"ancestor_record_offset":2588,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":845,"record_offset":7192,"parent_record_offset":7156,"ancestor_record_offset":2588,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":845,"record_offset":10920,"parent_record_offset":10884,"ancestor_record_offset":2588,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":845,"record_offset":10884,"parent_record_offset":10860,"ancestor_record_offset":2588,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":12068,"parent_record_offset":12044,"ancestor_record_offset":2588,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":7616,"parent_record_offset":7572,"ancestor_record_offset":2588,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":3432,"parent_record_offset":3396,"ancestor_record_offset":2588,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":3396,"parent_record_offset":3360,"ancestor_record_offset":2588,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":12020,"parent_record_offset":11996,"ancestor_record_offset":2588,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":3048,"parent_record_offset":3024,"ancestor_record_offset":2588,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":2588,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":11528,"parent_record_offset":11468,"ancestor_record_offset":2588,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":5312,"parent_record_offset":5284,"ancestor_record_offset":2588,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":10060,"parent_record_offset":10036,"ancestor_record_offset":2588,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$30::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":6040,"parent_record_offset":6016,"ancestor_record_offset":2588,"inlinee":4954,"source_line":3825,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":6064,"parent_record_offset":6040,"ancestor_record_offset":2588,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":845,"record_offset":12044,"parent_record_offset":12020,"ancestor_record_offset":2588,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":845,"record_offset":9332,"parent_record_offset":9292,"ancestor_record_offset":2588,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":845,"record_offset":9292,"parent_record_offset":9260,"ancestor_record_offset":2588,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":845,"record_offset":15072,"parent_record_offset":15048,"ancestor_record_offset":2588,"inlinee":6205,"source_line":253,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":845,"record_offset":15048,"parent_record_offset":15024,"ancestor_record_offset":2588,"inlinee":6203,"source_line":49,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":845,"record_offset":15148,"parent_record_offset":15120,"ancestor_record_offset":2588,"inlinee":6210,"source_line":715,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":845,"record_offset":15096,"parent_record_offset":15072,"ancestor_record_offset":2588,"inlinee":6207,"source_line":72,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":845,"record_offset":15120,"parent_record_offset":15096,"ancestor_record_offset":2588,"inlinee":6208,"source_line":76,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] blake3::ChunkState::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":845,"record_offset":2984,"parent_record_offset":2956,"ancestor_record_offset":2588,"inlinee":54470,"source_line":477,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] blake3::Hasher::new_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":845,"record_offset":2956,"parent_record_offset":2932,"ancestor_record_offset":2588,"inlinee":54468,"source_line":1001,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":845,"record_offset":4676,"parent_record_offset":4644,"ancestor_record_offset":2588,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":845,"record_offset":5460,"parent_record_offset":5436,"ancestor_record_offset":2588,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":845,"record_offset":5484,"parent_record_offset":5460,"ancestor_record_offset":2588,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":845,"record_offset":3136,"parent_record_offset":3108,"ancestor_record_offset":2588,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":845,"record_offset":3688,"parent_record_offset":3660,"ancestor_record_offset":2588,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":845,"record_offset":3840,"parent_record_offset":3788,"ancestor_record_offset":2588,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":845,"record_offset":7272,"parent_record_offset":7192,"ancestor_record_offset":2588,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":845,"record_offset":7100,"parent_record_offset":7076,"ancestor_record_offset":2588,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":845,"record_offset":5532,"parent_record_offset":5508,"ancestor_record_offset":2588,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":845,"record_offset":3108,"parent_record_offset":3080,"ancestor_record_offset":2588,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":845,"record_offset":4700,"parent_record_offset":4676,"ancestor_record_offset":2588,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":845,"record_offset":5508,"parent_record_offset":5484,"ancestor_record_offset":2588,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":845,"record_offset":10732,"parent_record_offset":10704,"ancestor_record_offset":2588,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":845,"record_offset":7076,"parent_record_offset":6644,"ancestor_record_offset":2588,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=89
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":845,"record_offset":4304,"parent_record_offset":4212,"ancestor_record_offset":2588,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":845,"record_offset":4444,"parent_record_offset":4416,"ancestor_record_offset":2588,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":845,"record_offset":4492,"parent_record_offset":4468,"ancestor_record_offset":2588,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":845,"record_offset":4468,"parent_record_offset":4444,"ancestor_record_offset":2588,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":845,"record_offset":10952,"parent_record_offset":10920,"ancestor_record_offset":2588,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":845,"record_offset":7736,"parent_record_offset":7572,"ancestor_record_offset":2588,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":845,"record_offset":7300,"parent_record_offset":7192,"ancestor_record_offset":2588,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":845,"record_offset":4268,"parent_record_offset":4244,"ancestor_record_offset":2588,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":845,"record_offset":3712,"parent_record_offset":3688,"ancestor_record_offset":2588,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":845,"record_offset":3736,"parent_record_offset":3712,"ancestor_record_offset":2588,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":845,"record_offset":3660,"parent_record_offset":3632,"ancestor_record_offset":2588,"inlinee":5362,"source_line":2612,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":845,"record_offset":3632,"parent_record_offset":3604,"ancestor_record_offset":2588,"inlinee":8777,"source_line":1024,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$4::is_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":845,"record_offset":3604,"parent_record_offset":3576,"ancestor_record_offset":2588,"inlinee":20253,"source_line":603,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":845,"record_offset":10980,"parent_record_offset":10920,"ancestor_record_offset":2588,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":845,"record_offset":11012,"parent_record_offset":10884,"ancestor_record_offset":2588,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":11948,"parent_record_offset":11924,"ancestor_record_offset":2588,"inlinee":5420,"source_line":1849,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":12272,"parent_record_offset":12244,"ancestor_record_offset":2588,"inlinee":5435,"source_line":1859,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":11812,"parent_record_offset":11776,"ancestor_record_offset":2588,"inlinee":8160,"source_line":25,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":11836,"parent_record_offset":11812,"ancestor_record_offset":2588,"inlinee":8161,"source_line":49,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":11924,"parent_record_offset":11900,"ancestor_record_offset":2588,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":11900,"parent_record_offset":11868,"ancestor_record_offset":2588,"inlinee":5727,"source_line":409,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":12244,"parent_record_offset":12216,"ancestor_record_offset":2588,"inlinee":5730,"source_line":720,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":12216,"parent_record_offset":11868,"ancestor_record_offset":2588,"inlinee":5728,"source_line":774,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":12156,"parent_record_offset":11868,"ancestor_record_offset":2588,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":11776,"parent_record_offset":11468,"ancestor_record_offset":2588,"inlinee":8158,"source_line":2149,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":845,"record_offset":6412,"parent_record_offset":6384,"ancestor_record_offset":2588,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":845,"record_offset":3936,"parent_record_offset":3912,"ancestor_record_offset":2588,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":845,"record_offset":3984,"parent_record_offset":3960,"ancestor_record_offset":2588,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":845,"record_offset":3960,"parent_record_offset":3936,"ancestor_record_offset":2588,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":845,"record_offset":4036,"parent_record_offset":3788,"ancestor_record_offset":2588,"inlinee":8210,"source_line":3032,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":845,"record_offset":3812,"parent_record_offset":3788,"ancestor_record_offset":2588,"inlinee":8209,"source_line":3008,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":845,"record_offset":8340,"parent_record_offset":8308,"ancestor_record_offset":2588,"inlinee":11594,"source_line":1342,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":845,"record_offset":8600,"parent_record_offset":8308,"ancestor_record_offset":2588,"inlinee":11599,"source_line":1559,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":845,"record_offset":4244,"parent_record_offset":4212,"ancestor_record_offset":2588,"inlinee":12458,"source_line":2381,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::io::Read::read_to_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\mod.rs
//     evidence={"module":845,"record_offset":5756,"parent_record_offset":5732,"ancestor_record_offset":2588,"inlinee":23007,"source_line":881,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::io::copy::generic_copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\copy.rs
//     evidence={"module":845,"record_offset":5704,"parent_record_offset":5680,"ancestor_record_offset":2588,"inlinee":21294,"source_line":77,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::io::copy::impl$6::copy_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\copy.rs
//     evidence={"module":845,"record_offset":5732,"parent_record_offset":5704,"ancestor_record_offset":2588,"inlinee":22908,"source_line":263,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":845,"record_offset":4416,"parent_record_offset":4388,"ancestor_record_offset":2588,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":845,"record_offset":4388,"parent_record_offset":4360,"ancestor_record_offset":2588,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":845,"record_offset":11996,"parent_record_offset":11972,"ancestor_record_offset":2588,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":845,"record_offset":11972,"parent_record_offset":11948,"ancestor_record_offset":2588,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":845,"record_offset":13956,"parent_record_offset":13932,"ancestor_record_offset":2588,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":845,"record_offset":3912,"parent_record_offset":3888,"ancestor_record_offset":2588,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":845,"record_offset":3888,"parent_record_offset":3864,"ancestor_record_offset":2588,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":845,"record_offset":3864,"parent_record_offset":3840,"ancestor_record_offset":2588,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::create_new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":845,"record_offset":8400,"parent_record_offset":8340,"ancestor_record_offset":2588,"inlinee":32384,"source_line":221,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":845,"record_offset":8372,"parent_record_offset":8340,"ancestor_record_offset":2588,"inlinee":11605,"source_line":206,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::checked_zip::_::impl$0::serialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/checked_zip.rs
//     evidence={"module":845,"record_offset":11868,"parent_record_offset":11776,"ancestor_record_offset":2588,"inlinee":54475,"source_line":9,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::verify_folder_checksums
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":847,"record_offset":2272,"rva":14236064,"va":5382945184}
// 编译实例 RVAs: 0xd939a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$blake3..Hash$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t136773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e845160200","size":5,"soft_fail":false,"target_va":5383082784,"va":5382946006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$memmap2..os..MmapInner$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t9406044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e85c868f00","size":5,"soft_fail":false,"target_va":5392353392,"va":5382947343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d1240
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8140219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e845ca83ff","size":5,"soft_fail":false,"target_va":5374808640,"va":5382948854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8047342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8123585ff","size":5,"soft_fail":false,"target_va":5374900736,"va":5382948073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e88a0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8044278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e80a4185ff","size":5,"soft_fail":false,"target_va":5374904480,"va":5382948753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=36
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7863809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8ff0188ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5382945308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7863913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8970188ff","size":5,"soft_fail":false,"target_va":5375081568,"va":5382945476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-117543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8d934feff","size":5,"soft_fail":false,"target_va":5382827728,"va":5382945266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d8f4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8ddabffff","size":5,"soft_fail":false,"target_va":5382927584,"va":5382949118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419b0180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12698151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e827c2c100","size":5,"soft_fail":false,"target_va":5395644800,"va":5382946644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28901088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8e0feb801","size":5,"soft_fail":false,"target_va":5411847152,"va":5382946059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-107487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8215cfeff","size":5,"soft_fail":false,"target_va":5382840208,"va":5382947690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t28381137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8d10fb101","size":5,"soft_fail":false,"target_va":5411327296,"va":5382946154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h6ddf3353abc1c5bf (.llvm.12582498285691889218)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t28599923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e87366b401","size":5,"soft_fail":false,"target_va":5411545360,"va":5382945432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::he1efa0065b165f30 (.llvm.12582498285691889218)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t28600747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8ab69b401","size":5,"soft_fail":false,"target_va":5411546352,"va":5382945600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t28596836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8645ab401","size":5,"soft_fail":false,"target_va":5411544576,"va":5382947735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t28598836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e83462b401","size":5,"soft_fail":false,"target_va":5411546880,"va":5382948039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hasher::finalize
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t145021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e87d360200","size":5,"soft_fail":false,"target_va":5383090912,"va":5382945886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hasher::update
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t141764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8c4290200","size":5,"soft_fail":false,"target_va":5383087600,"va":5382945831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t139303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e827200200","size":5,"soft_fail":false,"target_va":5383086224,"va":5382946916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<memmap2::Mmap,std::io::error::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8b3edffff","size":5,"soft_fail":false,"target_va":5382944800,"va":5382949480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28594486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e83651b401","size":5,"soft_fail":false,"target_va":5411542880,"va":5382948389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] memmap2::os::MmapInner::map
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"memmap2","version":"0.9.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9407217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8f18a8f00","size":5,"soft_fail":false,"target_va":5392353952,"va":5382946730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] memmap2::os::file_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"memmap2","version":"0.9.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-2101363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e88defdfff","size":5,"soft_fail":false,"target_va":5380845328,"va":5382946686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-250702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8b22cfcff","size":5,"soft_fail":false,"target_va":5382694656,"va":5382945353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-237962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8765efcff","size":5,"soft_fail":false,"target_va":5382708672,"va":5382946629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-240473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8a754fcff","size":5,"soft_fail":false,"target_va":5382704816,"va":5382945284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-201183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e821eefcff","size":5,"soft_fail":false,"target_va":5382744080,"va":5382945258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*37741582(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5382945184,"raw_hex":"ff150ee43f02","size":6,"soft_fail":false,"va":5382946780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":847,"record_offset":9700,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":847,"record_offset":3948,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":847,"record_offset":3648,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":847,"record_offset":6240,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":847,"record_offset":3308,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] blake3::Hasher::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":847,"record_offset":3556,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":54467,"source_line":1010,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":847,"record_offset":4364,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":847,"record_offset":4756,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":847,"record_offset":6772,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":847,"record_offset":4528,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":847,"record_offset":9444,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":8221,"source_line":709,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":847,"record_offset":6968,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":847,"record_offset":11464,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":847,"record_offset":3796,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] memmap2::Mmap::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":847,"record_offset":6528,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":13581,"source_line":579,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":847,"record_offset":6412,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":11604,"source_line":382,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":847,"record_offset":2504,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":12456,"source_line":267,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":847,"record_offset":2476,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":847,"record_offset":2928,"parent_record_offset":2904,"ancestor_record_offset":2272,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":847,"record_offset":2904,"parent_record_offset":2852,"ancestor_record_offset":2272,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":847,"record_offset":8300,"parent_record_offset":8272,"ancestor_record_offset":2272,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":847,"record_offset":9748,"parent_record_offset":9724,"ancestor_record_offset":2272,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":847,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":2272,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":847,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":2272,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":847,"record_offset":2828,"parent_record_offset":2808,"ancestor_record_offset":2272,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":847,"record_offset":4036,"parent_record_offset":3984,"ancestor_record_offset":2272,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":847,"record_offset":3984,"parent_record_offset":3948,"ancestor_record_offset":2272,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$78::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":847,"record_offset":4392,"parent_record_offset":4364,"ancestor_record_offset":2272,"inlinee":5606,"source_line":359,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":847,"record_offset":3704,"parent_record_offset":3672,"ancestor_record_offset":2272,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":847,"record_offset":4808,"parent_record_offset":4784,"ancestor_record_offset":2272,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":847,"record_offset":3672,"parent_record_offset":3648,"ancestor_record_offset":2272,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":847,"record_offset":10556,"parent_record_offset":9980,"ancestor_record_offset":2272,"inlinee":4929,"source_line":112,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":847,"record_offset":9980,"parent_record_offset":9956,"ancestor_record_offset":2272,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":847,"record_offset":10776,"parent_record_offset":10752,"ancestor_record_offset":2272,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":847,"record_offset":10628,"parent_record_offset":10604,"ancestor_record_offset":2272,"inlinee":4934,"source_line":229,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":847,"record_offset":4420,"parent_record_offset":4392,"ancestor_record_offset":2272,"inlinee":5608,"source_line":15,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":847,"record_offset":3396,"parent_record_offset":3348,"ancestor_record_offset":2272,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":847,"record_offset":3348,"parent_record_offset":3308,"ancestor_record_offset":2272,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":847,"record_offset":13532,"parent_record_offset":13508,"ancestor_record_offset":2272,"inlinee":6205,"source_line":253,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":847,"record_offset":13508,"parent_record_offset":13484,"ancestor_record_offset":2272,"inlinee":6203,"source_line":49,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":847,"record_offset":13608,"parent_record_offset":13580,"ancestor_record_offset":2272,"inlinee":6210,"source_line":715,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":847,"record_offset":13556,"parent_record_offset":13532,"ancestor_record_offset":2272,"inlinee":6207,"source_line":72,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":847,"record_offset":13580,"parent_record_offset":13556,"ancestor_record_offset":2272,"inlinee":6208,"source_line":76,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] blake3::ChunkState::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":847,"record_offset":3608,"parent_record_offset":3580,"ancestor_record_offset":2272,"inlinee":54470,"source_line":477,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] blake3::Hasher::new_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":847,"record_offset":3580,"parent_record_offset":3556,"ancestor_record_offset":2272,"inlinee":54468,"source_line":1001,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":847,"record_offset":8332,"parent_record_offset":8300,"ancestor_record_offset":2272,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":847,"record_offset":4064,"parent_record_offset":3984,"ancestor_record_offset":2272,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":847,"record_offset":10604,"parent_record_offset":10580,"ancestor_record_offset":2272,"inlinee":4933,"source_line":116,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":847,"record_offset":10580,"parent_record_offset":10556,"ancestor_record_offset":2272,"inlinee":4931,"source_line":155,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":847,"record_offset":8356,"parent_record_offset":8332,"ancestor_record_offset":2272,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":847,"record_offset":5480,"parent_record_offset":5452,"ancestor_record_offset":2272,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":847,"record_offset":9724,"parent_record_offset":9700,"ancestor_record_offset":2272,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":847,"record_offset":6612,"parent_record_offset":6588,"ancestor_record_offset":2272,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":847,"record_offset":10700,"parent_record_offset":10676,"ancestor_record_offset":2272,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=103
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":847,"record_offset":2708,"parent_record_offset":2504,"ancestor_record_offset":2272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":847,"record_offset":3756,"parent_record_offset":3732,"ancestor_record_offset":2272,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":847,"record_offset":10676,"parent_record_offset":10652,"ancestor_record_offset":2272,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":847,"record_offset":8100,"parent_record_offset":8072,"ancestor_record_offset":2272,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":847,"record_offset":8148,"parent_record_offset":8124,"ancestor_record_offset":2272,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":847,"record_offset":8124,"parent_record_offset":8100,"ancestor_record_offset":2272,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":847,"record_offset":3908,"parent_record_offset":3884,"ancestor_record_offset":2272,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":847,"record_offset":6344,"parent_record_offset":6240,"ancestor_record_offset":2272,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":847,"record_offset":10652,"parent_record_offset":10628,"ancestor_record_offset":2272,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":847,"record_offset":3832,"parent_record_offset":3796,"ancestor_record_offset":2272,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":847,"record_offset":6372,"parent_record_offset":6344,"ancestor_record_offset":2272,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":847,"record_offset":4092,"parent_record_offset":3984,"ancestor_record_offset":2272,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":847,"record_offset":6728,"parent_record_offset":6556,"ancestor_record_offset":2272,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":847,"record_offset":6660,"parent_record_offset":6636,"ancestor_record_offset":2272,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":847,"record_offset":6700,"parent_record_offset":6556,"ancestor_record_offset":2272,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":847,"record_offset":4448,"parent_record_offset":4420,"ancestor_record_offset":2272,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":847,"record_offset":4476,"parent_record_offset":4448,"ancestor_record_offset":2272,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":847,"record_offset":3732,"parent_record_offset":3672,"ancestor_record_offset":2272,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":847,"record_offset":3884,"parent_record_offset":3860,"ancestor_record_offset":2272,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":847,"record_offset":3860,"parent_record_offset":3796,"ancestor_record_offset":2272,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::get_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":847,"record_offset":6588,"parent_record_offset":6556,"ancestor_record_offset":2272,"inlinee":13584,"source_line":228,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":847,"record_offset":6556,"parent_record_offset":6528,"ancestor_record_offset":2272,"inlinee":13583,"source_line":338,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::impl$2::get_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":847,"record_offset":6636,"parent_record_offset":6612,"ancestor_record_offset":2272,"inlinee":13586,"source_line":229,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":847,"record_offset":2612,"parent_record_offset":2588,"ancestor_record_offset":2272,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":847,"record_offset":2660,"parent_record_offset":2636,"ancestor_record_offset":2272,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":847,"record_offset":2636,"parent_record_offset":2612,"ancestor_record_offset":2272,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":847,"record_offset":6436,"parent_record_offset":6412,"ancestor_record_offset":2272,"inlinee":11594,"source_line":1342,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":847,"record_offset":6496,"parent_record_offset":6412,"ancestor_record_offset":2272,"inlinee":11599,"source_line":1559,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":847,"record_offset":8072,"parent_record_offset":8040,"ancestor_record_offset":2272,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":847,"record_offset":8040,"parent_record_offset":8012,"ancestor_record_offset":2272,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":847,"record_offset":6892,"parent_record_offset":6868,"ancestor_record_offset":2272,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":847,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":2272,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":847,"record_offset":2564,"parent_record_offset":2540,"ancestor_record_offset":2272,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":847,"record_offset":2540,"parent_record_offset":2504,"ancestor_record_offset":2272,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":847,"record_offset":6464,"parent_record_offset":6436,"ancestor_record_offset":2272,"inlinee":11605,"source_line":206,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: zip::read::zip_archive::ZipArchive::new<std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":863,"record_offset":260,"rva":14381984,"va":5383091104}
// 编译实例 RVAs: 0xdb73a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-244512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383091104,"raw_hex":"e8e044fcff","size":5,"soft_fail":false,"target_va":5382847888,"va":5383092395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140052a30
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14045241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383091104,"raw_hex":"e8c7af29ff","size":5,"soft_fail":false,"target_va":5369047600,"va":5383092836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e8250
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8190414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383091104,"raw_hex":"e8320683ff","size":5,"soft_fail":false,"target_va":5374902864,"va":5383093273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fa2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8116124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383091104,"raw_hex":"e8642884ff","size":5,"soft_fail":false,"target_va":5374976736,"va":5383092855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8010402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383091104,"raw_hex":"e85ec585ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5383091885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8010206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383091104,"raw_hex":"e822c685ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5383091705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28754809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383091104,"raw_hex":"e879c3b601","size":5,"soft_fail":false,"target_va":5411847168,"va":5383092354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28754929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383091104,"raw_hex":"e8f1c3b601","size":5,"soft_fail":false,"target_va":5411847184,"va":5383092250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28447843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383091104,"raw_hex":"e86314b201","size":5,"soft_fail":false,"target_va":5411540992,"va":5383093144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-7967589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383091104,"raw_hex":"e89b6c86ff","size":5,"soft_fail":false,"target_va":5375124896,"va":5383092480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28447831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383091104,"raw_hex":"e85714b201","size":5,"soft_fail":false,"target_va":5411540963,"va":5383093127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$zip..read..zip_archive..Shared$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-161279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383091104,"raw_hex":"e8018afdff","size":5,"soft_fail":false,"target_va":5382931920,"va":5383093194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::map::HashMap$LT$K$C$V$C$S$C$A$GT$::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12685971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383091104,"raw_hex":"e89392c100","size":5,"soft_fail":false,"target_va":5395778400,"va":5383092424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTable$LT$T$C$A$GT$::with_capacity_in
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12547709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383091104,"raw_hex":"e87d76bf00","size":5,"soft_fail":false,"target_va":5395639824,"va":5383092110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-434160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383091104,"raw_hex":"e81060f9ff","size":5,"soft_fail":false,"target_va":5382657856,"va":5383092011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-338263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383091104,"raw_hex":"e8a9d6faff","size":5,"soft_fail":false,"target_va":5382753792,"va":5383092050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zip::read::central_header_to_zip_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12682625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383091104,"raw_hex":"e88185c100","size":5,"soft_fail":false,"target_va":5395774928,"va":5383092298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zip::spec::CentralDirectoryEnd::find_and_parse
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12680614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383091104,"raw_hex":"e8a67dc100","size":5,"soft_fail":false,"target_va":5395771792,"va":5383091173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zip::spec::Zip64CentralDirectoryEnd::find_and_parse
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12679781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383091104,"raw_hex":"e8657ac100","size":5,"soft_fail":false,"target_va":5395772752,"va":5383092966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":863,"record_offset":3996,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":863,"record_offset":3960,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":863,"record_offset":3736,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":863,"record_offset":2564,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":863,"record_offset":3844,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":863,"record_offset":2248,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":863,"record_offset":484,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":863,"record_offset":516,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":863,"record_offset":2968,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":13588,"source_line":268,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_direct] zip::read::unsupported_zip_error
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/read.rs
//     evidence={"module":863,"record_offset":2536,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":55217,"source_line":641,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_direct] zip::read::zip_archive::ZipArchive::get_directory_counts
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/read.rs
//     evidence={"module":863,"record_offset":576,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":55214,"source_line":297,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_direct] zip::spec::CentralDirectoryEnd::record_too_small
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/spec.rs
//     evidence={"module":863,"record_offset":544,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":55213,"source_line":29,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":863,"record_offset":2888,"parent_record_offset":2864,"ancestor_record_offset":260,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":863,"record_offset":2912,"parent_record_offset":2888,"ancestor_record_offset":260,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":863,"record_offset":2484,"parent_record_offset":2460,"ancestor_record_offset":260,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":863,"record_offset":4060,"parent_record_offset":4028,"ancestor_record_offset":260,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":863,"record_offset":2864,"parent_record_offset":2688,"ancestor_record_offset":260,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":863,"record_offset":2460,"parent_record_offset":2380,"ancestor_record_offset":260,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":863,"record_offset":4028,"parent_record_offset":3996,"ancestor_record_offset":260,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":863,"record_offset":2628,"parent_record_offset":2596,"ancestor_record_offset":260,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":863,"record_offset":2420,"parent_record_offset":2380,"ancestor_record_offset":260,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":863,"record_offset":2380,"parent_record_offset":2344,"ancestor_record_offset":260,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":863,"record_offset":2688,"parent_record_offset":2656,"ancestor_record_offset":260,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":863,"record_offset":2656,"parent_record_offset":2628,"ancestor_record_offset":260,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":863,"record_offset":2344,"parent_record_offset":2312,"ancestor_record_offset":260,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":863,"record_offset":2724,"parent_record_offset":2688,"ancestor_record_offset":260,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":863,"record_offset":2596,"parent_record_offset":2564,"ancestor_record_offset":260,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] byteorder::io::ReadBytesExt::read_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":863,"record_offset":904,"parent_record_offset":856,"ancestor_record_offset":260,"inlinee":21404,"source_line":217,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] byteorder::io::ReadBytesExt::read_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":863,"record_offset":1468,"parent_record_offset":856,"ancestor_record_offset":260,"inlinee":40637,"source_line":321,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":863,"record_offset":2748,"parent_record_offset":2724,"ancestor_record_offset":260,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":863,"record_offset":2772,"parent_record_offset":2748,"ancestor_record_offset":260,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":863,"record_offset":3504,"parent_record_offset":3472,"ancestor_record_offset":260,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":863,"record_offset":3564,"parent_record_offset":3532,"ancestor_record_offset":260,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":863,"record_offset":1020,"parent_record_offset":964,"ancestor_record_offset":260,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":863,"record_offset":3892,"parent_record_offset":3868,"ancestor_record_offset":260,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":863,"record_offset":992,"parent_record_offset":964,"ancestor_record_offset":260,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":863,"record_offset":1192,"parent_record_offset":1168,"ancestor_record_offset":260,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":863,"record_offset":820,"parent_record_offset":732,"ancestor_record_offset":260,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":863,"record_offset":3868,"parent_record_offset":3844,"ancestor_record_offset":260,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":863,"record_offset":3360,"parent_record_offset":3336,"ancestor_record_offset":260,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":863,"record_offset":2796,"parent_record_offset":2772,"ancestor_record_offset":260,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":863,"record_offset":2820,"parent_record_offset":2796,"ancestor_record_offset":260,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":863,"record_offset":2160,"parent_record_offset":576,"ancestor_record_offset":260,"inlinee":7229,"source_line":486,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":863,"record_offset":732,"parent_record_offset":704,"ancestor_record_offset":260,"inlinee":14148,"source_line":593,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":863,"record_offset":1912,"parent_record_offset":576,"ancestor_record_offset":260,"inlinee":18849,"source_line":653,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":863,"record_offset":788,"parent_record_offset":760,"ancestor_record_offset":260,"inlinee":14150,"source_line":2350,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":863,"record_offset":760,"parent_record_offset":732,"ancestor_record_offset":260,"inlinee":14149,"source_line":2431,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":863,"record_offset":3532,"parent_record_offset":3472,"ancestor_record_offset":260,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":863,"record_offset":3108,"parent_record_offset":3072,"ancestor_record_offset":260,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":863,"record_offset":1940,"parent_record_offset":576,"ancestor_record_offset":260,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":863,"record_offset":2096,"parent_record_offset":576,"ancestor_record_offset":260,"inlinee":9461,"source_line":1252,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":863,"record_offset":3224,"parent_record_offset":3196,"ancestor_record_offset":260,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":863,"record_offset":2280,"parent_record_offset":2248,"ancestor_record_offset":260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":863,"record_offset":3784,"parent_record_offset":3736,"ancestor_record_offset":260,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":863,"record_offset":3312,"parent_record_offset":3196,"ancestor_record_offset":260,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":863,"record_offset":3388,"parent_record_offset":3360,"ancestor_record_offset":260,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":863,"record_offset":4168,"parent_record_offset":4140,"ancestor_record_offset":260,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":863,"record_offset":3336,"parent_record_offset":3312,"ancestor_record_offset":260,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":863,"record_offset":3416,"parent_record_offset":3360,"ancestor_record_offset":260,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":863,"record_offset":2124,"parent_record_offset":576,"ancestor_record_offset":260,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":863,"record_offset":1168,"parent_record_offset":1136,"ancestor_record_offset":260,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":863,"record_offset":1048,"parent_record_offset":964,"ancestor_record_offset":260,"inlinee":6334,"source_line":1916,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":863,"record_offset":1072,"parent_record_offset":1048,"ancestor_record_offset":260,"inlinee":6335,"source_line":2115,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":863,"record_offset":1096,"parent_record_offset":1072,"ancestor_record_offset":260,"inlinee":6336,"source_line":2001,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::with_capacity_and_hasher
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":863,"record_offset":3668,"parent_record_offset":2992,"ancestor_record_offset":260,"inlinee":11991,"source_line":499,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::with_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":863,"record_offset":3696,"parent_record_offset":3668,"ancestor_record_offset":260,"inlinee":8476,"source_line":592,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":863,"record_offset":4348,"parent_record_offset":4324,"ancestor_record_offset":260,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":863,"record_offset":3024,"parent_record_offset":2992,"ancestor_record_offset":260,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":863,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":260,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":863,"record_offset":3136,"parent_record_offset":3108,"ancestor_record_offset":260,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":863,"record_offset":3472,"parent_record_offset":3072,"ancestor_record_offset":260,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":863,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":260,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::split
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":863,"record_offset":964,"parent_record_offset":932,"ancestor_record_offset":260,"inlinee":21422,"source_line":233,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$4::seek
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":863,"record_offset":704,"parent_record_offset":576,"ancestor_record_offset":260,"inlinee":14156,"source_line":293,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$5::read_exact
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":863,"record_offset":932,"parent_record_offset":904,"ancestor_record_offset":260,"inlinee":21967,"source_line":360,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$8::read_exact
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":863,"record_offset":1136,"parent_record_offset":932,"ancestor_record_offset":260,"inlinee":20267,"source_line":347,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":863,"record_offset":3164,"parent_record_offset":3136,"ancestor_record_offset":260,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":863,"record_offset":3196,"parent_record_offset":3164,"ancestor_record_offset":260,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":863,"record_offset":3072,"parent_record_offset":3048,"ancestor_record_offset":260,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":863,"record_offset":3048,"parent_record_offset":3024,"ancestor_record_offset":260,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] zip::read::impl$4::get_directory_counts::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/read.rs
//     evidence={"module":863,"record_offset":1968,"parent_record_offset":1940,"ancestor_record_offset":260,"inlinee":55219,"source_line":337,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] zip::spec::CentralDirectoryEnd::record_too_small
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/spec.rs
//     evidence={"module":863,"record_offset":2028,"parent_record_offset":576,"ancestor_record_offset":260,"inlinee":55213,"source_line":29,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   [pdb_inline_transitive] zip::spec::Zip64CentralDirectoryEndLocator::parse
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/spec.rs
//     evidence={"module":863,"record_offset":856,"parent_record_offset":576,"ancestor_record_offset":260,"inlinee":55215,"source_line":118,"archive":"_pdb/B/modules/00863.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::ThemeFetcher::new<std::path::PathBuf>
// 记录形式: compiled_procedure; instances=1
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":228,"record_offset":24424,"rva":5548544,"va":5374257664}
// 编译实例 RVAs: 0x54aa00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14054b290
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374257664,"raw_hex":"e873040000","size":5,"soft_fail":false,"target_va":5374259856,"va":5374258712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d3f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t561456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374257664,"raw_hex":"e830910800","size":5,"soft_fail":false,"target_va":5374820240,"va":5374258779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d9ae0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t585760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374257664,"raw_hex":"e820f00800","size":5,"soft_fail":false,"target_va":5374843616,"va":5374257851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t641808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374257664,"raw_hex":"e810cb0900","size":5,"soft_fail":false,"target_va":5374900480,"va":5374258667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t823266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374257664,"raw_hex":"e8e28f0c00","size":5,"soft_fail":false,"target_va":5375081488,"va":5374258217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t823660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374257664,"raw_hex":"e86c910c00","size":5,"soft_fail":false,"target_va":5375081504,"va":5374257839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140718a00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1892101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374257664,"raw_hex":"e805df1c00","size":5,"soft_fail":false,"target_va":5376150016,"va":5374257910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8569970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374257664,"raw_hex":"e872c48200","size":5,"soft_fail":false,"target_va":5382827728,"va":5374257753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a4000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37066101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374257664,"raw_hex":"e875953502","size":5,"soft_fail":false,"target_va":5411323904,"va":5374257798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37589207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374257664,"raw_hex":"e8d7903d02","size":5,"soft_fail":false,"target_va":5411847168,"va":5374257956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37282492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374257664,"raw_hex":"e8bce23802","size":5,"soft_fail":false,"target_va":5411540992,"va":5374258495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1793370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374257664,"raw_hex":"e85a5d1b00","size":5,"soft_fail":false,"target_va":5376052112,"va":5374258737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37284397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374257664,"raw_hex":"e82dea3802","size":5,"soft_fail":false,"target_va":5411542880,"va":5374258478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] reqwest::blocking::client::ClientBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1889730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374257664,"raw_hex":"e8c2d51c00","size":5,"soft_fail":false,"target_va":5376147808,"va":5374258073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8451801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374257664,"raw_hex":"e8d9f68000","size":5,"soft_fail":false,"target_va":5382709584,"va":5374257778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::from_pointee
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":228,"record_offset":25396,"parent_record_offset":24424,"ancestor_record_offset":24424,"inlinee":8006,"source_line":691,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":228,"record_offset":24588,"parent_record_offset":24424,"ancestor_record_offset":24424,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":228,"record_offset":24952,"parent_record_offset":24424,"ancestor_record_offset":24424,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":228,"record_offset":25352,"parent_record_offset":24424,"ancestor_record_offset":24424,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":228,"record_offset":24924,"parent_record_offset":24424,"ancestor_record_offset":24424,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] reqwest::blocking::client::ClientBuilder::pool_max_idle_per_host
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":228,"record_offset":25296,"parent_record_offset":24424,"ancestor_record_offset":24424,"inlinee":15435,"source_line":405,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] reqwest::blocking::client::ClientBuilder::timeout
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":228,"record_offset":25264,"parent_record_offset":24424,"ancestor_record_offset":24424,"inlinee":14791,"source_line":357,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":228,"record_offset":24644,"parent_record_offset":24424,"ancestor_record_offset":24424,"inlinee":8207,"source_line":2766,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":25560,"parent_record_offset":25536,"ancestor_record_offset":24424,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":25584,"parent_record_offset":25560,"ancestor_record_offset":24424,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":25200,"parent_record_offset":25176,"ancestor_record_offset":24424,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":25504,"parent_record_offset":25468,"ancestor_record_offset":24424,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":25536,"parent_record_offset":25504,"ancestor_record_offset":24424,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":25176,"parent_record_offset":25120,"ancestor_record_offset":24424,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":228,"record_offset":25468,"parent_record_offset":25432,"ancestor_record_offset":24424,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":228,"record_offset":25148,"parent_record_offset":25120,"ancestor_record_offset":24424,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":228,"record_offset":25120,"parent_record_offset":25096,"ancestor_record_offset":24424,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":228,"record_offset":25096,"parent_record_offset":25072,"ancestor_record_offset":24424,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":228,"record_offset":26088,"parent_record_offset":26052,"ancestor_record_offset":24424,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":228,"record_offset":25432,"parent_record_offset":25396,"ancestor_record_offset":24424,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":228,"record_offset":26052,"parent_record_offset":26028,"ancestor_record_offset":24424,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":228,"record_offset":24696,"parent_record_offset":24644,"ancestor_record_offset":24424,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$4::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":228,"record_offset":24612,"parent_record_offset":24588,"ancestor_record_offset":24424,"inlinee":10038,"source_line":770,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":228,"record_offset":24976,"parent_record_offset":24952,"ancestor_record_offset":24424,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":228,"record_offset":26120,"parent_record_offset":26088,"ancestor_record_offset":24424,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":228,"record_offset":25612,"parent_record_offset":25584,"ancestor_record_offset":24424,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":228,"record_offset":26148,"parent_record_offset":26088,"ancestor_record_offset":24424,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":228,"record_offset":26180,"parent_record_offset":26052,"ancestor_record_offset":24424,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::ClientBuilder::with_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":228,"record_offset":25320,"parent_record_offset":25296,"ancestor_record_offset":24424,"inlinee":15436,"source_line":839,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":228,"record_offset":24792,"parent_record_offset":24768,"ancestor_record_offset":24424,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":228,"record_offset":24840,"parent_record_offset":24816,"ancestor_record_offset":24424,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":228,"record_offset":24816,"parent_record_offset":24792,"ancestor_record_offset":24424,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":228,"record_offset":24892,"parent_record_offset":24644,"ancestor_record_offset":24424,"inlinee":8210,"source_line":3032,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":228,"record_offset":24668,"parent_record_offset":24644,"ancestor_record_offset":24424,"inlinee":8209,"source_line":3008,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":228,"record_offset":24768,"parent_record_offset":24744,"ancestor_record_offset":24424,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":228,"record_offset":24744,"parent_record_offset":24720,"ancestor_record_offset":24424,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":228,"record_offset":24720,"parent_record_offset":24696,"ancestor_record_offset":24424,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00228.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::ThemeFetcher::fetch_events
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F49
// 首条位置: {"module":259,"inlinee":16201}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":259,"record_offset":20188,"parent_record_offset":16640,"ancestor_record_offset":16640,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::String::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":259,"record_offset":16972,"parent_record_offset":16640,"ancestor_record_offset":16640,"inlinee":10711,"source_line":1839,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":259,"record_offset":19388,"parent_record_offset":16640,"ancestor_record_offset":16640,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":259,"record_offset":22212,"parent_record_offset":16640,"ancestor_record_offset":16640,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::store
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":259,"record_offset":22472,"parent_record_offset":16640,"ancestor_record_offset":16640,"inlinee":16234,"source_line":461,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":259,"record_offset":17760,"parent_record_offset":16640,"ancestor_record_offset":16640,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":259,"record_offset":20592,"parent_record_offset":16640,"ancestor_record_offset":16640,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":259,"record_offset":20272,"parent_record_offset":16640,"ancestor_record_offset":16640,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":259,"record_offset":21508,"parent_record_offset":16640,"ancestor_record_offset":16640,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":259,"record_offset":20620,"parent_record_offset":16640,"ancestor_record_offset":16640,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":18476,"parent_record_offset":16640,"ancestor_record_offset":16640,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":17000,"parent_record_offset":16640,"ancestor_record_offset":16640,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":17036,"parent_record_offset":16640,"ancestor_record_offset":16640,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] http::header::map::HeaderMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/http-0.2.9/src/header/map.rs
//     evidence={"module":259,"record_offset":17280,"parent_record_offset":16640,"ancestor_record_offset":16640,"inlinee":16205,"source_line":676,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":259,"record_offset":17792,"parent_record_offset":16640,"ancestor_record_offset":16640,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":259,"record_offset":17676,"parent_record_offset":16640,"ancestor_record_offset":16640,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":17204,"parent_record_offset":17180,"ancestor_record_offset":16640,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":17228,"parent_record_offset":17204,"ancestor_record_offset":16640,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":20796,"parent_record_offset":20772,"ancestor_record_offset":16640,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":17148,"parent_record_offset":17112,"ancestor_record_offset":16640,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":17180,"parent_record_offset":17148,"ancestor_record_offset":16640,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":20772,"parent_record_offset":20716,"ancestor_record_offset":16640,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::convert::impl$17::from
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed\convert.rs
//     evidence={"module":259,"record_offset":17072,"parent_record_offset":17036,"ancestor_record_offset":16640,"inlinee":16203,"source_line":550,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":259,"record_offset":17112,"parent_record_offset":17072,"ancestor_record_offset":16640,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":259,"record_offset":20236,"parent_record_offset":20212,"ancestor_record_offset":16640,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":17536,"parent_record_offset":17512,"ancestor_record_offset":16640,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":19744,"parent_record_offset":19716,"ancestor_record_offset":16640,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":20744,"parent_record_offset":20716,"ancestor_record_offset":16640,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":20716,"parent_record_offset":20692,"ancestor_record_offset":16640,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":17584,"parent_record_offset":17560,"ancestor_record_offset":16640,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":17560,"parent_record_offset":17536,"ancestor_record_offset":16640,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":19804,"parent_record_offset":19772,"ancestor_record_offset":16640,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":19772,"parent_record_offset":19744,"ancestor_record_offset":16640,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":20692,"parent_record_offset":20668,"ancestor_record_offset":16640,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":19848,"parent_record_offset":19804,"ancestor_record_offset":16640,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":259,"record_offset":19676,"parent_record_offset":19640,"ancestor_record_offset":16640,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":259,"record_offset":19640,"parent_record_offset":19604,"ancestor_record_offset":16640,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":259,"record_offset":19568,"parent_record_offset":19532,"ancestor_record_offset":16640,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":259,"record_offset":19604,"parent_record_offset":19568,"ancestor_record_offset":16640,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":259,"record_offset":19532,"parent_record_offset":19496,"ancestor_record_offset":16640,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":259,"record_offset":19496,"parent_record_offset":19460,"ancestor_record_offset":16640,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$110::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":259,"record_offset":19424,"parent_record_offset":19388,"ancestor_record_offset":16640,"inlinee":10837,"source_line":2877,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":259,"record_offset":19460,"parent_record_offset":19424,"ancestor_record_offset":16640,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":259,"record_offset":22664,"parent_record_offset":22640,"ancestor_record_offset":16640,"inlinee":7989,"source_line":1446,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":259,"record_offset":22688,"parent_record_offset":22664,"ancestor_record_offset":16640,"inlinee":7990,"source_line":1684,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":259,"record_offset":22832,"parent_record_offset":22808,"ancestor_record_offset":16640,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":259,"record_offset":17512,"parent_record_offset":17484,"ancestor_record_offset":16640,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":259,"record_offset":17484,"parent_record_offset":17432,"ancestor_record_offset":16640,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":259,"record_offset":22108,"parent_record_offset":22084,"ancestor_record_offset":16640,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":259,"record_offset":21680,"parent_record_offset":21628,"ancestor_record_offset":16640,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":259,"record_offset":19716,"parent_record_offset":19676,"ancestor_record_offset":16640,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":259,"record_offset":17404,"parent_record_offset":17308,"ancestor_record_offset":16640,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":259,"record_offset":21548,"parent_record_offset":21508,"ancestor_record_offset":16640,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":259,"record_offset":23288,"parent_record_offset":23264,"ancestor_record_offset":16640,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":259,"record_offset":17432,"parent_record_offset":17404,"ancestor_record_offset":16640,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":259,"record_offset":22084,"parent_record_offset":21628,"ancestor_record_offset":16640,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":259,"record_offset":21588,"parent_record_offset":21548,"ancestor_record_offset":16640,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":259,"record_offset":21628,"parent_record_offset":21588,"ancestor_record_offset":16640,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ArcSwapAny::swap
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":259,"record_offset":22496,"parent_record_offset":22472,"ancestor_record_offset":16640,"inlinee":16235,"source_line":466,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay_all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":259,"record_offset":22608,"parent_record_offset":22584,"ancestor_record_offset":16640,"inlinee":7948,"source_line":81,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$0::from_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":259,"record_offset":22640,"parent_record_offset":22496,"ancestor_record_offset":16640,"inlinee":7988,"source_line":121,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::wait_for_readers
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":259,"record_offset":22584,"parent_record_offset":22496,"ancestor_record_offset":16640,"inlinee":8000,"source_line":196,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] bytes::bytes::impl$3::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bytes","version":"1.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytes-1.6.0/src/bytes.rs
//     evidence={"module":259,"record_offset":21476,"parent_record_offset":21452,"ancestor_record_offset":16640,"inlinee":16231,"source_line":546,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":259,"record_offset":19876,"parent_record_offset":19848,"ancestor_record_offset":16640,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":259,"record_offset":19904,"parent_record_offset":19876,"ancestor_record_offset":16640,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":259,"record_offset":20120,"parent_record_offset":20096,"ancestor_record_offset":16640,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":259,"record_offset":19952,"parent_record_offset":19928,"ancestor_record_offset":16640,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":259,"record_offset":22132,"parent_record_offset":22108,"ancestor_record_offset":16640,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":259,"record_offset":22156,"parent_record_offset":22132,"ancestor_record_offset":16640,"inlinee":6052,"source_line":796,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":259,"record_offset":22784,"parent_record_offset":22472,"ancestor_record_offset":16640,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":259,"record_offset":19928,"parent_record_offset":19904,"ancestor_record_offset":16640,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":259,"record_offset":21936,"parent_record_offset":21912,"ancestor_record_offset":16640,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":259,"record_offset":23764,"parent_record_offset":23732,"ancestor_record_offset":16640,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":259,"record_offset":20212,"parent_record_offset":20188,"ancestor_record_offset":16640,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":259,"record_offset":17848,"parent_record_offset":17816,"ancestor_record_offset":16640,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":259,"record_offset":17876,"parent_record_offset":17816,"ancestor_record_offset":16640,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":259,"record_offset":22712,"parent_record_offset":22688,"ancestor_record_offset":16640,"inlinee":7991,"source_line":1097,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":259,"record_offset":20096,"parent_record_offset":19676,"ancestor_record_offset":16640,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":259,"record_offset":22736,"parent_record_offset":22712,"ancestor_record_offset":16640,"inlinee":7992,"source_line":1039,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":259,"record_offset":20644,"parent_record_offset":20620,"ancestor_record_offset":16640,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":259,"record_offset":22392,"parent_record_offset":22364,"ancestor_record_offset":16640,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":17372,"parent_record_offset":17348,"ancestor_record_offset":16640,"inlinee":5961,"source_line":1439,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":259,"record_offset":17628,"parent_record_offset":17432,"ancestor_record_offset":16640,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":22528,"parent_record_offset":22496,"ancestor_record_offset":16640,"inlinee":11099,"source_line":1628,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":22868,"parent_record_offset":22832,"ancestor_record_offset":16640,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":17700,"parent_record_offset":17676,"ancestor_record_offset":16640,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":17724,"parent_record_offset":17700,"ancestor_record_offset":16640,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":22892,"parent_record_offset":22868,"ancestor_record_offset":16640,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":22552,"parent_record_offset":22528,"ancestor_record_offset":16640,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":22924,"parent_record_offset":22832,"ancestor_record_offset":16640,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] http::header::map::HeaderMap::get2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/http-0.2.9/src/header/map.rs
//     evidence={"module":259,"record_offset":17308,"parent_record_offset":17280,"ancestor_record_offset":16640,"inlinee":16206,"source_line":683,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] http::header::map::as_header_name::impl$4::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/http-0.2.9/src/header/map.rs
//     evidence={"module":259,"record_offset":17348,"parent_record_offset":17308,"ancestor_record_offset":16640,"inlinee":16208,"source_line":3422,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":259,"record_offset":17904,"parent_record_offset":17816,"ancestor_record_offset":16640,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":259,"record_offset":17816,"parent_record_offset":17792,"ancestor_record_offset":16640,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":259,"record_offset":17928,"parent_record_offset":17904,"ancestor_record_offset":16640,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::impl$0::fetch_events::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 124
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xD7C0
// 首条位置: {"module":866,"inlinee":55232}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] arc_swap::ArcSwapAny::from_pointee
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":866,"record_offset":1288,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":8006,"source_line":691,"archive":"_pdb/B/modules/00866.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":866,"record_offset":1232,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00866.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":866,"record_offset":1540,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00866.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":866,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":1172,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00866.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":866,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":1172,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00866.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":866,"record_offset":1716,"parent_record_offset":1692,"ancestor_record_offset":1172,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00866.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":866,"record_offset":1376,"parent_record_offset":1344,"ancestor_record_offset":1172,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00866.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":866,"record_offset":1408,"parent_record_offset":1376,"ancestor_record_offset":1172,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00866.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":866,"record_offset":1692,"parent_record_offset":1636,"ancestor_record_offset":1172,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00866.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":866,"record_offset":1344,"parent_record_offset":1316,"ancestor_record_offset":1172,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00866.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":866,"record_offset":1664,"parent_record_offset":1636,"ancestor_record_offset":1172,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00866.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":866,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":1172,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00866.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":866,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":1172,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00866.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":866,"record_offset":1316,"parent_record_offset":1288,"ancestor_record_offset":1172,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00866.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":866,"record_offset":1564,"parent_record_offset":1540,"ancestor_record_offset":1172,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00866.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":866,"record_offset":1484,"parent_record_offset":1456,"ancestor_record_offset":1172,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00866.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::ThemeFetcher::download_blob
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F54
// 首条位置: {"module":259,"inlinee":16212}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":259,"record_offset":29876,"parent_record_offset":28800,"ancestor_record_offset":28800,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":259,"record_offset":32156,"parent_record_offset":28800,"ancestor_record_offset":28800,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$90::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":259,"record_offset":32592,"parent_record_offset":28800,"ancestor_record_offset":28800,"inlinee":16229,"source_line":2537,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] anyhow::Error::msg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":259,"record_offset":31584,"parent_record_offset":28800,"ancestor_record_offset":28800,"inlinee":15839,"source_line":79,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":259,"record_offset":29460,"parent_record_offset":28800,"ancestor_record_offset":28800,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":259,"record_offset":33016,"parent_record_offset":28800,"ancestor_record_offset":28800,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":259,"record_offset":29960,"parent_record_offset":28800,"ancestor_record_offset":28800,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":259,"record_offset":32788,"parent_record_offset":28800,"ancestor_record_offset":28800,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":30704,"parent_record_offset":28800,"ancestor_record_offset":28800,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":30740,"parent_record_offset":28800,"ancestor_record_offset":28800,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":259,"record_offset":29492,"parent_record_offset":28800,"ancestor_record_offset":28800,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":259,"record_offset":29376,"parent_record_offset":28800,"ancestor_record_offset":28800,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] reqwest::blocking::client::Client::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":259,"record_offset":29988,"parent_record_offset":28800,"ancestor_record_offset":28800,"inlinee":16214,"source_line":892,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":29204,"parent_record_offset":28800,"ancestor_record_offset":28800,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":29064,"parent_record_offset":28800,"ancestor_record_offset":28800,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":31004,"parent_record_offset":30980,"ancestor_record_offset":28800,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":31028,"parent_record_offset":31004,"ancestor_record_offset":28800,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":30404,"parent_record_offset":30376,"ancestor_record_offset":28800,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":30948,"parent_record_offset":30912,"ancestor_record_offset":28800,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":30980,"parent_record_offset":30948,"ancestor_record_offset":28800,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":30376,"parent_record_offset":30316,"ancestor_record_offset":28800,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":259,"record_offset":30912,"parent_record_offset":30876,"ancestor_record_offset":28800,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":259,"record_offset":29924,"parent_record_offset":29900,"ancestor_record_offset":28800,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":30344,"parent_record_offset":30316,"ancestor_record_offset":28800,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":30316,"parent_record_offset":30288,"ancestor_record_offset":28800,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":30288,"parent_record_offset":30264,"ancestor_record_offset":28800,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":259,"record_offset":32244,"parent_record_offset":32192,"ancestor_record_offset":28800,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":259,"record_offset":32192,"parent_record_offset":32156,"ancestor_record_offset":28800,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":259,"record_offset":30560,"parent_record_offset":30528,"ancestor_record_offset":28800,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":259,"record_offset":30528,"parent_record_offset":30504,"ancestor_record_offset":28800,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":259,"record_offset":34376,"parent_record_offset":34348,"ancestor_record_offset":28800,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":259,"record_offset":30876,"parent_record_offset":30840,"ancestor_record_offset":28800,"inlinee":15840,"source_line":237,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_adhoc
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":259,"record_offset":31636,"parent_record_offset":31584,"ancestor_record_offset":28800,"inlinee":15801,"source_line":114,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_std
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":259,"record_offset":30840,"parent_record_offset":30784,"ancestor_record_offset":28800,"inlinee":15802,"source_line":88,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::error::impl$1::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":259,"record_offset":30784,"parent_record_offset":30740,"ancestor_record_offset":28800,"inlinee":15842,"source_line":564,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] bytes::bytes::impl$3::drop
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bytes","version":"1.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytes-1.6.0/src/bytes.rs
//     evidence={"module":259,"record_offset":33068,"parent_record_offset":33044,"ancestor_record_offset":28800,"inlinee":16231,"source_line":546,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":259,"record_offset":32620,"parent_record_offset":32592,"ancestor_record_offset":28800,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":259,"record_offset":32676,"parent_record_offset":32648,"ancestor_record_offset":28800,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":259,"record_offset":32272,"parent_record_offset":32192,"ancestor_record_offset":28800,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":259,"record_offset":29900,"parent_record_offset":29876,"ancestor_record_offset":28800,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":259,"record_offset":29548,"parent_record_offset":29516,"ancestor_record_offset":28800,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":259,"record_offset":29576,"parent_record_offset":29516,"ancestor_record_offset":28800,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":259,"record_offset":30192,"parent_record_offset":30156,"ancestor_record_offset":28800,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":259,"record_offset":30596,"parent_record_offset":30560,"ancestor_record_offset":28800,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":259,"record_offset":31056,"parent_record_offset":31028,"ancestor_record_offset":28800,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":32300,"parent_record_offset":32192,"ancestor_record_offset":28800,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":30060,"parent_record_offset":30020,"ancestor_record_offset":28800,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":259,"record_offset":32704,"parent_record_offset":32676,"ancestor_record_offset":28800,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":259,"record_offset":32732,"parent_record_offset":32704,"ancestor_record_offset":28800,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":259,"record_offset":32648,"parent_record_offset":32620,"ancestor_record_offset":28800,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":29400,"parent_record_offset":29376,"ancestor_record_offset":28800,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":30624,"parent_record_offset":30560,"ancestor_record_offset":28800,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":29424,"parent_record_offset":29400,"ancestor_record_offset":28800,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":34484,"parent_record_offset":34416,"ancestor_record_offset":28800,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":34520,"parent_record_offset":34376,"ancestor_record_offset":28800,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":259,"record_offset":29604,"parent_record_offset":29516,"ancestor_record_offset":28800,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":259,"record_offset":29516,"parent_record_offset":29492,"ancestor_record_offset":28800,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":259,"record_offset":29628,"parent_record_offset":29604,"ancestor_record_offset":28800,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::async_impl::request::Request::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/async_impl/request.rs
//     evidence={"module":259,"record_offset":30156,"parent_record_offset":30124,"ancestor_record_offset":28800,"inlinee":16221,"source_line":43,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::Client::request
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":259,"record_offset":30020,"parent_record_offset":29988,"ancestor_record_offset":28800,"inlinee":16215,"source_line":949,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$11::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":259,"record_offset":30480,"parent_record_offset":30020,"ancestor_record_offset":28800,"inlinee":16223,"source_line":49,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$12::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":259,"record_offset":30504,"parent_record_offset":30480,"ancestor_record_offset":28800,"inlinee":16225,"source_line":987,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$13::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":259,"record_offset":30660,"parent_record_offset":30504,"ancestor_record_offset":28800,"inlinee":16227,"source_line":1157,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$4::request::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":259,"record_offset":30096,"parent_record_offset":30060,"ancestor_record_offset":28800,"inlinee":16217,"source_line":950,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::request::Request::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/request.rs
//     evidence={"module":259,"record_offset":30124,"parent_record_offset":30096,"ancestor_record_offset":28800,"inlinee":16219,"source_line":37,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":259,"record_offset":29112,"parent_record_offset":29088,"ancestor_record_offset":28800,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":259,"record_offset":29160,"parent_record_offset":29136,"ancestor_record_offset":28800,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":259,"record_offset":29136,"parent_record_offset":29112,"ancestor_record_offset":28800,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":29088,"parent_record_offset":29064,"ancestor_record_offset":28800,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::ThemeFetcher::trim_cas
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 204
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F44
// 首条位置: {"module":259,"inlinee":16196}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":259,"record_offset":5168,"parent_record_offset":4420,"ancestor_record_offset":4420,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":259,"record_offset":4688,"parent_record_offset":4420,"ancestor_record_offset":4420,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":259,"record_offset":12100,"parent_record_offset":4420,"ancestor_record_offset":4420,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":259,"record_offset":4720,"parent_record_offset":4420,"ancestor_record_offset":4420,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":259,"record_offset":4604,"parent_record_offset":4420,"ancestor_record_offset":4420,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":259,"record_offset":4896,"parent_record_offset":4420,"ancestor_record_offset":4420,"inlinee":12459,"source_line":2936,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] theme_fetcher::ThemeFetcher::validate_cas_cache_entry
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/lib.rs
//     evidence={"module":259,"record_offset":5284,"parent_record_offset":4420,"ancestor_record_offset":4420,"inlinee":16197,"source_line":226,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":6124,"parent_record_offset":6100,"ancestor_record_offset":4420,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":6100,"parent_record_offset":6044,"ancestor_record_offset":4420,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":259,"record_offset":12484,"parent_record_offset":12456,"ancestor_record_offset":4420,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":6072,"parent_record_offset":6044,"ancestor_record_offset":4420,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":4420,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":4420,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":259,"record_offset":14576,"parent_record_offset":14540,"ancestor_record_offset":4420,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":259,"record_offset":14540,"parent_record_offset":14516,"ancestor_record_offset":4420,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":259,"record_offset":5244,"parent_record_offset":5200,"ancestor_record_offset":4420,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":259,"record_offset":5200,"parent_record_offset":5168,"ancestor_record_offset":4420,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":259,"record_offset":12516,"parent_record_offset":12484,"ancestor_record_offset":4420,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::gt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":259,"record_offset":6552,"parent_record_offset":5284,"ancestor_record_offset":4420,"inlinee":11236,"source_line":1407,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":259,"record_offset":7768,"parent_record_offset":5284,"ancestor_record_offset":4420,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":259,"record_offset":6628,"parent_record_offset":6600,"ancestor_record_offset":4420,"inlinee":5097,"source_line":1820,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":259,"record_offset":4928,"parent_record_offset":4896,"ancestor_record_offset":4420,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":259,"record_offset":12544,"parent_record_offset":12516,"ancestor_record_offset":4420,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":259,"record_offset":4776,"parent_record_offset":4744,"ancestor_record_offset":4420,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":259,"record_offset":4804,"parent_record_offset":4744,"ancestor_record_offset":4420,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=121
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":259,"record_offset":5876,"parent_record_offset":5672,"ancestor_record_offset":4420,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":259,"record_offset":12272,"parent_record_offset":12244,"ancestor_record_offset":4420,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":259,"record_offset":12320,"parent_record_offset":12296,"ancestor_record_offset":4420,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":259,"record_offset":12296,"parent_record_offset":12272,"ancestor_record_offset":4420,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":259,"record_offset":14608,"parent_record_offset":14576,"ancestor_record_offset":4420,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":5124,"parent_record_offset":4896,"ancestor_record_offset":4420,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":5432,"parent_record_offset":5284,"ancestor_record_offset":4420,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":5468,"parent_record_offset":5284,"ancestor_record_offset":4420,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":4628,"parent_record_offset":4604,"ancestor_record_offset":4420,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":4652,"parent_record_offset":4628,"ancestor_record_offset":4420,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":14636,"parent_record_offset":14576,"ancestor_record_offset":4420,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":14668,"parent_record_offset":14540,"ancestor_record_offset":4420,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":259,"record_offset":6600,"parent_record_offset":6552,"ancestor_record_offset":4420,"inlinee":5095,"source_line":79,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":259,"record_offset":4832,"parent_record_offset":4744,"ancestor_record_offset":4420,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":259,"record_offset":7800,"parent_record_offset":5284,"ancestor_record_offset":4420,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":259,"record_offset":4744,"parent_record_offset":4720,"ancestor_record_offset":4420,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":259,"record_offset":4856,"parent_record_offset":4832,"ancestor_record_offset":4420,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":259,"record_offset":7684,"parent_record_offset":5284,"ancestor_record_offset":4420,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":259,"record_offset":5024,"parent_record_offset":5000,"ancestor_record_offset":4420,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":259,"record_offset":5072,"parent_record_offset":5048,"ancestor_record_offset":4420,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":259,"record_offset":5048,"parent_record_offset":5024,"ancestor_record_offset":4420,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":259,"record_offset":5520,"parent_record_offset":5496,"ancestor_record_offset":4420,"inlinee":13008,"source_line":2036,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::Metadata::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":259,"record_offset":5496,"parent_record_offset":5284,"ancestor_record_offset":4420,"inlinee":16199,"source_line":1622,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::remove_dir_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":259,"record_offset":6664,"parent_record_offset":5284,"ancestor_record_offset":4420,"inlinee":16200,"source_line":2860,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":259,"record_offset":5672,"parent_record_offset":5284,"ancestor_record_offset":4420,"inlinee":8068,"source_line":2342,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":259,"record_offset":12244,"parent_record_offset":12208,"ancestor_record_offset":4420,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":259,"record_offset":12208,"parent_record_offset":12172,"ancestor_record_offset":4420,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":4420,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":4976,"parent_record_offset":4952,"ancestor_record_offset":4420,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":4952,"parent_record_offset":4928,"ancestor_record_offset":4420,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":259,"record_offset":5544,"parent_record_offset":5520,"ancestor_record_offset":4420,"inlinee":13010,"source_line":1126,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_symlink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":259,"record_offset":5568,"parent_record_offset":5544,"ancestor_record_offset":4420,"inlinee":13011,"source_line":1132,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::ThemeFetcher::validate_cas_cache_entry
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 226
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F45
// 首条位置: {"module":259,"inlinee":16197}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::gt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":259,"record_offset":6552,"parent_record_offset":5284,"ancestor_record_offset":5284,"inlinee":11236,"source_line":1407,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":259,"record_offset":7768,"parent_record_offset":5284,"ancestor_record_offset":5284,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":259,"record_offset":7976,"parent_record_offset":5284,"ancestor_record_offset":5284,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":5432,"parent_record_offset":5284,"ancestor_record_offset":5284,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":5468,"parent_record_offset":5284,"ancestor_record_offset":5284,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":259,"record_offset":7800,"parent_record_offset":5284,"ancestor_record_offset":5284,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":259,"record_offset":7684,"parent_record_offset":5284,"ancestor_record_offset":5284,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] std::fs::Metadata::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":259,"record_offset":5496,"parent_record_offset":5284,"ancestor_record_offset":5284,"inlinee":16199,"source_line":1622,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_dir_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":259,"record_offset":6664,"parent_record_offset":5284,"ancestor_record_offset":5284,"inlinee":16200,"source_line":2860,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":259,"record_offset":5672,"parent_record_offset":5284,"ancestor_record_offset":5284,"inlinee":8068,"source_line":2342,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":7544,"parent_record_offset":5284,"ancestor_record_offset":5284,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":6124,"parent_record_offset":6100,"ancestor_record_offset":5284,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":6100,"parent_record_offset":6044,"ancestor_record_offset":5284,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":6072,"parent_record_offset":6044,"ancestor_record_offset":5284,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":5284,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":5284,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":259,"record_offset":6628,"parent_record_offset":6600,"ancestor_record_offset":5284,"inlinee":5097,"source_line":1820,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":259,"record_offset":7856,"parent_record_offset":7824,"ancestor_record_offset":5284,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":259,"record_offset":7884,"parent_record_offset":7824,"ancestor_record_offset":5284,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=75
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":259,"record_offset":5876,"parent_record_offset":5672,"ancestor_record_offset":5284,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":7708,"parent_record_offset":7684,"ancestor_record_offset":5284,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":7732,"parent_record_offset":7708,"ancestor_record_offset":5284,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":259,"record_offset":6600,"parent_record_offset":6552,"ancestor_record_offset":5284,"inlinee":5095,"source_line":79,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":259,"record_offset":7912,"parent_record_offset":7824,"ancestor_record_offset":5284,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":259,"record_offset":7824,"parent_record_offset":7800,"ancestor_record_offset":5284,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":259,"record_offset":7936,"parent_record_offset":7912,"ancestor_record_offset":5284,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":259,"record_offset":5780,"parent_record_offset":5756,"ancestor_record_offset":5284,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":259,"record_offset":5828,"parent_record_offset":5804,"ancestor_record_offset":5284,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":259,"record_offset":5804,"parent_record_offset":5780,"ancestor_record_offset":5284,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":259,"record_offset":5520,"parent_record_offset":5496,"ancestor_record_offset":5284,"inlinee":13008,"source_line":2036,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":5756,"parent_record_offset":5732,"ancestor_record_offset":5284,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":5732,"parent_record_offset":5708,"ancestor_record_offset":5284,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":5708,"parent_record_offset":5672,"ancestor_record_offset":5284,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":259,"record_offset":5544,"parent_record_offset":5520,"ancestor_record_offset":5284,"inlinee":13010,"source_line":1126,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_symlink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":259,"record_offset":5568,"parent_record_offset":5544,"ancestor_record_offset":5284,"inlinee":13011,"source_line":1132,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::ThemeFetcher::run
// 记录形式: compiled_procedure; instances=1
// 源码行: 261
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":259,"record_offset":4216,"rva":6390544,"va":5375099664}
// 编译实例 RVAs: 0x618310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t7734888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e868067600","size":5,"soft_fail":false,"target_va":5382847888,"va":5375112995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$reqwest..into_url..IntoUrlSealed$GT$::into_url
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t870827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8ab490d00","size":5,"soft_fail":false,"target_va":5375979568,"va":5375108736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$arc_swap..debt..list..LocalNode$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t6738702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e80ed36600","size":5,"soft_fail":false,"target_va":5381846464,"va":5375107757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$arc_swap..strategy..hybrid..HybridStrategy$LT$Cfg$GT$$u20$as$u20$arc_swap..strategy..sealed..InnerStrategy$LT$T$GT$$GT$::load::_$u7b$$u7b$closure$u7d$$u7d$::h0210ff617624088d (.llvm.8099390888442591687)
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t7969498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8da9a7900","size":5,"soft_fail":false,"target_va":5383077168,"va":5375107665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$blake3..Hash$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7972046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8cea47900","size":5,"soft_fail":false,"target_va":5383082784,"va":5375110733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=21
//     evidence={"assembly":"\tcallq\t7544587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e80b1f7300","size":5,"soft_fail":false,"target_va":5382644528,"va":5375099936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7608338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e812187400","size":5,"soft_fail":false,"target_va":5382708864,"va":5375100521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..fs..FindNextFileHandle$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7665262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e86ef67400","size":5,"soft_fail":false,"target_va":5382768048,"va":5375102781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..time..Instant$u20$as$u20$core..ops..arith..Add$LT$core..time..Duration$GT$$GT$::add
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7642498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8829d7400","size":5,"soft_fail":false,"target_va":5382748704,"va":5375106201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14002e900
//     owner=unresolved_namespace; count=1
//     target_aliases=130 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6222191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8910ea1ff","size":5,"soft_fail":false,"target_va":5368899840,"va":5375122026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cdd40
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5570315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8f500abff","size":5,"soft_fail":false,"target_va":5369552192,"va":5375122502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cddc0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5568926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e86206abff","size":5,"soft_fail":false,"target_va":5369552320,"va":5375121241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014a860
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5055863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e889dab2ff","size":5,"soft_fail":false,"target_va":5370062944,"va":5375118802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403cf120
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2417326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8521ddbff","size":5,"soft_fail":false,"target_va":5372703008,"va":5375120329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e69f0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-225774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8128efcff","size":5,"soft_fail":false,"target_va":5374896624,"va":5375122393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-218359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e809abfcff","size":5,"soft_fail":false,"target_va":5374900736,"va":5375119090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e88a0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-213326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8b2befcff","size":5,"soft_fail":false,"target_va":5374904480,"va":5375117801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e817aaffff","size":5,"soft_fail":false,"target_va":5375081488,"va":5375103476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=45
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8ecb3ffff","size":5,"soft_fail":false,"target_va":5375081504,"va":5375100975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8e4a0ffff","size":5,"soft_fail":false,"target_va":5375082160,"va":5375106503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=21
//     target_aliases=69 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t416261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8055a0600","size":5,"soft_fail":false,"target_va":5375516032,"va":5375099766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=10
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7727751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e887ea7500","size":5,"soft_fail":false,"target_va":5382827728,"va":5375099972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d921c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7826833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8916d7700","size":5,"soft_fail":false,"target_va":5382939072,"va":5375112234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140db73a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7979020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e80cc07900","size":5,"soft_fail":false,"target_va":5383091104,"va":5375112079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140db8370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7979401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e889c17900","size":5,"soft_fail":false,"target_va":5383095152,"va":5375115746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36736361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8698d3002","size":5,"soft_fail":false,"target_va":5411847152,"va":5375110786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36746734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8eeb53002","size":5,"soft_fail":false,"target_va":5411847168,"va":5375100429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t36423938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e802c92b02","size":5,"soft_fail":false,"target_va":5411540992,"va":5375117049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::boxed::convert::_$LT$impl$u20$core..convert..From$LT$$RF$str$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$core..error..Error$GT$$GT$::from
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7731321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e879f87500","size":5,"soft_fail":false,"target_va":5382837952,"va":5375106626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::boxed::convert::_$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$core..error..Error$GT$$GT$::from
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7731628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8acf97500","size":5,"soft_fail":false,"target_va":5382837776,"va":5375106143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t7734385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e871047600","size":5,"soft_fail":false,"target_va":5382840208,"va":5375105818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t36423650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8e2c72b02","size":5,"soft_fail":false,"target_va":5411540963,"va":5375117308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=12
//     evidence={"assembly":"\tcallq\t7582140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8bcb17300","size":5,"soft_fail":false,"target_va":5382683584,"va":5375101439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36216420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8649e2802","size":5,"soft_fail":false,"target_va":5411327296,"va":5375110871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h6ddf3353abc1c5bf (.llvm.12582498285691889218)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36445212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e81c1c2c02","size":5,"soft_fail":false,"target_va":5411545360,"va":5375100143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36446245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e825202c02","size":5,"soft_fail":false,"target_va":5411546880,"va":5375100630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..fmt..Display$u20$for$u20$anyhow..Error$GT$::fmt
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7543296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8001a7300","size":5,"soft_fail":false,"target_va":5382654992,"va":5375111691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7554620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e83c467300","size":5,"soft_fail":false,"target_va":5382655024,"va":5375100399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7971867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e81ba47900","size":5,"soft_fail":false,"target_va":5383079280,"va":5375107408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::Node::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6738220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e82cd16600","size":5,"soft_fail":false,"target_va":5381845872,"va":5375107647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::THREAD_HEAD::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6738232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e838d16600","size":5,"soft_fail":false,"target_va":5381845808,"va":5375107571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7975602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8b2b27900","size":5,"soft_fail":false,"target_va":5383086224,"va":5375110617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] cache_control::CacheControl::from_value
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"cache_control","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7966755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e823907900","size":5,"soft_fail":false,"target_va":5383071664,"va":5375104904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::iter::traits::iterator::Iterator::eq_by
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7979393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e881c17900","size":5,"soft_fail":false,"target_va":5383094656,"va":5375115258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36424742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e826cc2b02","size":5,"soft_fail":false,"target_va":5411541776,"va":5375117029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36424926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8decc2b02","size":5,"soft_fail":false,"target_va":5411542296,"va":5375117365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..sync..ArcInner$LT$core..result..Result$LT$std..path..PathBuf$C$theme_defs..EventDataError$GT$$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7804843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8ab177700","size":5,"soft_fail":false,"target_va":5382928448,"va":5375123600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$theme_fetcher..MaybePendingActiveEvent$GT$$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7818795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e82b4e7700","size":5,"soft_fail":false,"target_va":5382936720,"va":5375117920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$anyhow..error..ErrorImpl$LT$anyhow..wrapper..MessageError$LT$alloc..string..String$GT$$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7804843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8ab177700","size":5,"soft_fail":false,"target_va":5382927120,"va":5375122272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$anyhow..error..ErrorImpl$LT$reqwest..error..Error$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7812219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e87b347700","size":5,"soft_fail":false,"target_va":5382934976,"va":5375122752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$reqwest..blocking..response..Response$GT$
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t7827847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e887717700","size":5,"soft_fail":false,"target_va":5382934016,"va":5375106164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$reqwest..error..Error$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t7811707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e87b327700","size":5,"soft_fail":false,"target_va":5382930016,"va":5375118304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$theme_fetcher..ThemeFetcher$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7813476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e864397700","size":5,"soft_fail":false,"target_va":5382930448,"va":5375116967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-22786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8fea6ffff","size":5,"soft_fail":false,"target_va":5375099104,"va":5375121885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::path::PathBuf,anyhow::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2564359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8f9ded8ff","size":5,"soft_fail":false,"target_va":5372559664,"va":5375124018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t36425778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e832d02b02","size":5,"soft_fail":false,"target_va":5411542880,"va":5375117097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] http::header::map::HeaderMap$LT$T$GT$::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7965041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e871897900","size":5,"soft_fail":false,"target_va":5383069888,"va":5375104842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] http::header::map::HeaderMap::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7971682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e862a37900","size":5,"soft_fail":false,"target_va":5383080480,"va":5375108793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] http::header::name::HdrName::from_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7960626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e832787900","size":5,"soft_fail":false,"target_va":5383064208,"va":5375103577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] http::header::value::HeaderValue::to_str
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7961770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8aa7c7900","size":5,"soft_fail":false,"target_va":5383065424,"va":5375103649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] httpdate::parse_http_date
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"httpdate","version":"1.0.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7963482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e85a837900","size":5,"soft_fail":false,"target_va":5383067184,"va":5375103697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] reqwest::blocking::client::Client::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7846460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e83cba7700","size":5,"soft_fail":false,"target_va":5382949776,"va":5375103311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] reqwest::blocking::request::RequestBuilder::header
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7847160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8f8bc7700","size":5,"soft_fail":false,"target_va":5382950560,"va":5375103395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] reqwest::blocking::request::RequestBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7924785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e831ec7800","size":5,"soft_fail":false,"target_va":5383034096,"va":5375109306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] reqwest::blocking::request::RequestBuilder::send
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7934520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e838127900","size":5,"soft_fail":false,"target_va":5383037952,"va":5375103427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] reqwest::blocking::response::Response::bytes
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t975064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8d8e00e00","size":5,"soft_fail":false,"target_va":5376081488,"va":5375106419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] reqwest::blocking::response::Response::content_length
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t975019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8abe00e00","size":5,"soft_fail":false,"target_va":5376081392,"va":5375106368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t7594611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e873e27300","size":5,"soft_fail":false,"target_va":5382694656,"va":5375100040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_name
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t7608417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e861187400","size":5,"soft_fail":false,"target_va":5382709552,"va":5375101130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::metadata
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7608814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8ee197400","size":5,"soft_fail":false,"target_va":5382709424,"va":5375100605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::path
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t7607989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8b5167400","size":5,"soft_fail":false,"target_va":5382709008,"va":5375101014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::Metadata::modified
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7607994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8ba167400","size":5,"soft_fail":false,"target_va":5382708832,"va":5375100833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7636178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8d2847400","size":5,"soft_fail":false,"target_va":5382744080,"va":5375107897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::components
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7630705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8716f7400","size":5,"soft_fail":false,"target_va":5382745088,"va":5375114378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::is_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7637673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8a98a7400","size":5,"soft_fail":false,"target_va":5382745616,"va":5375107938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::Sender$LT$T$GT$::send
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7979591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e847c27900","size":5,"soft_fail":false,"target_va":5383094208,"va":5375114612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::readdir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7670565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8250b7500","size":5,"soft_fail":false,"target_va":5382770560,"va":5375099990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::remove_dir_all
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7672403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e853127500","size":5,"soft_fail":false,"target_va":5382773968,"va":5375101560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::unlink
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7671077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8250d7500","size":5,"soft_fail":false,"target_va":5382772112,"va":5375101030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36439990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8b6072c02","size":5,"soft_fail":false,"target_va":5411547616,"va":5375107621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7576350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e81e9b7300","size":5,"soft_fail":false,"target_va":5382691200,"va":5375114845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7644598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8b6a57400","size":5,"soft_fail":false,"target_va":5382747616,"va":5375103013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::SystemTime::duration_since
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t7647654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8a6b17400","size":5,"soft_fail":false,"target_va":5382749136,"va":5375101477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::SystemTime::now
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t7649142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e876b77400","size":5,"soft_fail":false,"target_va":5382749088,"va":5375099941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] theme_fetcher::checked_zip::verify_folder_checksums
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7843517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375099664,"raw_hex":"e8bdae7700","size":5,"soft_fail":false,"target_va":5382945184,"va":5375101662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5375099664,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5375100710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":259,"record_offset":24904,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":259,"record_offset":36384,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":259,"record_offset":28132,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":259,"record_offset":26568,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":16211,"source_line":453,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::store
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":259,"record_offset":36616,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":16234,"source_line":461,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":259,"record_offset":24876,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":259,"record_offset":40492,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":259,"record_offset":16492,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":11109,"source_line":1425,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":259,"record_offset":16196,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":259,"record_offset":37452,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":5087,"source_line":1371,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":259,"record_offset":16404,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":28160,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":8084,"source_line":607,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":39692,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":35800,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":38268,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":13528,"source_line":1730,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":259,"record_offset":26624,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":259,"record_offset":26708,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":259,"record_offset":16228,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":259,"record_offset":16112,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpsc.rs
//     evidence={"module":259,"record_offset":42992,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":7835,"source_line":607,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] theme_fetcher::ThemeFetcher::download_blob
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/lib.rs
//     evidence={"module":259,"record_offset":28800,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":16212,"source_line":164,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] theme_fetcher::ThemeFetcher::fetch_events
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/lib.rs
//     evidence={"module":259,"record_offset":16640,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":16201,"source_line":59,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] theme_fetcher::ThemeFetcher::trim_cas
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/lib.rs
//     evidence={"module":259,"record_offset":4420,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":16196,"source_line":204,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":17204,"parent_record_offset":17180,"ancestor_record_offset":4216,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":17228,"parent_record_offset":17204,"ancestor_record_offset":4216,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=41
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":6124,"parent_record_offset":6100,"ancestor_record_offset":4216,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":17148,"parent_record_offset":17112,"ancestor_record_offset":4216,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":17180,"parent_record_offset":17148,"ancestor_record_offset":4216,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=41
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":6100,"parent_record_offset":6044,"ancestor_record_offset":4216,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::convert::impl$17::from
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed\convert.rs
//     evidence={"module":259,"record_offset":17072,"parent_record_offset":17036,"ancestor_record_offset":4216,"inlinee":16203,"source_line":550,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":259,"record_offset":17112,"parent_record_offset":17072,"ancestor_record_offset":4216,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$21::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":259,"record_offset":25048,"parent_record_offset":24940,"ancestor_record_offset":4216,"inlinee":16210,"source_line":1927,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":259,"record_offset":12484,"parent_record_offset":12456,"ancestor_record_offset":4216,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":259,"record_offset":20188,"parent_record_offset":16640,"ancestor_record_offset":4216,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":259,"record_offset":20236,"parent_record_offset":20212,"ancestor_record_offset":4216,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":17536,"parent_record_offset":17512,"ancestor_record_offset":4216,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":19744,"parent_record_offset":19716,"ancestor_record_offset":4216,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":6072,"parent_record_offset":6044,"ancestor_record_offset":4216,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":4216,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":17584,"parent_record_offset":17560,"ancestor_record_offset":4216,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":17560,"parent_record_offset":17536,"ancestor_record_offset":4216,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":19804,"parent_record_offset":19772,"ancestor_record_offset":4216,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":19772,"parent_record_offset":19744,"ancestor_record_offset":4216,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":4216,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":19848,"parent_record_offset":19804,"ancestor_record_offset":4216,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":259,"record_offset":19676,"parent_record_offset":19640,"ancestor_record_offset":4216,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":259,"record_offset":19640,"parent_record_offset":19604,"ancestor_record_offset":4216,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":259,"record_offset":19568,"parent_record_offset":19532,"ancestor_record_offset":4216,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":259,"record_offset":19604,"parent_record_offset":19568,"ancestor_record_offset":4216,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":259,"record_offset":19532,"parent_record_offset":19496,"ancestor_record_offset":4216,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":259,"record_offset":19496,"parent_record_offset":19460,"ancestor_record_offset":4216,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":259,"record_offset":16972,"parent_record_offset":16640,"ancestor_record_offset":4216,"inlinee":10711,"source_line":1839,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":259,"record_offset":24992,"parent_record_offset":24940,"ancestor_record_offset":4216,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$110::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":259,"record_offset":19424,"parent_record_offset":19388,"ancestor_record_offset":4216,"inlinee":10837,"source_line":2877,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":259,"record_offset":19388,"parent_record_offset":16640,"ancestor_record_offset":4216,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":259,"record_offset":24940,"parent_record_offset":24904,"ancestor_record_offset":4216,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":259,"record_offset":19460,"parent_record_offset":19424,"ancestor_record_offset":4216,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$78::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":259,"record_offset":40684,"parent_record_offset":40660,"ancestor_record_offset":4216,"inlinee":5606,"source_line":359,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$90::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":259,"record_offset":32592,"parent_record_offset":28800,"ancestor_record_offset":4216,"inlinee":16229,"source_line":2537,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":259,"record_offset":27320,"parent_record_offset":27296,"ancestor_record_offset":4216,"inlinee":7985,"source_line":1606,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":259,"record_offset":22664,"parent_record_offset":22640,"ancestor_record_offset":4216,"inlinee":7989,"source_line":1446,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":259,"record_offset":22688,"parent_record_offset":22664,"ancestor_record_offset":4216,"inlinee":7990,"source_line":1684,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":259,"record_offset":14576,"parent_record_offset":14540,"ancestor_record_offset":4216,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::into_raw
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":259,"record_offset":27296,"parent_record_offset":27272,"ancestor_record_offset":4216,"inlinee":12275,"source_line":1554,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":259,"record_offset":22212,"parent_record_offset":16640,"ancestor_record_offset":4216,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":259,"record_offset":30528,"parent_record_offset":30504,"ancestor_record_offset":4216,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":259,"record_offset":14540,"parent_record_offset":14516,"ancestor_record_offset":4216,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":259,"record_offset":17512,"parent_record_offset":17484,"ancestor_record_offset":4216,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":259,"record_offset":17484,"parent_record_offset":17432,"ancestor_record_offset":4216,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":259,"record_offset":22108,"parent_record_offset":22084,"ancestor_record_offset":4216,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":259,"record_offset":21680,"parent_record_offset":21628,"ancestor_record_offset":4216,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":259,"record_offset":19716,"parent_record_offset":19676,"ancestor_record_offset":4216,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":259,"record_offset":38460,"parent_record_offset":38432,"ancestor_record_offset":4216,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":259,"record_offset":17404,"parent_record_offset":17308,"ancestor_record_offset":4216,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":259,"record_offset":21548,"parent_record_offset":21508,"ancestor_record_offset":4216,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":259,"record_offset":23288,"parent_record_offset":23264,"ancestor_record_offset":4216,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":259,"record_offset":17432,"parent_record_offset":17404,"ancestor_record_offset":4216,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":259,"record_offset":40712,"parent_record_offset":40684,"ancestor_record_offset":4216,"inlinee":5608,"source_line":15,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":259,"record_offset":22084,"parent_record_offset":21628,"ancestor_record_offset":4216,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":259,"record_offset":21588,"parent_record_offset":21548,"ancestor_record_offset":4216,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":259,"record_offset":21628,"parent_record_offset":21588,"ancestor_record_offset":4216,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":259,"record_offset":30876,"parent_record_offset":30840,"ancestor_record_offset":4216,"inlinee":15840,"source_line":237,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_adhoc
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":259,"record_offset":31636,"parent_record_offset":31584,"ancestor_record_offset":4216,"inlinee":15801,"source_line":114,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":259,"record_offset":5244,"parent_record_offset":5200,"ancestor_record_offset":4216,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_std
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":259,"record_offset":30840,"parent_record_offset":30784,"ancestor_record_offset":4216,"inlinee":15802,"source_line":88,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::msg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":259,"record_offset":31584,"parent_record_offset":28800,"ancestor_record_offset":4216,"inlinee":15839,"source_line":79,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":259,"record_offset":5200,"parent_record_offset":5168,"ancestor_record_offset":4216,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":259,"record_offset":5168,"parent_record_offset":4420,"ancestor_record_offset":4216,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::error::impl$1::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":259,"record_offset":30784,"parent_record_offset":30740,"ancestor_record_offset":4216,"inlinee":15842,"source_line":564,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ArcSwapAny::store
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":259,"record_offset":22472,"parent_record_offset":16640,"ancestor_record_offset":4216,"inlinee":16234,"source_line":461,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ArcSwapAny::swap
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":259,"record_offset":22496,"parent_record_offset":22472,"ancestor_record_offset":4216,"inlinee":16235,"source_line":466,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":259,"record_offset":27356,"parent_record_offset":26856,"ancestor_record_offset":4216,"inlinee":7981,"source_line":65,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay_all
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":259,"record_offset":22608,"parent_record_offset":22584,"ancestor_record_offset":4216,"inlinee":7948,"source_line":81,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::list::LocalNode::with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/list.rs
//     evidence={"module":259,"record_offset":27524,"parent_record_offset":27488,"ancestor_record_offset":4216,"inlinee":15362,"source_line":223,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::list::THREAD_HEAD::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":259,"record_offset":27620,"parent_record_offset":27592,"ancestor_record_offset":4216,"inlinee":15353,"source_line":90,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::list::impl$3::with::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/list.rs
//     evidence={"module":259,"record_offset":27724,"parent_record_offset":27564,"ancestor_record_offset":4216,"inlinee":15358,"source_line":226,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::list::impl$3::with::closure$1
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/list.rs
//     evidence={"module":259,"record_offset":27968,"parent_record_offset":27928,"ancestor_record_offset":4216,"inlinee":15355,"source_line":239,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$0::as_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":259,"record_offset":27272,"parent_record_offset":26856,"ancestor_record_offset":4216,"inlinee":7984,"source_line":96,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$0::from_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":259,"record_offset":22640,"parent_record_offset":22496,"ancestor_record_offset":4216,"inlinee":7988,"source_line":121,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$1::drop
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":259,"record_offset":26856,"parent_record_offset":26828,"ancestor_record_offset":4216,"inlinee":7978,"source_line":104,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":259,"record_offset":26592,"parent_record_offset":26568,"ancestor_record_offset":4216,"inlinee":13273,"source_line":186,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::wait_for_readers
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":259,"record_offset":22584,"parent_record_offset":22496,"ancestor_record_offset":4216,"inlinee":8000,"source_line":196,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] bytes::bytes::impl$3::drop
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bytes","version":"1.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytes-1.6.0/src/bytes.rs
//     evidence={"module":259,"record_offset":21476,"parent_record_offset":21452,"ancestor_record_offset":4216,"inlinee":16231,"source_line":546,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":259,"record_offset":12516,"parent_record_offset":12484,"ancestor_record_offset":4216,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":259,"record_offset":19876,"parent_record_offset":19848,"ancestor_record_offset":4216,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":259,"record_offset":19904,"parent_record_offset":19876,"ancestor_record_offset":4216,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":259,"record_offset":27832,"parent_record_offset":27808,"ancestor_record_offset":4216,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":259,"record_offset":27808,"parent_record_offset":27724,"ancestor_record_offset":4216,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":259,"record_offset":32620,"parent_record_offset":32592,"ancestor_record_offset":4216,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::gt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":259,"record_offset":6552,"parent_record_offset":5284,"ancestor_record_offset":4216,"inlinee":11236,"source_line":1407,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":259,"record_offset":4688,"parent_record_offset":4420,"ancestor_record_offset":4216,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":259,"record_offset":6628,"parent_record_offset":6600,"ancestor_record_offset":4216,"inlinee":5097,"source_line":1820,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$79::cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":259,"record_offset":37408,"parent_record_offset":37384,"ancestor_record_offset":4216,"inlinee":11375,"source_line":1836,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":259,"record_offset":32676,"parent_record_offset":32648,"ancestor_record_offset":4216,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":259,"record_offset":4928,"parent_record_offset":4896,"ancestor_record_offset":4216,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":259,"record_offset":20592,"parent_record_offset":16640,"ancestor_record_offset":4216,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":259,"record_offset":33016,"parent_record_offset":28800,"ancestor_record_offset":4216,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":259,"record_offset":25020,"parent_record_offset":24940,"ancestor_record_offset":4216,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":259,"record_offset":20272,"parent_record_offset":16640,"ancestor_record_offset":4216,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":259,"record_offset":20120,"parent_record_offset":20096,"ancestor_record_offset":4216,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":259,"record_offset":19952,"parent_record_offset":19928,"ancestor_record_offset":4216,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":259,"record_offset":41564,"parent_record_offset":41392,"ancestor_record_offset":4216,"inlinee":10401,"source_line":20,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":259,"record_offset":21508,"parent_record_offset":16640,"ancestor_record_offset":4216,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":259,"record_offset":22132,"parent_record_offset":22108,"ancestor_record_offset":4216,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":259,"record_offset":22156,"parent_record_offset":22132,"ancestor_record_offset":4216,"inlinee":6052,"source_line":796,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":259,"record_offset":22784,"parent_record_offset":22472,"ancestor_record_offset":4216,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":259,"record_offset":26900,"parent_record_offset":26856,"ancestor_record_offset":4216,"inlinee":7982,"source_line":252,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":259,"record_offset":27180,"parent_record_offset":27156,"ancestor_record_offset":4216,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":259,"record_offset":12544,"parent_record_offset":12516,"ancestor_record_offset":4216,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":259,"record_offset":19928,"parent_record_offset":19904,"ancestor_record_offset":4216,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":259,"record_offset":21936,"parent_record_offset":21912,"ancestor_record_offset":4216,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":259,"record_offset":23764,"parent_record_offset":23732,"ancestor_record_offset":4216,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":259,"record_offset":27592,"parent_record_offset":27564,"ancestor_record_offset":4216,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":259,"record_offset":41772,"parent_record_offset":41740,"ancestor_record_offset":4216,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":259,"record_offset":27752,"parent_record_offset":27724,"ancestor_record_offset":4216,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":259,"record_offset":27776,"parent_record_offset":27752,"ancestor_record_offset":4216,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":259,"record_offset":20212,"parent_record_offset":20188,"ancestor_record_offset":4216,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":259,"record_offset":27156,"parent_record_offset":26856,"ancestor_record_offset":4216,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":259,"record_offset":28020,"parent_record_offset":27968,"ancestor_record_offset":4216,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":259,"record_offset":41464,"parent_record_offset":41392,"ancestor_record_offset":4216,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":259,"record_offset":4776,"parent_record_offset":4744,"ancestor_record_offset":4216,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":259,"record_offset":4804,"parent_record_offset":4744,"ancestor_record_offset":4216,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":259,"record_offset":27692,"parent_record_offset":27564,"ancestor_record_offset":4216,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::byte_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":259,"record_offset":22712,"parent_record_offset":22688,"ancestor_record_offset":4216,"inlinee":7991,"source_line":1097,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":259,"record_offset":20096,"parent_record_offset":19676,"ancestor_record_offset":4216,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":259,"record_offset":22736,"parent_record_offset":22712,"ancestor_record_offset":4216,"inlinee":7992,"source_line":1039,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=232
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":259,"record_offset":5876,"parent_record_offset":5672,"ancestor_record_offset":4216,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":259,"record_offset":26672,"parent_record_offset":26648,"ancestor_record_offset":4216,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":259,"record_offset":12272,"parent_record_offset":12244,"ancestor_record_offset":4216,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":259,"record_offset":12320,"parent_record_offset":12296,"ancestor_record_offset":4216,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":259,"record_offset":12296,"parent_record_offset":12272,"ancestor_record_offset":4216,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":259,"record_offset":14608,"parent_record_offset":14576,"ancestor_record_offset":4216,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":259,"record_offset":26760,"parent_record_offset":26708,"ancestor_record_offset":4216,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":259,"record_offset":27208,"parent_record_offset":27180,"ancestor_record_offset":4216,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":259,"record_offset":22392,"parent_record_offset":22364,"ancestor_record_offset":4216,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":259,"record_offset":27236,"parent_record_offset":27180,"ancestor_record_offset":4216,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":25076,"parent_record_offset":24940,"ancestor_record_offset":4216,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":28184,"parent_record_offset":28160,"ancestor_record_offset":4216,"inlinee":6884,"source_line":564,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":5124,"parent_record_offset":4896,"ancestor_record_offset":4216,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":18476,"parent_record_offset":16640,"ancestor_record_offset":4216,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":17372,"parent_record_offset":17348,"ancestor_record_offset":4216,"inlinee":5961,"source_line":1439,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":27928,"parent_record_offset":27524,"ancestor_record_offset":4216,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":5432,"parent_record_offset":5284,"ancestor_record_offset":4216,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":5468,"parent_record_offset":5284,"ancestor_record_offset":4216,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$31::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":40540,"parent_record_offset":40492,"ancestor_record_offset":4216,"inlinee":10821,"source_line":524,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":259,"record_offset":32704,"parent_record_offset":32676,"ancestor_record_offset":4216,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":259,"record_offset":32732,"parent_record_offset":32704,"ancestor_record_offset":4216,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":259,"record_offset":17628,"parent_record_offset":17432,"ancestor_record_offset":4216,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":259,"record_offset":26648,"parent_record_offset":26624,"ancestor_record_offset":4216,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":259,"record_offset":32648,"parent_record_offset":32620,"ancestor_record_offset":4216,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":22528,"parent_record_offset":22496,"ancestor_record_offset":4216,"inlinee":11099,"source_line":1628,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":27380,"parent_record_offset":27356,"ancestor_record_offset":4216,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":22868,"parent_record_offset":22832,"ancestor_record_offset":4216,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":4628,"parent_record_offset":4604,"ancestor_record_offset":4216,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":30624,"parent_record_offset":30560,"ancestor_record_offset":4216,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":27404,"parent_record_offset":27380,"ancestor_record_offset":4216,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":4652,"parent_record_offset":4628,"ancestor_record_offset":4216,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":14636,"parent_record_offset":14576,"ancestor_record_offset":4216,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":22552,"parent_record_offset":22528,"ancestor_record_offset":4216,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":259,"record_offset":14668,"parent_record_offset":14540,"ancestor_record_offset":4216,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":259,"record_offset":6600,"parent_record_offset":6552,"ancestor_record_offset":4216,"inlinee":5095,"source_line":79,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] http::header::map::HeaderMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/http-0.2.9/src/header/map.rs
//     evidence={"module":259,"record_offset":17280,"parent_record_offset":16640,"ancestor_record_offset":4216,"inlinee":16205,"source_line":676,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] http::header::map::HeaderMap::get2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/http-0.2.9/src/header/map.rs
//     evidence={"module":259,"record_offset":17308,"parent_record_offset":17280,"ancestor_record_offset":4216,"inlinee":16206,"source_line":683,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] http::header::map::as_header_name::impl$4::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/http-0.2.9/src/header/map.rs
//     evidence={"module":259,"record_offset":17348,"parent_record_offset":17308,"ancestor_record_offset":4216,"inlinee":16208,"source_line":3422,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":259,"record_offset":4832,"parent_record_offset":4744,"ancestor_record_offset":4216,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":259,"record_offset":4720,"parent_record_offset":4420,"ancestor_record_offset":4216,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":259,"record_offset":4744,"parent_record_offset":4720,"ancestor_record_offset":4216,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":259,"record_offset":4856,"parent_record_offset":4832,"ancestor_record_offset":4216,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":259,"record_offset":4604,"parent_record_offset":4420,"ancestor_record_offset":4216,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::async_impl::request::Request::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/async_impl/request.rs
//     evidence={"module":259,"record_offset":30156,"parent_record_offset":30124,"ancestor_record_offset":4216,"inlinee":16221,"source_line":43,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::Client::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":259,"record_offset":29988,"parent_record_offset":28800,"ancestor_record_offset":4216,"inlinee":16214,"source_line":892,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::Client::request
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":259,"record_offset":30020,"parent_record_offset":29988,"ancestor_record_offset":4216,"inlinee":16215,"source_line":949,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$11::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":259,"record_offset":30480,"parent_record_offset":30020,"ancestor_record_offset":4216,"inlinee":16223,"source_line":49,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$12::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":259,"record_offset":30504,"parent_record_offset":30480,"ancestor_record_offset":4216,"inlinee":16225,"source_line":987,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$13::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":259,"record_offset":30660,"parent_record_offset":30504,"ancestor_record_offset":4216,"inlinee":16227,"source_line":1157,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$4::request::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":259,"record_offset":30096,"parent_record_offset":30060,"ancestor_record_offset":4216,"inlinee":16217,"source_line":950,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::request::Request::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/request.rs
//     evidence={"module":259,"record_offset":30124,"parent_record_offset":30096,"ancestor_record_offset":4216,"inlinee":16219,"source_line":37,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":259,"record_offset":38376,"parent_record_offset":38348,"ancestor_record_offset":4216,"inlinee":8126,"source_line":642,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":259,"record_offset":5024,"parent_record_offset":5000,"ancestor_record_offset":4216,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":259,"record_offset":5072,"parent_record_offset":5048,"ancestor_record_offset":4216,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":259,"record_offset":5048,"parent_record_offset":5024,"ancestor_record_offset":4216,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":259,"record_offset":5520,"parent_record_offset":5496,"ancestor_record_offset":4216,"inlinee":13008,"source_line":2036,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::Metadata::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":259,"record_offset":5496,"parent_record_offset":5284,"ancestor_record_offset":4216,"inlinee":16199,"source_line":1622,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":259,"record_offset":4896,"parent_record_offset":4420,"ancestor_record_offset":4216,"inlinee":12459,"source_line":2936,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::remove_dir_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":259,"record_offset":6664,"parent_record_offset":5284,"ancestor_record_offset":4216,"inlinee":16200,"source_line":2860,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":259,"record_offset":5672,"parent_record_offset":5284,"ancestor_record_offset":4216,"inlinee":8068,"source_line":2342,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":259,"record_offset":12244,"parent_record_offset":12208,"ancestor_record_offset":4216,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":259,"record_offset":12208,"parent_record_offset":12172,"ancestor_record_offset":4216,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":41740,"parent_record_offset":41324,"ancestor_record_offset":4216,"inlinee":10391,"source_line":646,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":29204,"parent_record_offset":28800,"ancestor_record_offset":4216,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":4216,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Prefix::is_verbatim
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":41836,"parent_record_offset":41808,"ancestor_record_offset":4216,"inlinee":10393,"source_line":222,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$100::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":41712,"parent_record_offset":41324,"ancestor_record_offset":4216,"inlinee":10389,"source_line":371,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$116::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":41392,"parent_record_offset":41324,"ancestor_record_offset":4216,"inlinee":10395,"source_line":589,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":41324,"parent_record_offset":41028,"ancestor_record_offset":4216,"inlinee":10387,"source_line":987,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":38348,"parent_record_offset":38268,"ancestor_record_offset":4216,"inlinee":8124,"source_line":1738,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":4976,"parent_record_offset":4952,"ancestor_record_offset":4216,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$60::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":41028,"parent_record_offset":40868,"ancestor_record_offset":4216,"inlinee":10524,"source_line":2057,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":4952,"parent_record_offset":4928,"ancestor_record_offset":4216,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$89::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":41504,"parent_record_offset":41464,"ancestor_record_offset":4216,"inlinee":10397,"source_line":135,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$98::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":259,"record_offset":41536,"parent_record_offset":41392,"ancestor_record_offset":4216,"inlinee":10399,"source_line":371,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":259,"record_offset":38404,"parent_record_offset":38376,"ancestor_record_offset":4216,"inlinee":8128,"source_line":13,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":259,"record_offset":5544,"parent_record_offset":5520,"ancestor_record_offset":4216,"inlinee":13010,"source_line":1126,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_symlink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":259,"record_offset":5568,"parent_record_offset":5544,"ancestor_record_offset":4216,"inlinee":13011,"source_line":1132,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::impl$20::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":259,"record_offset":16548,"parent_record_offset":16524,"ancestor_record_offset":4216,"inlinee":5093,"source_line":14,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::impl$2::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":259,"record_offset":40940,"parent_record_offset":40916,"ancestor_record_offset":4216,"inlinee":16254,"source_line":110,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::impl$4::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":259,"record_offset":37360,"parent_record_offset":37336,"ancestor_record_offset":4216,"inlinee":16239,"source_line":118,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::impl$5::cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":259,"record_offset":37384,"parent_record_offset":37360,"ancestor_record_offset":4216,"inlinee":16241,"source_line":124,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":259,"record_offset":27648,"parent_record_offset":27620,"ancestor_record_offset":4216,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$35::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys_common\wtf8.rs
//     evidence={"module":259,"record_offset":38432,"parent_record_offset":38404,"ancestor_record_offset":4216,"inlinee":8130,"source_line":132,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":259,"record_offset":27564,"parent_record_offset":27524,"ancestor_record_offset":4216,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$23::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\time.rs
//     evidence={"module":259,"record_offset":16524,"parent_record_offset":16492,"ancestor_record_offset":4216,"inlinee":5091,"source_line":149,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$29::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\time.rs
//     evidence={"module":259,"record_offset":40916,"parent_record_offset":40868,"ancestor_record_offset":4216,"inlinee":16252,"source_line":242,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$31::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\time.rs
//     evidence={"module":259,"record_offset":37336,"parent_record_offset":37312,"ancestor_record_offset":4216,"inlinee":16237,"source_line":242,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] theme_defs::impl$0::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"theme-defs","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-defs/src/lib.rs
//     evidence={"module":259,"record_offset":38308,"parent_record_offset":38268,"ancestor_record_offset":4216,"inlinee":16243,"source_line":3,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] theme_defs::impl$2::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-defs","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-defs/src/lib.rs
//     evidence={"module":259,"record_offset":40628,"parent_record_offset":40600,"ancestor_record_offset":4216,"inlinee":16246,"source_line":3,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] theme_defs::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-defs","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-defs/src/lib.rs
//     evidence={"module":259,"record_offset":42140,"parent_record_offset":42084,"ancestor_record_offset":4216,"inlinee":16248,"source_line":10,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] theme_defs::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-defs","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-defs/src/lib.rs
//     evidence={"module":259,"record_offset":40868,"parent_record_offset":40828,"ancestor_record_offset":4216,"inlinee":16250,"source_line":10,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::ThemeFetcher::validate_cas_cache_entry
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/lib.rs
//     evidence={"module":259,"record_offset":5284,"parent_record_offset":4420,"ancestor_record_offset":4216,"inlinee":16197,"source_line":226,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::impl$0::run::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/lib.rs
//     evidence={"module":259,"record_offset":35848,"parent_record_offset":35800,"ancestor_record_offset":4216,"inlinee":16233,"source_line":294,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::impl$0::run::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/lib.rs
//     evidence={"module":259,"record_offset":39720,"parent_record_offset":39692,"ancestor_record_offset":4216,"inlinee":16244,"source_line":305,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::impl$0::run::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 294
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F69
// 首条位置: {"module":259,"inlinee":16233}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":259,"record_offset":35884,"parent_record_offset":35848,"ancestor_record_offset":35848,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":259,"record_offset":36320,"parent_record_offset":35848,"ancestor_record_offset":35848,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":36256,"parent_record_offset":36228,"ancestor_record_offset":35848,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":259,"record_offset":36228,"parent_record_offset":36164,"ancestor_record_offset":35848,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":36196,"parent_record_offset":36164,"ancestor_record_offset":35848,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":36164,"parent_record_offset":36136,"ancestor_record_offset":35848,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":259,"record_offset":36136,"parent_record_offset":36112,"ancestor_record_offset":35848,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":259,"record_offset":35972,"parent_record_offset":35920,"ancestor_record_offset":35848,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":259,"record_offset":35920,"parent_record_offset":35884,"ancestor_record_offset":35848,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":259,"record_offset":36000,"parent_record_offset":35920,"ancestor_record_offset":35848,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":259,"record_offset":36064,"parent_record_offset":35920,"ancestor_record_offset":35848,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":259,"record_offset":36028,"parent_record_offset":35920,"ancestor_record_offset":35848,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00259.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::impl$0::run::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 305
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F74
// 首条位置: {"module":259,"inlinee":16244}
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
// [PDB 命名实体；类型/枚举变体身份待确认] theme_fetcher::MaybePendingActiveEvent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 21
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["theme_fetcher::MaybePendingActiveEvent", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::HybridProtection<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "arc_swap::strategy::hybrid::HybridProtection", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<arc_swap::strategy::hybrid::HybridProtection<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["arc_swap::strategy::hybrid::HybridProtection<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::HybridProtection::fallback<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "arc_swap::strategy::hybrid::HybridProtection::fallback", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>", "constructor": "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "arc_swap::strategy::hybrid::DefaultConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::helping::Slots::help<arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>,alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "arc_swap::debt::helping::Slots::help", "arguments": ["arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>", "alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::impl$2::pay_all::closure$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::debt::impl$2::pay_all::closure$0", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::impl$5::load::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>", "constructor": "arc_swap::strategy::hybrid::impl$5::load::closure_env$0", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "arc_swap::strategy::hybrid::DefaultConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::list::LocalNode::with<arc_swap::strategy::hybrid::HybridProtection<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> >,arc_swap::strategy::hybrid::impl$5::load::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::debt::list::LocalNode::with", "arguments": ["arc_swap::strategy::hybrid::HybridProtection<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> >", "arc_swap::strategy::hybrid::impl$5::load::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::impl$2::pay_all::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::debt::impl$2::pay_all::closure_env$0", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::list::LocalNode::with<tuple$<>,arc_swap::debt::impl$2::pay_all::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig> > >", "constructor": "arc_swap::debt::list::LocalNode::with", "arguments": ["tuple$<>", "arc_swap::debt::impl$2::pay_all::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global> >", "constructor": "alloc::sync::ArcInner", "arguments": ["alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::ArcInner<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::ArcInner<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::ArcSwapAny<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::ArcSwapAny", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<arc_swap::ArcSwapAny<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["arc_swap::ArcSwapAny<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::Guard<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::Guard", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<arc_swap::Guard<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["arc_swap::Guard<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["theme_fetcher::MaybePendingActiveEvent", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::vec::impl$20::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["theme_fetcher::MaybePendingActiveEvent", "alloc::vec::impl$20::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<theme_fetcher::MaybePendingActiveEvent,core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::vec::impl$20::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["theme_fetcher::MaybePendingActiveEvent", "core::slice::iter::Iter<theme_server_proto::ActiveEvent>", "theme_fetcher::impl$0::fetch_events::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::vec::impl$20::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] theme_fetcher::ThemeFetcher
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "theme_fetcher::ThemeFetcher::new<std::path::PathBuf>", "constructor": "theme_fetcher::ThemeFetcher::new", "arguments": ["std::path::PathBuf"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<theme_fetcher::ThemeFetcher>", "constructor": "core::ptr::drop_in_place", "arguments": ["theme_fetcher::ThemeFetcher"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: serde_json::de::Deserializer::parse_exponent<serde_json::read::SliceRead>
// 信息 ID: 3d775db911774ef6e0660b41
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::Deserializer::parse_exponent -> ["serde_json::read::SliceRead"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::Deserializer::parse_number<serde_json::read::SliceRead> @ 0x5742c -> 0x58210 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_number<serde_json::read::SliceRead> @ 0x57579 -> 0x58210 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead> @ 0x614196 -> 0x58210 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> @ 0x614376 -> 0x58210 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> @ 0x614479 -> 0x58210 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_long_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_long_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_long_integer<serde_json::read::SliceRead> @ 0x28a49f7 -> 0x58210 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_decimal_overflow<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal_overflow<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal_overflow<serde_json::read::SliceRead> @ 0x28a4b83 -> 0x58210 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead>
// 信息 ID: f706ce6298aa4eb3f5229dde
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::Deserializer::parse_integer -> ["serde_json::read::SliceRead"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impl$5::deserialize<i64,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0x615826 -> 0x614300 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::de::impl$5::deserialize<i64,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0x61585b -> 0x614300 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::de::impl$5::deserialize<u32,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0xf74193 -> 0x614300 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::de::impl$5::deserialize<u32,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0xf741cd -> 0x614300 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0x1406989 -> 0x614300 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0x14069d1 -> 0x614300 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x28a46fe -> 0x614300 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x28a4775 -> 0x614300 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x28e4f8e -> 0x614300 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x28e5005 -> 0x614300 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: std::io::cursor::impl$5::read_to_string<ref$<bytes::bytes::Bytes> >
// 信息 ID: a4a96c8dcbe459eae369a1ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["bytes::bytes::Bytes"]; 字段布局 = unknown。
// [类型关系] std::io::cursor::impl$5::read_to_string -> ["ref$<bytes::bytes::Bytes>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: std::io::default_read_to_end<zip::read::ZipFile>
// 信息 ID: 92ce9275c23d391f1bc8ef6c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] std::io::default_read_to_end -> ["zip::read::ZipFile"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] theme_fetcher::checked_zip::extract_with_checksums<std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> > > @ 0xd9257b -> 0x19af170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::ThemeFetcher::download_blob
// 信息 ID: bf1c0823f7a31e615c77d951
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=259, inline_record=28800; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::ThemeFetcher::fetch_events
// 信息 ID: 13e56d0d1b15fc880149fde4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=259, inline_record=16640; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::ThemeFetcher::new<std::path::PathBuf>
// 信息 ID: 7e2f329303909b68a26d83ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] theme_fetcher::ThemeFetcher::new -> ["std::path::PathBuf"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::theme_fetcher_plugin::impl$0::build @ 0x3d10d5 -> 0x54aa00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::ThemeFetcher::run
// 信息 ID: 8b9ebf8a8cac38de9359d792
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0,tuple$<> > @ 0xd83c1 -> 0x618310 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::ThemeFetcher::trim_cas
// 信息 ID: 5110e9157ecb5a03a2655f1f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=259, inline_record=4420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::ThemeFetcher::validate_cas_cache_entry
// 信息 ID: fc896017892e1e274b6d25c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=259, inline_record=5284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::extract_with_checksums<std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> > >
// 信息 ID: 2f230566b7c03de394947fa7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["bytes::bytes::Bytes"]; 字段布局 = unknown。
// [类型关系] std::io::cursor::Cursor -> ["ref$<bytes::bytes::Bytes>"]; 字段布局 = unknown。
// [类型关系] theme_fetcher::checked_zip::extract_with_checksums -> ["std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] theme_fetcher::ThemeFetcher::run @ 0x61b42a -> 0xd921c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::verify_folder_checksums
// 信息 ID: 96c525f6fcbd7e83db80fbd4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] theme_fetcher::ThemeFetcher::run @ 0x618ade -> 0xd939a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::impl$0::fetch_events::closure$0
// 信息 ID: ad7c970adbeda207702981b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "theme_fetcher::impl$0::fetch_events", "captures": "unknown"}
// [闭包环境类型引用] theme_fetcher::impl$0::fetch_events::closure_env$0 <- core::iter::adapters::map::impl$2::fold<theme_fetcher::MaybePendingActiveEvent,core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::vec::impl$20::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] theme_fetcher::impl$0::fetch_events::closure_env$0 <- core::iter::adapters::map::impl$2::fold<theme_fetcher::MaybePendingActiveEvent,core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::vec::impl$20::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<theme_fetcher::MaybePendingActiveEvent,core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::vec::impl$20::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> > > >; module=866, inline_record=1172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::impl$0::run::closure$0
// 信息 ID: 7a6d6ce5e307aedb508e45ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "theme_fetcher::impl$0::run", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=259, inline_record=35848; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::impl$0::run::closure$1
// 信息 ID: 3f3a2970080e77a5df1272b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "theme_fetcher::impl$0::run", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=259, inline_record=39720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zip::read::zip_archive::ZipArchive::new<std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> > >
// 信息 ID: d0de0066f894a52ac7845a98
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["bytes::bytes::Bytes"]; 字段布局 = unknown。
// [类型关系] std::io::cursor::Cursor -> ["ref$<bytes::bytes::Bytes>"]; 字段布局 = unknown。
// [类型关系] zip::read::zip_archive::ZipArchive::new -> ["std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] theme_fetcher::ThemeFetcher::run @ 0x61b38f -> 0xdb73a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
