// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/placement_data.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: ffa42cf859e7cba8a1e8626224ee661e0641fc2cf7b1f1acf7fa3fc228380cf7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: placement_data.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 477, 538, 567, 586, 1600, 1615, 1622
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
// 函数符号: country_core::resources::placement_data::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x829F
// 首条位置: {"module":1600,"inlinee":33439}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::placement_data::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":586,"record_offset":73976,"rva":11778400,"va":5380487520}
// 编译实例 RVAs: 0xb3b960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: placement_data.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31359551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380487520,"raw_hex":"e83f82de01","size":5,"soft_fail":false,"target_va":5411847152,"va":5380487596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":586,"record_offset":74168,"parent_record_offset":73976,"ancestor_record_offset":73976,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":586,"record_offset":74196,"parent_record_offset":74168,"ancestor_record_offset":73976,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":586,"record_offset":74224,"parent_record_offset":74196,"ancestor_record_offset":73976,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":586,"record_offset":74252,"parent_record_offset":74224,"ancestor_record_offset":73976,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::placement_data::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11BDC
// 首条位置: {"module":1600,"inlinee":72668}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1600,"record_offset":411848,"parent_record_offset":411824,"ancestor_record_offset":411824,"inlinee":6604,"source_line":126,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1600,"record_offset":411992,"parent_record_offset":411872,"ancestor_record_offset":411824,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":411908,"parent_record_offset":411872,"ancestor_record_offset":411824,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":411872,"parent_record_offset":411848,"ancestor_record_offset":411824,"inlinee":6421,"source_line":2151,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":411932,"parent_record_offset":411908,"ancestor_record_offset":411824,"inlinee":6488,"source_line":642,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":411956,"parent_record_offset":411932,"ancestor_record_offset":411824,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::placement_data::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7F02
// 首条位置: {"module":586,"inlinee":32514}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::placement_data::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11BDA
// 首条位置: {"module":1600,"inlinee":72666}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1600,"record_offset":412080,"parent_record_offset":410872,"ancestor_record_offset":410872,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1600,"record_offset":415976,"parent_record_offset":410872,"ancestor_record_offset":410872,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::placement_data::default_anim_t
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/placement_data.rs
//     evidence={"module":1600,"record_offset":416232,"parent_record_offset":410872,"ancestor_record_offset":410872,"inlinee":32488,"source_line":43,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1600,"record_offset":416260,"parent_record_offset":410872,"ancestor_record_offset":410872,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1600,"record_offset":410924,"parent_record_offset":410872,"ancestor_record_offset":410872,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1600,"record_offset":412112,"parent_record_offset":410872,"ancestor_record_offset":410872,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1600,"record_offset":411992,"parent_record_offset":411872,"ancestor_record_offset":410872,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1600,"record_offset":412036,"parent_record_offset":411000,"ancestor_record_offset":410872,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::placement_data::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/placement_data.rs
//     evidence={"module":1600,"record_offset":411824,"parent_record_offset":411800,"ancestor_record_offset":410872,"inlinee":72668,"source_line":8,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1600,"record_offset":416332,"parent_record_offset":416308,"ancestor_record_offset":410872,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1600,"record_offset":416420,"parent_record_offset":416396,"ancestor_record_offset":410872,"inlinee":6671,"source_line":126,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1600,"record_offset":416644,"parent_record_offset":416620,"ancestor_record_offset":410872,"inlinee":27074,"source_line":126,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_tuple
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1600,"record_offset":416308,"parent_record_offset":416284,"ancestor_record_offset":410872,"inlinee":6683,"source_line":126,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1600,"record_offset":411800,"parent_record_offset":411000,"ancestor_record_offset":410872,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$54::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1600,"record_offset":416620,"parent_record_offset":416596,"ancestor_record_offset":410872,"inlinee":13338,"source_line":134,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1600,"record_offset":413056,"parent_record_offset":413032,"ancestor_record_offset":410872,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1600,"record_offset":416396,"parent_record_offset":416372,"ancestor_record_offset":410872,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$96::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1600,"record_offset":415936,"parent_record_offset":415912,"ancestor_record_offset":410872,"inlinee":13361,"source_line":1338,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$99::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1600,"record_offset":412548,"parent_record_offset":412524,"ancestor_record_offset":410872,"inlinee":6681,"source_line":1338,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":411400,"parent_record_offset":411116,"ancestor_record_offset":410872,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":412172,"parent_record_offset":412140,"ancestor_record_offset":410872,"inlinee":5761,"source_line":982,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":411116,"parent_record_offset":411000,"ancestor_record_offset":410872,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":411204,"parent_record_offset":411116,"ancestor_record_offset":410872,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":411000,"parent_record_offset":410924,"ancestor_record_offset":410872,"inlinee":6601,"source_line":1963,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":412140,"parent_record_offset":412112,"ancestor_record_offset":410872,"inlinee":6516,"source_line":1996,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":411872,"parent_record_offset":411848,"ancestor_record_offset":410872,"inlinee":6421,"source_line":2151,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1600,"record_offset":411848,"parent_record_offset":411824,"ancestor_record_offset":410872,"inlinee":6604,"source_line":126,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":411424,"parent_record_offset":411400,"ancestor_record_offset":410872,"inlinee":6488,"source_line":642,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":411264,"parent_record_offset":411204,"ancestor_record_offset":410872,"inlinee":6487,"source_line":637,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":411448,"parent_record_offset":411424,"ancestor_record_offset":410872,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":411324,"parent_record_offset":411264,"ancestor_record_offset":410872,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::placement_data::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11BDD
// 首条位置: {"module":1600,"inlinee":72669}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1600,"record_offset":416832,"parent_record_offset":416796,"ancestor_record_offset":416796,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::placement_data::default_anim_t
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/placement_data.rs
//     evidence={"module":1600,"record_offset":417056,"parent_record_offset":416796,"ancestor_record_offset":416796,"inlinee":32488,"source_line":43,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1600,"record_offset":416860,"parent_record_offset":416796,"ancestor_record_offset":416796,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::placement_data::default_anim_t
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7EE8
// 首条位置: {"module":1600,"inlinee":32488}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::placement_data::impl$1::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7EEA
// 首条位置: {"module":1622,"inlinee":32490}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::placement_data::PlacementDataSsbo::to_transform
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x66A3
// 首条位置: {"module":477,"inlinee":26275}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_axis_angle
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":477,"record_offset":16196,"parent_record_offset":16164,"ancestor_record_offset":16164,"inlinee":24914,"source_line":136,"archive":"_pdb/B/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":477,"record_offset":16336,"parent_record_offset":16312,"ancestor_record_offset":16164,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":477,"record_offset":16224,"parent_record_offset":16196,"ancestor_record_offset":16164,"inlinee":24915,"source_line":167,"archive":"_pdb/B/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":477,"record_offset":16312,"parent_record_offset":16196,"ancestor_record_offset":16164,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":477,"record_offset":16272,"parent_record_offset":16248,"ancestor_record_offset":16164,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":477,"record_offset":16248,"parent_record_offset":16224,"ancestor_record_offset":16164,"inlinee":24916,"source_line":860,"archive":"_pdb/B/modules/00477.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::placement_data::PlacementDataSsbo::to_affine3a
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x66A2
// 首条位置: {"module":477,"inlinee":26274}
// 编译实例 RVAs: 0xad5020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: placement_data.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31780352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380067360,"raw_hex":"e800eee401","size":5,"soft_fail":false,"target_va":5411847808,"va":5380067451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: placement_data.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31780699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380067360,"raw_hex":"e85befe401","size":5,"soft_fail":false,"target_va":5411848144,"va":5380067440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::placement_data::PlacementDataSsbo::to_transform
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/placement_data.rs
//     evidence={"module":477,"record_offset":16164,"parent_record_offset":16140,"ancestor_record_offset":16140,"inlinee":26275,"source_line":75,"archive":"_pdb/B/modules/00477.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":477,"record_offset":16376,"parent_record_offset":16140,"ancestor_record_offset":16140,"inlinee":24721,"source_line":29,"archive":"_pdb/B/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":477,"record_offset":16836,"parent_record_offset":16400,"ancestor_record_offset":16140,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":477,"record_offset":16636,"parent_record_offset":16584,"ancestor_record_offset":16140,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":477,"record_offset":16608,"parent_record_offset":16584,"ancestor_record_offset":16140,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":477,"record_offset":16336,"parent_record_offset":16312,"ancestor_record_offset":16140,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":477,"record_offset":16400,"parent_record_offset":16376,"ancestor_record_offset":16140,"inlinee":24722,"source_line":233,"archive":"_pdb/B/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":477,"record_offset":16224,"parent_record_offset":16196,"ancestor_record_offset":16140,"inlinee":24915,"source_line":167,"archive":"_pdb/B/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":477,"record_offset":16436,"parent_record_offset":16400,"ancestor_record_offset":16140,"inlinee":24723,"source_line":270,"archive":"_pdb/B/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_axis_angle
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":477,"record_offset":16196,"parent_record_offset":16164,"ancestor_record_offset":16140,"inlinee":24914,"source_line":136,"archive":"_pdb/B/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":477,"record_offset":16492,"parent_record_offset":16436,"ancestor_record_offset":16140,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":477,"record_offset":16860,"parent_record_offset":16836,"ancestor_record_offset":16140,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":477,"record_offset":16584,"parent_record_offset":16400,"ancestor_record_offset":16140,"inlinee":24650,"source_line":1245,"archive":"_pdb/B/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":477,"record_offset":16312,"parent_record_offset":16196,"ancestor_record_offset":16140,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":477,"record_offset":16272,"parent_record_offset":16248,"ancestor_record_offset":16140,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":477,"record_offset":16248,"parent_record_offset":16224,"ancestor_record_offset":16140,"inlinee":24916,"source_line":860,"archive":"_pdb/B/modules/00477.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::placement_data::PlacementDataSsbo::position
// 记录形式: compiled_procedure; instances=1
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":567,"record_offset":35716,"rva":11358624,"va":5380067744}
// 编译实例 RVAs: 0xad51a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":567,"record_offset":35868,"parent_record_offset":35716,"ancestor_record_offset":35716,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":567,"record_offset":35908,"parent_record_offset":35888,"ancestor_record_offset":35716,"inlinee":4588,"source_line":75,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$94::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":567,"record_offset":35888,"parent_record_offset":35868,"ancestor_record_offset":35716,"inlinee":26261,"source_line":1873,"archive":"_pdb/B/modules/00567.jsonl.gz"}
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
// 完整结构及原始类型记录: placement_data.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::placement_data::PlacementDataSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 33
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::asset_libraries::Handle", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize>", "constructor": "tuple$", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>", "usize"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize> >", "constructor": "tuple$", "arguments": ["country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>", "tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::NaniInstanceData<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::nani::gpu_world::NaniInstanceData", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::nani::gpu_world::NaniInstanceData<country_core::resources::placement_data::PlacementDataSsbo> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::nani::gpu_world::NaniInstanceData<country_core::resources::placement_data::PlacementDataSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<country_core::resources::placement_data::PlacementDataSsbo,serde_json::de::SeqAccess<serde_json::read::StrRead> >", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo", "serde_json::de::SeqAccess<serde_json::read::StrRead>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo> >", "constructor": "tuple$", "arguments": ["std::path::PathBuf", "country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["std::path::PathBuf", "country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>", "tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::clear<tuple$<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTable::clear", "arguments": ["tuple$<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::clear<tuple$<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTable::clear", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "core::slice::iter::Iter", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::placement_data::PlacementDataSsbo>,country_core::startup::clearing::init_clearing::closure_env$0>", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::slice::iter::Iter<country_core::resources::placement_data::PlacementDataSsbo>", "country_core::startup::clearing::init_clearing::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$0::spec_extend<country_core::resources::tree_storage::ForestTree,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::placement_data::PlacementDataSsbo>,country_core::startup::clearing::init_clearing::closure_env$0>,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$0::spec_extend", "arguments": ["country_core::resources::tree_storage::ForestTree", "core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::placement_data::PlacementDataSsbo>,country_core::startup::clearing::init_clearing::closure_env$0>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::from_str<alloc::vec::Vec<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global> >", "constructor": "serde_json::de::from_str", "arguments": ["alloc::vec::Vec<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<alloc::vec::Vec<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global>,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["alloc::vec::Vec<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global>", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<alloc::vec::Vec<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global>,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<alloc::vec::Vec<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global>,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<alloc::vec::Vec<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global>,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<alloc::vec::Vec<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global>,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "core::marker::PhantomData", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$9::next_element_seed<serde_json::read::StrRead,core::marker::PhantomData<country_core::resources::placement_data::PlacementDataSsbo> >", "constructor": "serde_json::de::impl$9::next_element_seed", "arguments": ["serde_json::read::StrRead", "core::marker::PhantomData<country_core::resources::placement_data::PlacementDataSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$7::deserialize_seq<serde_json::read::StrRead,serde::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::placement_data::PlacementDataSsbo> >", "constructor": "serde_json::de::impl$7::deserialize_seq", "arguments": ["serde_json::read::StrRead", "serde::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::placement_data::PlacementDataSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["std::path::PathBuf", "country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>", "tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::nani::gpu_world::NaniInstanceData<country_core::resources::placement_data::PlacementDataSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::nani::gpu_world::NaniInstanceData<country_core::resources::placement_data::PlacementDataSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$11::extend::closure_env$1<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::nani::gpu_world::impl$11::extend::closure_env$1", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::once::impl$2::call_once::closure$0<country_core::render::nani::gpu_world::impl$11::extend::closure_env$1<country_core::resources::placement_data::PlacementDataSsbo> >", "constructor": "std::sync::poison::once::impl$2::call_once::closure$0", "arguments": ["country_core::render::nani::gpu_world::impl$11::extend::closure_env$1<country_core::resources::placement_data::PlacementDataSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::once::impl$2::call_once::closure_env$0<country_core::render::nani::gpu_world::impl$11::extend::closure_env$1<country_core::resources::placement_data::PlacementDataSsbo> >", "constructor": "std::sync::poison::once::impl$2::call_once::closure_env$0", "arguments": ["country_core::render::nani::gpu_world::impl$11::extend::closure_env$1<country_core::resources::placement_data::PlacementDataSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<std::sync::poison::once::impl$2::call_once::closure_env$0<country_core::render::nani::gpu_world::impl$11::extend::closure_env$1<country_core::resources::placement_data::PlacementDataSsbo> >,tuple$<ref$<std::sync::poison::once::OnceState> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["std::sync::poison::once::impl$2::call_once::closure_env$0<country_core::render::nani::gpu_world::impl$11::extend::closure_env$1<country_core::resources::placement_data::PlacementDataSsbo> >", "tuple$<ref$<std::sync::poison::once::OnceState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::resources::placement_data::PlacementDataSsbo>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::GpuWorld::create_instances<country_core::resources::placement_data::PlacementDataSsbo,country_core::systems::prefab::create_prefab_gpu_instances::closure_env$0>", "constructor": "country_core::render::nani::gpu_world::GpuWorld::create_instances", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo", "country_core::systems::prefab::create_prefab_gpu_instances::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$14::type_name<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::nani::gpu_world::impl$14::type_name", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$14::as_any<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::nani::gpu_world::impl$14::as_any", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$14::as_any_mut<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::nani::gpu_world::impl$14::as_any_mut", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::nani::gpu_world::impl$14::gpu_flush", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$14::copy<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::nani::gpu_world::impl$14::copy", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer::new<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer::new", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::new", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: placement_data.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-critters/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::placement_data::PlacementDataSsbo", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: placement_data.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::resources::placement_data::PlacementDataSsbo::position
// 信息 ID: 1b1d3f55dadc749a6ff90245
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] forest::manage_tree_prefabs::manage_tree_prefabs @ 0x1bf1bda -> 0xad51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::tree_move_bend::tree_move_bend @ 0x2268c86 -> 0xad51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::recalc_tree_position_on_terrain_height_change::recalc_tree_position_on_terrain_height_change @ 0x2268f84 -> 0xad51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::recalc_tree_position_on_terrain_height_change::recalc_tree_position_on_terrain_height_change @ 0x22690f7 -> 0xad51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::hide_trees::hide_trees @ 0x226a676 -> 0xad51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::hide_trees::hide_trees @ 0x226aced -> 0xad51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::convert_preplaced_trees::convert_preplaced_trees @ 0x226c36a -> 0xad51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::convert_preplaced_trees::convert_preplaced_trees @ 0x226c546 -> 0xad51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_tree_vegetation_setdressing::add_tree_setdressing @ 0x22d3c56 -> 0xad51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_tree_vegetation_setdressing::add_tree_setdressing @ 0x22d3ca8 -> 0xad51a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::placement_data::PlacementDataSsbo::to_affine3a
// 信息 ID: 1464ca0dccebfe37edafdeea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::wren::update_wrens @ 0x231e6b3 -> 0xad5020 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren_boids::sim_wren_boids @ 0x2327830 -> 0xad5020 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren_boids::sim_wren_boids @ 0x2328c65 -> 0xad5020 (direct_call); 同地址别名未裁定。
// [静态直接引用] itertools::Itertools::multiunzip<core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::tree_storage::BirdHaven>,ref$<country_core::systems::prefab::PrefabInstanceData> >,bevy_ecs::query::filter::Without<country_core::components::TreeMarkedForDeletion> >,system_critters::wren_boids::sim_wren_boids::closure_env$2>,tuple$<alloc::vec::Vec<utils::geometry::intersect::Sphere,alloc::alloc::Global>,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global> > > @ 0x261fe04 -> 0xad5020 (direct_call); 同地址别名未裁定。
// [静态直接引用] itertools::Itertools::multiunzip<core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::tree_storage::BirdHaven>,ref$<country_core::systems::prefab::PrefabInstanceData> >,bevy_ecs::query::filter::Without<country_core::components::TreeMarkedForDeletion> >,system_critters::wren_boids::sim_wren_boids::closure_env$2>,tuple$<alloc::vec::Vec<utils::geometry::intersect::Sphere,alloc::alloc::Global>,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global> > > @ 0x2620015 -> 0xad5020 (direct_call); 同地址别名未裁定。
// [静态直接引用] itertools::Itertools::multiunzip<core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::tree_storage::BirdHaven>,ref$<country_core::systems::prefab::PrefabInstanceData> >,bevy_ecs::query::filter::Without<country_core::components::TreeMarkedForDeletion> >,system_critters::wren_boids::sim_wren_boids::closure_env$2>,tuple$<alloc::vec::Vec<utils::geometry::intersect::Sphere,alloc::alloc::Global>,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global> > > @ 0x26201bd -> 0xad5020 (direct_call); 同地址别名未裁定。
// [静态直接引用] itertools::Itertools::multiunzip<core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::tree_storage::BirdHaven>,ref$<country_core::systems::prefab::PrefabInstanceData> >,bevy_ecs::query::filter::Without<country_core::components::TreeMarkedForDeletion> >,system_critters::wren_boids::sim_wren_boids::closure_env$2>,tuple$<alloc::vec::Vec<utils::geometry::intersect::Sphere,alloc::alloc::Global>,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global> > > @ 0x26205f0 -> 0xad5020 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::prefab::create_prefab_rt; module=477, inline_record=16140; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::prefab::update_prefabs; module=477, inline_record=28448; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::placement_data::PlacementDataSsbo::to_transform
// 信息 ID: 4378e0a00769a3e27bc496e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::prefab::create_prefab_rt; module=477, inline_record=16164; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::prefab::update_prefabs; module=477, inline_record=28472; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<glam::f32::vec3::Vec3> >,country_core::startup::clearing::init_clearing::closure_env$1>; module=538, inline_record=185388; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::placement_data::PlacementDataSsbo::to_affine3a; module=567, inline_record=34952; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::glade_themes::flowery::startup_flowery; module=1615, inline_record=43792; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::glade_themes::olden::startup_olden; module=1615, inline_record=46620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::placement_data::_::impl$0::deserialize
// 信息 ID: 5d2ab109f542086cdb425da7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$9::next_element_seed<serde_json::read::StrRead,core::marker::PhantomData<country_core::resources::placement_data::PlacementDataSsbo> >; module=1600, inline_record=134272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::placement_data::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: 442a48d97ae6d069685b785f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::placement_data::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::placement_data::_::impl$0::deserialize::__Visitor> @ 0x142fa62 -> 0xb3b960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::placement_data::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 87f737f8adcab8b5c9e327e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::placement_data::_::impl$0::deserialize::__Visitor>; module=1600, inline_record=411824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::placement_data::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 64d8d7da599087dd19da48ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::placement_data::_::impl$0::deserialize::__Visitor>; module=586, inline_record=1480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::placement_data::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 754a7347253be6112dad1374
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::placement_data::_::impl$0::deserialize::__Visitor>; module=1600, inline_record=410872; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::placement_data::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: a7d599367b518ff3f9bc4d7e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::placement_data::_::impl$0::deserialize::__Visitor>; module=1600, inline_record=416796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::placement_data::default_anim_t
// 信息 ID: 0ec0419d00e4c76164ffeb0d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::placement_data::_::impl$0::deserialize::__Visitor>; module=1600, inline_record=416232; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::placement_data::_::impl$0::deserialize::__Visitor>; module=1600, inline_record=417056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::placement_data::impl$1::default
// 信息 ID: 914a7786cec9d639a90e5d7b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >; module=1622, inline_record=20796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
