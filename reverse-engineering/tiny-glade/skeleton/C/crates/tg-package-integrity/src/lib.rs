// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tg-package-integrity/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 867416bf961d26aea35c94844920a9af9ac34b2c5dd11c1317c3f1cea0374e66
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 341, 1220, 1221, 1222, 1223, 1224
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use blake3; // namespace candidate
//   package identities: [{"name":"blake3","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.5.3"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use relative_path; // namespace candidate
//   package identities: [{"name":"relative-path","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.0.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"},{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"},{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// use tg_package_integrity; // namespace candidate
//   package identities: [{"name":"tg-package-integrity","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: serde_json::de::Deserializer::parse_exponent<serde_json::read::SliceRead>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1220,"record_offset":5784,"rva":1431696,"va":5370140816}
// 编译实例 RVAs: 0x15d890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68ef0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46183525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370140816,"raw_hex":"e865b4c002","size":5,"soft_fail":false,"target_va":5416324848,"va":5370141318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d9f040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46405336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370140816,"raw_hex":"e8d816c402","size":5,"soft_fail":false,"target_va":5416546368,"va":5370141027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$2::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":1220,"record_offset":6372,"parent_record_offset":5784,"ancestor_record_offset":5784,"inlinee":7635,"source_line":1894,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$2::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":1220,"record_offset":6400,"parent_record_offset":5784,"ancestor_record_offset":5784,"inlinee":7636,"source_line":1936,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":6052,"parent_record_offset":5784,"ancestor_record_offset":5784,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::f64_from_parts
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":6428,"parent_record_offset":5784,"ancestor_record_offset":5784,"inlinee":7627,"source_line":639,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::next_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":6156,"parent_record_offset":5784,"ancestor_record_offset":5784,"inlinee":7587,"source_line":231,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::peek_or_null
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":5964,"parent_record_offset":5784,"ancestor_record_offset":5784,"inlinee":7625,"source_line":223,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_infinite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1220,"record_offset":6592,"parent_record_offset":6428,"ancestor_record_offset":5784,"inlinee":7631,"source_line":625,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::wrapping_abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":1220,"record_offset":6496,"parent_record_offset":6428,"ancestor_record_offset":5784,"inlinee":7629,"source_line":2397,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1220,"record_offset":6552,"parent_record_offset":6496,"ancestor_record_offset":5784,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":5988,"parent_record_offset":5964,"ancestor_record_offset":5784,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1220,"record_offset":6072,"parent_record_offset":6052,"ancestor_record_offset":5784,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::next
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1220,"record_offset":6180,"parent_record_offset":6156,"ancestor_record_offset":5784,"inlinee":7588,"source_line":545,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1220,"record_offset":6012,"parent_record_offset":5988,"ancestor_record_offset":5784,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/01220.jsonl.gz"}
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
// 首条位置: {"module":1220,"record_offset":3064,"rva":14163936,"va":5382873056}
// 编译实例 RVAs: 0xd81fe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68840
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33449980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382873056,"raw_hex":"e8fc67fe01","size":5,"soft_fail":false,"target_va":5416323136,"va":5382873151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33451354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382873056,"raw_hex":"e85a6dfe01","size":5,"soft_fail":false,"target_va":5416324848,"va":5382873489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d9ed40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33672319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382873056,"raw_hex":"e87fcc0102","size":5,"soft_fail":false,"target_va":5416545600,"va":5382873276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t-1018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382873056,"raw_hex":"e806fcffff","size":5,"soft_fail":false,"target_va":5382872496,"va":5382873509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::de::Deserializer::parse_exponent<serde_json::read::SliceRead>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t-12732545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382873056,"raw_hex":"e87fb73dff","size":5,"soft_fail":false,"target_va":5370140816,"va":5382873356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":3476,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::next_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":3232,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":7587,"source_line":231,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::parse_number
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":3532,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":7872,"source_line":509,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::peek_or_null
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":3288,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":7625,"source_line":223,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":3312,"parent_record_offset":3288,"ancestor_record_offset":3064,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek_or_null
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":3592,"parent_record_offset":3532,"ancestor_record_offset":3064,"inlinee":7625,"source_line":223,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1220,"record_offset":3500,"parent_record_offset":3476,"ancestor_record_offset":3064,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::next
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1220,"record_offset":3252,"parent_record_offset":3232,"ancestor_record_offset":3064,"inlinee":7588,"source_line":545,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1220,"record_offset":3344,"parent_record_offset":3312,"ancestor_record_offset":3064,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_json::de::impl$7::deserialize_seq<serde_json::read::SliceRead,serde_core::de::impls::impl$19::deserialize::VecVisitor<tuple$<alloc::string::String,alloc::string::String> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1220,"record_offset":19200,"rva":18205344,"va":5386914464}
// 编译实例 RVAs: 0x115caa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_core..de..impls..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$alloc..vec..Vec$LT$T$GT$$GT$..deserialize..VecVisitor$LT$T$GT$$u20$as$u20$serde_core..de..Visitor$GT$::visit_seq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t17874375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386914464,"raw_hex":"e8c7bd1001","size":5,"soft_fail":false,"target_va":5404789008,"va":5386914628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-11236072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386914464,"raw_hex":"e8188d54ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5386914771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17576069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386914464,"raw_hex":"e87bcff3fe","size":5,"soft_fail":false,"target_va":5369338848,"va":5386914912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14015ddc0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16772500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386914464,"raw_hex":"e86c1200ff","size":5,"soft_fail":false,"target_va":5370142144,"va":5386914639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa03d0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1820946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386914464,"raw_hex":"e8ee36e4ff","size":5,"soft_fail":false,"target_va":5385094096,"va":5386915037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68840
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29408546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386914464,"raw_hex":"e822bdc001","size":5,"soft_fail":false,"target_va":5416323136,"va":5386914585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d70150
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29439179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386914464,"raw_hex":"e8cb34c101","size":5,"soft_fail":false,"target_va":5416354128,"va":5386914944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142db88a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29735922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386914464,"raw_hex":"e8f2bbc501","size":5,"soft_fail":false,"target_va":5416650912,"va":5386914985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1220,"record_offset":19708,"parent_record_offset":19200,"ancestor_record_offset":19200,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":19652,"parent_record_offset":19200,"ancestor_record_offset":19200,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::fix_position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":21304,"parent_record_offset":19200,"ancestor_record_offset":19200,"inlinee":7591,"source_line":441,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":19476,"parent_record_offset":19200,"ancestor_record_offset":19200,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1220,"record_offset":19912,"parent_record_offset":19888,"ancestor_record_offset":19200,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1220,"record_offset":19936,"parent_record_offset":19912,"ancestor_record_offset":19200,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1220,"record_offset":19960,"parent_record_offset":19936,"ancestor_record_offset":19200,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1220,"record_offset":19888,"parent_record_offset":19856,"ancestor_record_offset":19200,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1220,"record_offset":19856,"parent_record_offset":19828,"ancestor_record_offset":19200,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1220,"record_offset":20556,"parent_record_offset":20524,"ancestor_record_offset":19200,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1220,"record_offset":20524,"parent_record_offset":20496,"ancestor_record_offset":19200,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1220,"record_offset":20496,"parent_record_offset":20468,"ancestor_record_offset":19200,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1220,"record_offset":20328,"parent_record_offset":20304,"ancestor_record_offset":19200,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1220,"record_offset":21144,"parent_record_offset":21108,"ancestor_record_offset":19200,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1220,"record_offset":19740,"parent_record_offset":19708,"ancestor_record_offset":19200,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":19592,"parent_record_offset":19476,"ancestor_record_offset":19200,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":19524,"parent_record_offset":19476,"ancestor_record_offset":19200,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1220,"record_offset":19616,"parent_record_offset":19592,"ancestor_record_offset":19200,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1220,"record_offset":19552,"parent_record_offset":19524,"ancestor_record_offset":19200,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tg_package_integrity::_::impl$0::deserialize::__Visitor>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1220,"record_offset":22492,"rva":18206352,"va":5386915472}
// 编译实例 RVAs: 0x115ce90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..read..SliceRead$u20$as$u20$serde_json..read..Read$GT$::ignore_str
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-3662392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386915472,"raw_hex":"e8c81dc8ff","size":5,"soft_fail":false,"target_va":5383254288,"va":5386916675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$serde_json..read..SliceRead$u20$as$u20$serde_json..read..Read$GT$::parse_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3661060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386915472,"raw_hex":"e8fc22c8ff","size":5,"soft_fail":false,"target_va":5383254800,"va":5386915855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t-11238981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386915472,"raw_hex":"e8bb8154ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5386917680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f920
//     owner=unresolved_namespace; count=2
//     target_aliases=16 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17685725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386915472,"raw_hex":"e82323f2fe","size":5,"soft_fail":false,"target_va":5369231648,"va":5386917368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17579092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386915472,"raw_hex":"e8acc3f3fe","size":5,"soft_fail":false,"target_va":5369338848,"va":5386917935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14015d610
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16776063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386915472,"raw_hex":"e8810400ff","size":5,"soft_fail":false,"target_va":5370140176,"va":5386916234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14015ddc0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16775460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386915472,"raw_hex":"e8dc0600ff","size":5,"soft_fail":false,"target_va":5370142144,"va":5386917599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140160ee0
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16762785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386915472,"raw_hex":"e85f3800ff","size":5,"soft_fail":false,"target_va":5370154720,"va":5386917500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa03d0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1825700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386915472,"raw_hex":"e85c24e4ff","size":5,"soft_fail":false,"target_va":5385094096,"va":5386919791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14115caa0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386915472,"raw_hex":"e8d5f4ffff","size":5,"soft_fail":false,"target_va":5386914464,"va":5386917318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f2300
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4804918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386915472,"raw_hex":"e836514900","size":5,"soft_fail":false,"target_va":5391721216,"va":5386916293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68840
//     owner=unresolved_namespace; count=24
//     target_aliases=11 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29407526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386915472,"raw_hex":"e826b9c001","size":5,"soft_fail":false,"target_va":5416323136,"va":5386915605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68ef0
//     owner=unresolved_namespace; count=6
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29406592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386915472,"raw_hex":"e880b5c001","size":5,"soft_fail":false,"target_va":5416324848,"va":5386918251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d70150
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29435193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386915472,"raw_hex":"e83925c101","size":5,"soft_fail":false,"target_va":5416354128,"va":5386918930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d706e0
//     owner=unresolved_namespace; count=4
//     target_aliases=6 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29437767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386915472,"raw_hex":"e8472fc101","size":5,"soft_fail":false,"target_va":5416355552,"va":5386917780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142db88a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29731991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386915472,"raw_hex":"e897acc501","size":5,"soft_fail":false,"target_va":5416650912,"va":5386918916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29702705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386915472,"raw_hex":"e8313ac501","size":5,"soft_fail":false,"target_va":5416622208,"va":5386919498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_core::de::Error::invalid_length
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30035080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386915472,"raw_hex":"e8884cca01","size":5,"soft_fail":false,"target_va":5416953088,"va":5386918003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_core::de::impl$5::deserialize<u32,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-3173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386915472,"raw_hex":"e89bf3ffff","size":5,"soft_fail":false,"target_va":5386914096,"va":5386917264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1220,"record_offset":29608,"parent_record_offset":22492,"ancestor_record_offset":22492,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":22904,"parent_record_offset":22492,"ancestor_record_offset":22492,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::end_map
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":30676,"parent_record_offset":22492,"ancestor_record_offset":22492,"inlinee":7878,"source_line":1090,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::fix_position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":32700,"parent_record_offset":22492,"ancestor_record_offset":22492,"inlinee":7591,"source_line":441,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":22728,"parent_record_offset":22492,"ancestor_record_offset":22492,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] tg_package_integrity::_::impl$0::deserialize::impl$2::visit_map
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":1220,"record_offset":22960,"parent_record_offset":22492,"ancestor_record_offset":22492,"inlinee":69916,"source_line":28,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] tg_package_integrity::_::impl$0::deserialize::impl$2::visit_seq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":1220,"record_offset":29344,"parent_record_offset":22492,"ancestor_record_offset":22492,"inlinee":69920,"source_line":28,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1220,"record_offset":28560,"parent_record_offset":28536,"ancestor_record_offset":22492,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1220,"record_offset":28584,"parent_record_offset":28560,"ancestor_record_offset":22492,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1220,"record_offset":28608,"parent_record_offset":28584,"ancestor_record_offset":22492,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1220,"record_offset":28536,"parent_record_offset":28472,"ancestor_record_offset":22492,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1220,"record_offset":30488,"parent_record_offset":30464,"ancestor_record_offset":22492,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1220,"record_offset":26472,"parent_record_offset":26440,"ancestor_record_offset":22492,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1220,"record_offset":28504,"parent_record_offset":28472,"ancestor_record_offset":22492,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1220,"record_offset":28472,"parent_record_offset":28444,"ancestor_record_offset":22492,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1220,"record_offset":26520,"parent_record_offset":26496,"ancestor_record_offset":22492,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1220,"record_offset":26496,"parent_record_offset":26472,"ancestor_record_offset":22492,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1220,"record_offset":28444,"parent_record_offset":28416,"ancestor_record_offset":22492,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1220,"record_offset":26440,"parent_record_offset":26408,"ancestor_record_offset":22492,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1220,"record_offset":23828,"parent_record_offset":23724,"ancestor_record_offset":22492,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1220,"record_offset":26408,"parent_record_offset":25100,"ancestor_record_offset":22492,"inlinee":5450,"source_line":2790,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1220,"record_offset":25608,"parent_record_offset":25100,"ancestor_record_offset":22492,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1220,"record_offset":28284,"parent_record_offset":28260,"ancestor_record_offset":22492,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1220,"record_offset":25632,"parent_record_offset":25608,"ancestor_record_offset":22492,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1220,"record_offset":23912,"parent_record_offset":23888,"ancestor_record_offset":22492,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1220,"record_offset":29068,"parent_record_offset":29040,"ancestor_record_offset":22492,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1220,"record_offset":24144,"parent_record_offset":22960,"ancestor_record_offset":22492,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1220,"record_offset":27952,"parent_record_offset":22960,"ancestor_record_offset":22492,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1220,"record_offset":26844,"parent_record_offset":26812,"ancestor_record_offset":22492,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1220,"record_offset":27884,"parent_record_offset":22960,"ancestor_record_offset":22492,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1220,"record_offset":23936,"parent_record_offset":23912,"ancestor_record_offset":22492,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1220,"record_offset":23964,"parent_record_offset":23936,"ancestor_record_offset":22492,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1220,"record_offset":23888,"parent_record_offset":23856,"ancestor_record_offset":22492,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1220,"record_offset":27980,"parent_record_offset":22960,"ancestor_record_offset":22492,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1220,"record_offset":28052,"parent_record_offset":28028,"ancestor_record_offset":22492,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1220,"record_offset":28028,"parent_record_offset":28004,"ancestor_record_offset":22492,"inlinee":7805,"source_line":125,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1220,"record_offset":28168,"parent_record_offset":28144,"ancestor_record_offset":22492,"inlinee":35640,"source_line":125,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1220,"record_offset":29228,"parent_record_offset":22960,"ancestor_record_offset":22492,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1220,"record_offset":28076,"parent_record_offset":28052,"ancestor_record_offset":22492,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1220,"record_offset":23024,"parent_record_offset":22960,"ancestor_record_offset":22492,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1220,"record_offset":24176,"parent_record_offset":22960,"ancestor_record_offset":22492,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1220,"record_offset":29380,"parent_record_offset":29344,"ancestor_record_offset":22492,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1220,"record_offset":24996,"parent_record_offset":24944,"ancestor_record_offset":22492,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1220,"record_offset":23640,"parent_record_offset":23072,"ancestor_record_offset":22492,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1220,"record_offset":27844,"parent_record_offset":27820,"ancestor_record_offset":22492,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$70::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1220,"record_offset":28144,"parent_record_offset":28120,"ancestor_record_offset":22492,"inlinee":9914,"source_line":132,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=27
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":22844,"parent_record_offset":22728,"ancestor_record_offset":22492,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::ignore_value
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":25100,"parent_record_offset":25048,"ancestor_record_offset":22492,"inlinee":7603,"source_line":1102,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::next_char
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":25844,"parent_record_offset":25776,"ancestor_record_offset":22492,"inlinee":7587,"source_line":231,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_ident
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":25776,"parent_record_offset":25100,"ancestor_record_offset":22492,"inlinee":7586,"source_line":445,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":24260,"parent_record_offset":24216,"ancestor_record_offset":22492,"inlinee":7597,"source_line":1061,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":23208,"parent_record_offset":23128,"ancestor_record_offset":22492,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":22776,"parent_record_offset":22728,"ancestor_record_offset":22492,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":23072,"parent_record_offset":23024,"ancestor_record_offset":22492,"inlinee":7724,"source_line":1986,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed::has_next_key
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":23128,"parent_record_offset":23072,"ancestor_record_offset":22492,"inlinee":7882,"source_line":1990,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":24216,"parent_record_offset":24176,"ancestor_record_offset":22492,"inlinee":7596,"source_line":2027,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":23724,"parent_record_offset":23696,"ancestor_record_offset":22492,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1220,"record_offset":23696,"parent_record_offset":23668,"ancestor_record_offset":22492,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$7::deserialize_ignored_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":25048,"parent_record_offset":24996,"ancestor_record_offset":22492,"inlinee":7602,"source_line":1910,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":29404,"parent_record_offset":29380,"ancestor_record_offset":22492,"inlinee":7609,"source_line":1933,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=29
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1220,"record_offset":22868,"parent_record_offset":22844,"ancestor_record_offset":22492,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::next
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1220,"record_offset":25880,"parent_record_offset":25844,"ancestor_record_offset":22492,"inlinee":7588,"source_line":545,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1220,"record_offset":22804,"parent_record_offset":22776,"ancestor_record_offset":22492,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] tg_package_integrity::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":1220,"record_offset":23856,"parent_record_offset":23724,"ancestor_record_offset":22492,"inlinee":69919,"source_line":28,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] tg_package_integrity::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":1220,"record_offset":23668,"parent_record_offset":23640,"ancestor_record_offset":22492,"inlinee":69918,"source_line":28,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::verify_package_manifest
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":341,"record_offset":1700,"rva":7458656,"va":5376167776}
// 编译实例 RVAs: 0x71cf60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t8627024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e850a38300","size":5,"soft_fail":false,"target_va":5384795984,"va":5376168955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8476421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e805578100","size":5,"soft_fail":false,"target_va":5384644912,"va":5376168486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::io::error::Error as core::fmt::Display>::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8598181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e8a5328300","size":5,"soft_fail":false,"target_va":5384766224,"va":5376168038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8488366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e8ae858100","size":5,"soft_fail":false,"target_va":5384656240,"va":5376167869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$blake3..Hash$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8944052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e8b4798800","size":5,"soft_fail":false,"target_va":5385113168,"va":5376169111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7074348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e82cf26b00","size":5,"soft_fail":false,"target_va":5383242672,"va":5376168319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=25
//     evidence={"assembly":"\tcallq\t-489234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e8ee88f8ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5376167933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f920
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6939649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e8ff1b96ff","size":5,"soft_fail":false,"target_va":5369231648,"va":5376171292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=5
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6829222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e85acb97ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5376168065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ae10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6827618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e89ed197ff","size":5,"soft_fail":false,"target_va":5369343504,"va":5376171117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b0540
//     owner=unresolved_namespace; count=6
//     target_aliases=26 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6738931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e80d2c99ff","size":5,"soft_fail":false,"target_va":5369431360,"va":5376170286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098f4c0
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2562925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e86d1b2700","size":5,"soft_fail":false,"target_va":5378733248,"va":5376170318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14115c3d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10744505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e8b9f2a300","size":5,"soft_fail":false,"target_va":5386912720,"va":5376168210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14115ec30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10753337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e83915a400","size":5,"soft_fail":false,"target_va":5386923056,"va":5376169714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40181598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e85e1f6502","size":5,"soft_fail":false,"target_va":5416350944,"va":5376169341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40887867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e83be66f02","size":5,"soft_fail":false,"target_va":5417057040,"va":5376169168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8948638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e89e8b8800","size":5,"soft_fail":false,"target_va":5385116768,"va":5376168125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::array::_$LT$impl$u20$core..hash..Hash$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$::hash
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10744124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e83cf1a300","size":5,"soft_fail":false,"target_va":5386912288,"va":5376168159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t40451151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e84f3c6902","size":5,"soft_fail":false,"target_va":5416621632,"va":5376170476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::map::HashMap$LT$K$C$V$C$S$C$A$GT$::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10755444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e8741da400","size":5,"soft_fail":false,"target_va":5386926080,"va":5376170631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTable$LT$T$C$A$GT$::reserve_rehash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t40481112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e858b16902","size":5,"soft_fail":false,"target_va":5416651696,"va":5376170579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8584029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e85dfb8200","size":5,"soft_fail":false,"target_va":5384751936,"va":5376167902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::stdio::_eprint
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8552524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e84c808200","size":5,"soft_fail":false,"target_va":5384721472,"va":5376168943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8556147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376167776,"raw_hex":"e8738e8200","size":5,"soft_fail":false,"target_va":5384726688,"va":5376170536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":341,"record_offset":10940,"parent_record_offset":1700,"ancestor_record_offset":1700,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":341,"record_offset":5892,"parent_record_offset":1700,"ancestor_record_offset":1700,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":341,"record_offset":9092,"parent_record_offset":1700,"ancestor_record_offset":1700,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":341,"record_offset":6032,"parent_record_offset":1700,"ancestor_record_offset":1700,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":341,"record_offset":4560,"parent_record_offset":1700,"ancestor_record_offset":1700,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":341,"record_offset":10132,"parent_record_offset":1700,"ancestor_record_offset":1700,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":341,"record_offset":2688,"parent_record_offset":1700,"ancestor_record_offset":1700,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":341,"record_offset":12544,"parent_record_offset":1700,"ancestor_record_offset":1700,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":341,"record_offset":3248,"parent_record_offset":1700,"ancestor_record_offset":1700,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":341,"record_offset":5948,"parent_record_offset":1700,"ancestor_record_offset":1700,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_direct] relative_path::relative_path_buf::RelativePathBuf::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/relative_path_buf.rs
//     evidence={"module":341,"record_offset":10912,"parent_record_offset":1700,"ancestor_record_offset":1700,"inlinee":21553,"source_line":61,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":341,"record_offset":1912,"parent_record_offset":1700,"ancestor_record_offset":1700,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":341,"record_offset":1884,"parent_record_offset":1700,"ancestor_record_offset":1700,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_direct] tg_package_integrity::is_valid_manifest_checksum
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":341,"record_offset":3276,"parent_record_offset":1700,"ancestor_record_offset":1700,"inlinee":21542,"source_line":41,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":341,"record_offset":2196,"parent_record_offset":2172,"ancestor_record_offset":1700,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":341,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":1700,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":341,"record_offset":2244,"parent_record_offset":2220,"ancestor_record_offset":1700,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":341,"record_offset":2172,"parent_record_offset":2116,"ancestor_record_offset":1700,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":341,"record_offset":4104,"parent_record_offset":4076,"ancestor_record_offset":1700,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":341,"record_offset":9204,"parent_record_offset":9172,"ancestor_record_offset":1700,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":341,"record_offset":2144,"parent_record_offset":2116,"ancestor_record_offset":1700,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":341,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":1700,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":341,"record_offset":9252,"parent_record_offset":9228,"ancestor_record_offset":1700,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":341,"record_offset":9228,"parent_record_offset":9204,"ancestor_record_offset":1700,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":341,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":1700,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":341,"record_offset":2856,"parent_record_offset":2804,"ancestor_record_offset":1700,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":341,"record_offset":2768,"parent_record_offset":2736,"ancestor_record_offset":1700,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":341,"record_offset":2804,"parent_record_offset":2768,"ancestor_record_offset":1700,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":341,"record_offset":7104,"parent_record_offset":7076,"ancestor_record_offset":1700,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$91::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":341,"record_offset":6372,"parent_record_offset":6348,"ancestor_record_offset":1700,"inlinee":21551,"source_line":2666,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":341,"record_offset":9172,"parent_record_offset":9128,"ancestor_record_offset":1700,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":341,"record_offset":9128,"parent_record_offset":9092,"ancestor_record_offset":1700,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":341,"record_offset":10188,"parent_record_offset":10160,"ancestor_record_offset":1700,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":341,"record_offset":7132,"parent_record_offset":7104,"ancestor_record_offset":1700,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] blake3::impl$23::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":341,"record_offset":3332,"parent_record_offset":3304,"ancestor_record_offset":1700,"inlinee":21546,"source_line":221,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":341,"record_offset":5168,"parent_record_offset":5140,"ancestor_record_offset":1700,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":341,"record_offset":5068,"parent_record_offset":4864,"ancestor_record_offset":1700,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":341,"record_offset":7076,"parent_record_offset":6248,"ancestor_record_offset":1700,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":341,"record_offset":6348,"parent_record_offset":6248,"ancestor_record_offset":1700,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":341,"record_offset":6572,"parent_record_offset":6548,"ancestor_record_offset":1700,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":341,"record_offset":11364,"parent_record_offset":11340,"ancestor_record_offset":1700,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":341,"record_offset":2884,"parent_record_offset":2804,"ancestor_record_offset":1700,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":341,"record_offset":5520,"parent_record_offset":5484,"ancestor_record_offset":1700,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":341,"record_offset":5580,"parent_record_offset":5556,"ancestor_record_offset":1700,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":341,"record_offset":5688,"parent_record_offset":5628,"ancestor_record_offset":1700,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":341,"record_offset":5556,"parent_record_offset":5408,"ancestor_record_offset":1700,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":341,"record_offset":5764,"parent_record_offset":5740,"ancestor_record_offset":1700,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":341,"record_offset":5032,"parent_record_offset":5008,"ancestor_record_offset":1700,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":341,"record_offset":12292,"parent_record_offset":12264,"ancestor_record_offset":1700,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":341,"record_offset":12320,"parent_record_offset":12292,"ancestor_record_offset":1700,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":341,"record_offset":5828,"parent_record_offset":5628,"ancestor_record_offset":1700,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":341,"record_offset":13460,"parent_record_offset":13432,"ancestor_record_offset":1700,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::is_multiple_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":341,"record_offset":3364,"parent_record_offset":3276,"ancestor_record_offset":1700,"inlinee":21547,"source_line":3742,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":341,"record_offset":5196,"parent_record_offset":5140,"ancestor_record_offset":1700,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":341,"record_offset":11584,"parent_record_offset":11560,"ancestor_record_offset":1700,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":341,"record_offset":6208,"parent_record_offset":6168,"ancestor_record_offset":1700,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":341,"record_offset":4776,"parent_record_offset":4740,"ancestor_record_offset":1700,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":341,"record_offset":6168,"parent_record_offset":6128,"ancestor_record_offset":1700,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":341,"record_offset":6128,"parent_record_offset":6032,"ancestor_record_offset":1700,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":341,"record_offset":4896,"parent_record_offset":4864,"ancestor_record_offset":1700,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":341,"record_offset":11396,"parent_record_offset":11312,"ancestor_record_offset":1700,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=83
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":341,"record_offset":1948,"parent_record_offset":1912,"ancestor_record_offset":1700,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":341,"record_offset":5996,"parent_record_offset":5972,"ancestor_record_offset":1700,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":341,"record_offset":11852,"parent_record_offset":11828,"ancestor_record_offset":1700,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":341,"record_offset":4984,"parent_record_offset":4864,"ancestor_record_offset":1700,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":341,"record_offset":11704,"parent_record_offset":11680,"ancestor_record_offset":1700,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":341,"record_offset":5660,"parent_record_offset":5628,"ancestor_record_offset":1700,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":341,"record_offset":6096,"parent_record_offset":6072,"ancestor_record_offset":1700,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":341,"record_offset":5008,"parent_record_offset":4984,"ancestor_record_offset":1700,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":341,"record_offset":9292,"parent_record_offset":9128,"ancestor_record_offset":1700,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":341,"record_offset":2912,"parent_record_offset":2804,"ancestor_record_offset":1700,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":341,"record_offset":6444,"parent_record_offset":6420,"ancestor_record_offset":1700,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":341,"record_offset":6472,"parent_record_offset":6444,"ancestor_record_offset":1700,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":341,"record_offset":5972,"parent_record_offset":5948,"ancestor_record_offset":1700,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":341,"record_offset":5628,"parent_record_offset":5604,"ancestor_record_offset":1700,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":341,"record_offset":6072,"parent_record_offset":6032,"ancestor_record_offset":1700,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":341,"record_offset":6396,"parent_record_offset":6372,"ancestor_record_offset":1700,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":341,"record_offset":11528,"parent_record_offset":11500,"ancestor_record_offset":1700,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":341,"record_offset":11560,"parent_record_offset":11528,"ancestor_record_offset":1700,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":341,"record_offset":11500,"parent_record_offset":11460,"ancestor_record_offset":1700,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":341,"record_offset":11340,"parent_record_offset":11312,"ancestor_record_offset":1700,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":341,"record_offset":5448,"parent_record_offset":5408,"ancestor_record_offset":1700,"inlinee":8392,"source_line":1103,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":341,"record_offset":5408,"parent_record_offset":5372,"ancestor_record_offset":1700,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":341,"record_offset":5788,"parent_record_offset":5764,"ancestor_record_offset":1700,"inlinee":12038,"source_line":4661,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":341,"record_offset":11828,"parent_record_offset":11224,"ancestor_record_offset":1700,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":341,"record_offset":11680,"parent_record_offset":11460,"ancestor_record_offset":1700,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":341,"record_offset":11312,"parent_record_offset":11256,"ancestor_record_offset":1700,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":341,"record_offset":11460,"parent_record_offset":11432,"ancestor_record_offset":1700,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":341,"record_offset":5484,"parent_record_offset":5448,"ancestor_record_offset":1700,"inlinee":8393,"source_line":955,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":341,"record_offset":12232,"parent_record_offset":12200,"ancestor_record_offset":1700,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":341,"record_offset":11284,"parent_record_offset":11256,"ancestor_record_offset":1700,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":341,"record_offset":11224,"parent_record_offset":11164,"ancestor_record_offset":1700,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":341,"record_offset":11164,"parent_record_offset":11136,"ancestor_record_offset":1700,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":341,"record_offset":12200,"parent_record_offset":11164,"ancestor_record_offset":1700,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":341,"record_offset":11196,"parent_record_offset":11164,"ancestor_record_offset":1700,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":341,"record_offset":11256,"parent_record_offset":11224,"ancestor_record_offset":1700,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":341,"record_offset":12264,"parent_record_offset":12232,"ancestor_record_offset":1700,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":341,"record_offset":11136,"parent_record_offset":11108,"ancestor_record_offset":1700,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":341,"record_offset":11432,"parent_record_offset":11224,"ancestor_record_offset":1700,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":341,"record_offset":5372,"parent_record_offset":5336,"ancestor_record_offset":1700,"inlinee":12034,"source_line":1307,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":341,"record_offset":5308,"parent_record_offset":4600,"ancestor_record_offset":1700,"inlinee":11864,"source_line":234,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":341,"record_offset":5336,"parent_record_offset":4600,"ancestor_record_offset":1700,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":341,"record_offset":4600,"parent_record_offset":4560,"ancestor_record_offset":1700,"inlinee":11863,"source_line":1149,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":341,"record_offset":6548,"parent_record_offset":6248,"ancestor_record_offset":1700,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":341,"record_offset":4684,"parent_record_offset":4656,"ancestor_record_offset":1700,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":341,"record_offset":4944,"parent_record_offset":4920,"ancestor_record_offset":1700,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":341,"record_offset":4804,"parent_record_offset":4776,"ancestor_record_offset":1700,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":341,"record_offset":5140,"parent_record_offset":4740,"ancestor_record_offset":1700,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":341,"record_offset":4656,"parent_record_offset":4600,"ancestor_record_offset":1700,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":341,"record_offset":6520,"parent_record_offset":6248,"ancestor_record_offset":1700,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":341,"record_offset":4832,"parent_record_offset":4804,"ancestor_record_offset":1700,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":341,"record_offset":4864,"parent_record_offset":4832,"ancestor_record_offset":1700,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":341,"record_offset":4740,"parent_record_offset":4712,"ancestor_record_offset":1700,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":341,"record_offset":4712,"parent_record_offset":4684,"ancestor_record_offset":1700,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] tg_package_integrity::check_manifest_file
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":341,"record_offset":6248,"parent_record_offset":6208,"ancestor_record_offset":1700,"inlinee":21549,"source_line":109,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] tg_package_integrity::is_valid_manifest_checksum::calculate_fnv_hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":341,"record_offset":3304,"parent_record_offset":3276,"ancestor_record_offset":1700,"inlinee":21544,"source_line":42,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] tg_package_integrity::verify_package_manifest::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":341,"record_offset":2736,"parent_record_offset":2688,"ancestor_record_offset":1700,"inlinee":21540,"source_line":56,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] tg_package_integrity::verify_package_manifest::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":341,"record_offset":3444,"parent_record_offset":3396,"ancestor_record_offset":1700,"inlinee":21548,"source_line":65,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] tg_package_integrity::verify_package_manifest::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":341,"record_offset":5712,"parent_record_offset":5688,"ancestor_record_offset":1700,"inlinee":21554,"source_line":74,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1111B
// 首条位置: {"module":1220,"inlinee":69915}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1111F
// 首条位置: {"module":1220,"inlinee":69919}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1220,"record_offset":23888,"parent_record_offset":23856,"ancestor_record_offset":23856,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1220,"record_offset":23912,"parent_record_offset":23888,"ancestor_record_offset":23856,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1220,"record_offset":23936,"parent_record_offset":23912,"ancestor_record_offset":23856,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1220,"record_offset":23964,"parent_record_offset":23936,"ancestor_record_offset":23856,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1111E
// 首条位置: {"module":1220,"inlinee":69918}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1220,"record_offset":23696,"parent_record_offset":23668,"ancestor_record_offset":23668,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1220,"record_offset":23828,"parent_record_offset":23724,"ancestor_record_offset":23668,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1220,"record_offset":23912,"parent_record_offset":23888,"ancestor_record_offset":23668,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1220,"record_offset":23936,"parent_record_offset":23912,"ancestor_record_offset":23668,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1220,"record_offset":23964,"parent_record_offset":23936,"ancestor_record_offset":23668,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1220,"record_offset":23888,"parent_record_offset":23856,"ancestor_record_offset":23668,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":23772,"parent_record_offset":23724,"ancestor_record_offset":23668,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":23724,"parent_record_offset":23696,"ancestor_record_offset":23668,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1220,"record_offset":23796,"parent_record_offset":23772,"ancestor_record_offset":23668,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] tg_package_integrity::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":1220,"record_offset":23856,"parent_record_offset":23724,"ancestor_record_offset":23668,"inlinee":69919,"source_line":28,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5434
// 首条位置: {"module":1222,"inlinee":21556}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1111C
// 首条位置: {"module":1220,"inlinee":69916}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1220,"record_offset":24144,"parent_record_offset":22960,"ancestor_record_offset":22960,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1220,"record_offset":27952,"parent_record_offset":22960,"ancestor_record_offset":22960,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1220,"record_offset":27884,"parent_record_offset":22960,"ancestor_record_offset":22960,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1220,"record_offset":27980,"parent_record_offset":22960,"ancestor_record_offset":22960,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1220,"record_offset":29228,"parent_record_offset":22960,"ancestor_record_offset":22960,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1220,"record_offset":23024,"parent_record_offset":22960,"ancestor_record_offset":22960,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1220,"record_offset":24176,"parent_record_offset":22960,"ancestor_record_offset":22960,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1220,"record_offset":28560,"parent_record_offset":28536,"ancestor_record_offset":22960,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1220,"record_offset":28584,"parent_record_offset":28560,"ancestor_record_offset":22960,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1220,"record_offset":28608,"parent_record_offset":28584,"ancestor_record_offset":22960,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1220,"record_offset":28536,"parent_record_offset":28472,"ancestor_record_offset":22960,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1220,"record_offset":26472,"parent_record_offset":26440,"ancestor_record_offset":22960,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1220,"record_offset":28504,"parent_record_offset":28472,"ancestor_record_offset":22960,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1220,"record_offset":28472,"parent_record_offset":28444,"ancestor_record_offset":22960,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1220,"record_offset":26520,"parent_record_offset":26496,"ancestor_record_offset":22960,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1220,"record_offset":26496,"parent_record_offset":26472,"ancestor_record_offset":22960,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1220,"record_offset":28444,"parent_record_offset":28416,"ancestor_record_offset":22960,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1220,"record_offset":26440,"parent_record_offset":26408,"ancestor_record_offset":22960,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1220,"record_offset":23828,"parent_record_offset":23724,"ancestor_record_offset":22960,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1220,"record_offset":26408,"parent_record_offset":25100,"ancestor_record_offset":22960,"inlinee":5450,"source_line":2790,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1220,"record_offset":25608,"parent_record_offset":25100,"ancestor_record_offset":22960,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1220,"record_offset":28284,"parent_record_offset":28260,"ancestor_record_offset":22960,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1220,"record_offset":25632,"parent_record_offset":25608,"ancestor_record_offset":22960,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1220,"record_offset":23912,"parent_record_offset":23888,"ancestor_record_offset":22960,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1220,"record_offset":29068,"parent_record_offset":29040,"ancestor_record_offset":22960,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1220,"record_offset":28308,"parent_record_offset":28284,"ancestor_record_offset":22960,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1220,"record_offset":26844,"parent_record_offset":26812,"ancestor_record_offset":22960,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1220,"record_offset":23936,"parent_record_offset":23912,"ancestor_record_offset":22960,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1220,"record_offset":23964,"parent_record_offset":23936,"ancestor_record_offset":22960,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1220,"record_offset":23888,"parent_record_offset":23856,"ancestor_record_offset":22960,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1220,"record_offset":28052,"parent_record_offset":28028,"ancestor_record_offset":22960,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1220,"record_offset":28028,"parent_record_offset":28004,"ancestor_record_offset":22960,"inlinee":7805,"source_line":125,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1220,"record_offset":28168,"parent_record_offset":28144,"ancestor_record_offset":22960,"inlinee":35640,"source_line":125,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1220,"record_offset":28076,"parent_record_offset":28052,"ancestor_record_offset":22960,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1220,"record_offset":24996,"parent_record_offset":24944,"ancestor_record_offset":22960,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1220,"record_offset":23640,"parent_record_offset":23072,"ancestor_record_offset":22960,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1220,"record_offset":27844,"parent_record_offset":27820,"ancestor_record_offset":22960,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$70::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1220,"record_offset":28144,"parent_record_offset":28120,"ancestor_record_offset":22960,"inlinee":9914,"source_line":132,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=24
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":23340,"parent_record_offset":23208,"ancestor_record_offset":22960,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::ignore_value
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":25100,"parent_record_offset":25048,"ancestor_record_offset":22960,"inlinee":7603,"source_line":1102,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::next_char
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":25844,"parent_record_offset":25776,"ancestor_record_offset":22960,"inlinee":7587,"source_line":231,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_ident
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":25776,"parent_record_offset":25100,"ancestor_record_offset":22960,"inlinee":7586,"source_line":445,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":24260,"parent_record_offset":24216,"ancestor_record_offset":22960,"inlinee":7597,"source_line":1061,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":23208,"parent_record_offset":23128,"ancestor_record_offset":22960,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":23264,"parent_record_offset":23208,"ancestor_record_offset":22960,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":23072,"parent_record_offset":23024,"ancestor_record_offset":22960,"inlinee":7724,"source_line":1986,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed::has_next_key
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":23128,"parent_record_offset":23072,"ancestor_record_offset":22960,"inlinee":7882,"source_line":1990,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":24216,"parent_record_offset":24176,"ancestor_record_offset":22960,"inlinee":7596,"source_line":2027,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":23724,"parent_record_offset":23696,"ancestor_record_offset":22960,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1220,"record_offset":23696,"parent_record_offset":23668,"ancestor_record_offset":22960,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$7::deserialize_ignored_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":25048,"parent_record_offset":24996,"ancestor_record_offset":22960,"inlinee":7602,"source_line":1910,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=24
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1220,"record_offset":23364,"parent_record_offset":23340,"ancestor_record_offset":22960,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::next
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1220,"record_offset":25880,"parent_record_offset":25844,"ancestor_record_offset":22960,"inlinee":7588,"source_line":545,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1220,"record_offset":23296,"parent_record_offset":23264,"ancestor_record_offset":22960,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] tg_package_integrity::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":1220,"record_offset":23856,"parent_record_offset":23724,"ancestor_record_offset":22960,"inlinee":69919,"source_line":28,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] tg_package_integrity::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":1220,"record_offset":23668,"parent_record_offset":23640,"ancestor_record_offset":22960,"inlinee":69918,"source_line":28,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11120
// 首条位置: {"module":1220,"inlinee":69920}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1220,"record_offset":29380,"parent_record_offset":29344,"ancestor_record_offset":29344,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1220,"record_offset":29540,"parent_record_offset":29488,"ancestor_record_offset":29344,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1220,"record_offset":29564,"parent_record_offset":29540,"ancestor_record_offset":29344,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1220,"record_offset":29404,"parent_record_offset":29380,"ancestor_record_offset":29344,"inlinee":7609,"source_line":1933,"archive":"_pdb/C/modules/01220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::is_valid_manifest_checksum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5426
// 首条位置: {"module":341,"inlinee":21542}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$9::is_multiple_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":341,"record_offset":3364,"parent_record_offset":3276,"ancestor_record_offset":3276,"inlinee":21547,"source_line":3742,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_direct] tg_package_integrity::is_valid_manifest_checksum::calculate_fnv_hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":341,"record_offset":3304,"parent_record_offset":3276,"ancestor_record_offset":3276,"inlinee":21544,"source_line":42,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] blake3::impl$23::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":341,"record_offset":3332,"parent_record_offset":3304,"ancestor_record_offset":3276,"inlinee":21546,"source_line":221,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::is_valid_manifest_checksum::calculate_fnv_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5428
// 首条位置: {"module":341,"inlinee":21544}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] blake3::impl$23::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":341,"record_offset":3332,"parent_record_offset":3304,"ancestor_record_offset":3304,"inlinee":21546,"source_line":221,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::verify_package_manifest::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5424
// 首条位置: {"module":341,"inlinee":21540}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":341,"record_offset":2768,"parent_record_offset":2736,"ancestor_record_offset":2736,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":341,"record_offset":3124,"parent_record_offset":3100,"ancestor_record_offset":2736,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":341,"record_offset":3148,"parent_record_offset":3124,"ancestor_record_offset":2736,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":341,"record_offset":3172,"parent_record_offset":3148,"ancestor_record_offset":2736,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":341,"record_offset":3100,"parent_record_offset":3044,"ancestor_record_offset":2736,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":341,"record_offset":3072,"parent_record_offset":3044,"ancestor_record_offset":2736,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":341,"record_offset":3044,"parent_record_offset":3020,"ancestor_record_offset":2736,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":341,"record_offset":3020,"parent_record_offset":2996,"ancestor_record_offset":2736,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":341,"record_offset":2856,"parent_record_offset":2804,"ancestor_record_offset":2736,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":341,"record_offset":2804,"parent_record_offset":2768,"ancestor_record_offset":2736,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":341,"record_offset":2884,"parent_record_offset":2804,"ancestor_record_offset":2736,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":341,"record_offset":2948,"parent_record_offset":2804,"ancestor_record_offset":2736,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":341,"record_offset":2912,"parent_record_offset":2804,"ancestor_record_offset":2736,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::verify_package_manifest::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x542C
// 首条位置: {"module":341,"inlinee":21548}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":341,"record_offset":3484,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":341,"record_offset":3956,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":341,"record_offset":3840,"parent_record_offset":3816,"ancestor_record_offset":3444,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":341,"record_offset":3864,"parent_record_offset":3840,"ancestor_record_offset":3444,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":341,"record_offset":3888,"parent_record_offset":3864,"ancestor_record_offset":3444,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":341,"record_offset":3816,"parent_record_offset":3760,"ancestor_record_offset":3444,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":341,"record_offset":4104,"parent_record_offset":4076,"ancestor_record_offset":3444,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":341,"record_offset":3788,"parent_record_offset":3760,"ancestor_record_offset":3444,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":341,"record_offset":3760,"parent_record_offset":3736,"ancestor_record_offset":3444,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":341,"record_offset":3736,"parent_record_offset":3712,"ancestor_record_offset":3444,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":341,"record_offset":3572,"parent_record_offset":3520,"ancestor_record_offset":3444,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":341,"record_offset":3520,"parent_record_offset":3484,"ancestor_record_offset":3444,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":341,"record_offset":3600,"parent_record_offset":3520,"ancestor_record_offset":3444,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":341,"record_offset":3664,"parent_record_offset":3520,"ancestor_record_offset":3444,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":341,"record_offset":3628,"parent_record_offset":3520,"ancestor_record_offset":3444,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::verify_package_manifest::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5432
// 首条位置: {"module":341,"inlinee":21554}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::check_manifest_file
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x542D
// 首条位置: {"module":341,"inlinee":21549}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":341,"record_offset":6604,"parent_record_offset":6248,"ancestor_record_offset":6248,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":341,"record_offset":7076,"parent_record_offset":6248,"ancestor_record_offset":6248,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":341,"record_offset":6348,"parent_record_offset":6248,"ancestor_record_offset":6248,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":341,"record_offset":7232,"parent_record_offset":6248,"ancestor_record_offset":6248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":341,"record_offset":6548,"parent_record_offset":6248,"ancestor_record_offset":6248,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":341,"record_offset":6520,"parent_record_offset":6248,"ancestor_record_offset":6248,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":341,"record_offset":6960,"parent_record_offset":6936,"ancestor_record_offset":6248,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":341,"record_offset":6984,"parent_record_offset":6960,"ancestor_record_offset":6248,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":341,"record_offset":7008,"parent_record_offset":6984,"ancestor_record_offset":6248,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":341,"record_offset":6936,"parent_record_offset":6880,"ancestor_record_offset":6248,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":341,"record_offset":6908,"parent_record_offset":6880,"ancestor_record_offset":6248,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":341,"record_offset":6880,"parent_record_offset":6856,"ancestor_record_offset":6248,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":341,"record_offset":6856,"parent_record_offset":6832,"ancestor_record_offset":6248,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":341,"record_offset":6692,"parent_record_offset":6640,"ancestor_record_offset":6248,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":341,"record_offset":6640,"parent_record_offset":6604,"ancestor_record_offset":6248,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":341,"record_offset":7104,"parent_record_offset":7076,"ancestor_record_offset":6248,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$91::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":341,"record_offset":6372,"parent_record_offset":6348,"ancestor_record_offset":6248,"inlinee":21551,"source_line":2666,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":341,"record_offset":7132,"parent_record_offset":7104,"ancestor_record_offset":6248,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":341,"record_offset":6420,"parent_record_offset":6396,"ancestor_record_offset":6248,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":341,"record_offset":6572,"parent_record_offset":6548,"ancestor_record_offset":6248,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":341,"record_offset":6720,"parent_record_offset":6640,"ancestor_record_offset":6248,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":341,"record_offset":6784,"parent_record_offset":6640,"ancestor_record_offset":6248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":341,"record_offset":6748,"parent_record_offset":6640,"ancestor_record_offset":6248,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":341,"record_offset":6444,"parent_record_offset":6420,"ancestor_record_offset":6248,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":341,"record_offset":6472,"parent_record_offset":6444,"ancestor_record_offset":6248,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":341,"record_offset":6396,"parent_record_offset":6372,"ancestor_record_offset":6248,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00341.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::find_extra_files<ref$<str$> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 139
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1222,"record_offset":9088,"rva":18213936,"va":5386923056}
// 编译实例 RVAs: 0x115ec30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::file_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2233963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386923056,"raw_hex":"e895e9ddff","size":5,"soft_fail":false,"target_va":5384689584,"va":5386923542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2234385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386923056,"raw_hex":"e8efe7ddff","size":5,"soft_fail":false,"target_va":5384689312,"va":5386923692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::ReadDir as core::iter::traits::iterator::Iterator>::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2155144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386923056,"raw_hex":"e8781ddfff","size":5,"soft_fail":false,"target_va":5384768304,"va":5386923443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::fs::windows::FindNextFileHandle as core::ops::drop::Drop>::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2160175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386923056,"raw_hex":"e8d109dfff","size":5,"soft_fail":false,"target_va":5384764464,"va":5386924634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11245066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386923056,"raw_hex":"e8f66954ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5386923749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=13
//     evidence={"assembly":"\tcallq\t-11244558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386923056,"raw_hex":"e8f26b54ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5386923257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18201537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386923056,"raw_hex":"e83f44eafe","size":5,"soft_fail":false,"target_va":5368722192,"va":5386923724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140098ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17590971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386923056,"raw_hex":"e84595f3fe","size":5,"soft_fail":false,"target_va":5369335008,"va":5386925974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17584383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386923056,"raw_hex":"e801aff3fe","size":5,"soft_fail":false,"target_va":5369338848,"va":5386923226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13443325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386923056,"raw_hex":"e803df32ff","size":5,"soft_fail":false,"target_va":5373480896,"va":5386924216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409e6e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7831848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386923056,"raw_hex":"e8d87e88ff","size":5,"soft_fail":false,"target_va":5379092080,"va":5386923923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29426518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386923056,"raw_hex":"e85603c101","size":5,"soft_fail":false,"target_va":5416350944,"va":5386924421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30132686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386923056,"raw_hex":"e8cec9cb01","size":5,"soft_fail":false,"target_va":5417057040,"va":5386924349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30133847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386923056,"raw_hex":"e857cecb01","size":5,"soft_fail":false,"target_va":5417057056,"va":5386923204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29696180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386923056,"raw_hex":"e8b420c501","size":5,"soft_fail":false,"target_va":5416620794,"va":5386924609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-4800846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386923056,"raw_hex":"e8b2beb6ff","size":5,"soft_fail":false,"target_va":5382122800,"va":5386923641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386923056,"raw_hex":"e8d1000000","size":5,"soft_fail":false,"target_va":5386926048,"va":5386925834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2093501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386923056,"raw_hex":"e8430ee0ff","size":5,"soft_fail":false,"target_va":5384830080,"va":5386923576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] relative_path::relative_path_buf::RelativePathBuf::push
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t17865791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386923056,"raw_hex":"e83f9c1001","size":5,"soft_fail":false,"target_va":5404789648,"va":5386923852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::read_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2197995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386923056,"raw_hex":"e81576deff","size":5,"soft_fail":false,"target_va":5384725152,"va":5386923142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_package_integrity::find_extra_files<std::path::PathBuf>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386923056,"raw_hex":"e8f0efffff","size":5,"soft_fail":false,"target_va":5386920000,"va":5386924107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1222,"record_offset":13576,"parent_record_offset":9088,"ancestor_record_offset":9088,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1222,"record_offset":9312,"parent_record_offset":9088,"ancestor_record_offset":9088,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_direct] relative_path::RelativePath::join
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/lib.rs
//     evidence={"module":1222,"record_offset":10348,"parent_record_offset":9088,"ancestor_record_offset":9088,"inlinee":69925,"source_line":792,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_direct] relative_path::impl$91::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/lib.rs
//     evidence={"module":1222,"record_offset":11352,"parent_record_offset":9088,"ancestor_record_offset":9088,"inlinee":69927,"source_line":2021,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1222,"record_offset":12524,"parent_record_offset":9088,"ancestor_record_offset":9088,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1222,"record_offset":9680,"parent_record_offset":9088,"ancestor_record_offset":9088,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_direct] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1222,"record_offset":12144,"parent_record_offset":9088,"ancestor_record_offset":9088,"inlinee":13260,"source_line":2392,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1222,"record_offset":9252,"parent_record_offset":9088,"ancestor_record_offset":9088,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1222,"record_offset":10852,"parent_record_offset":10828,"ancestor_record_offset":9088,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1222,"record_offset":10876,"parent_record_offset":10852,"ancestor_record_offset":9088,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1222,"record_offset":9540,"parent_record_offset":9516,"ancestor_record_offset":9088,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1222,"record_offset":9564,"parent_record_offset":9540,"ancestor_record_offset":9088,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1222,"record_offset":10900,"parent_record_offset":10876,"ancestor_record_offset":9088,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1222,"record_offset":9588,"parent_record_offset":9564,"ancestor_record_offset":9088,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1222,"record_offset":10828,"parent_record_offset":10680,"ancestor_record_offset":9088,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1222,"record_offset":9516,"parent_record_offset":9460,"ancestor_record_offset":9088,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1222,"record_offset":13684,"parent_record_offset":13652,"ancestor_record_offset":9088,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1222,"record_offset":10620,"parent_record_offset":10592,"ancestor_record_offset":9088,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1222,"record_offset":9488,"parent_record_offset":9460,"ancestor_record_offset":9088,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1222,"record_offset":9460,"parent_record_offset":9436,"ancestor_record_offset":9088,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1222,"record_offset":13732,"parent_record_offset":13708,"ancestor_record_offset":9088,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1222,"record_offset":13708,"parent_record_offset":13684,"ancestor_record_offset":9088,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1222,"record_offset":10680,"parent_record_offset":10648,"ancestor_record_offset":9088,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1222,"record_offset":10648,"parent_record_offset":10620,"ancestor_record_offset":9088,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1222,"record_offset":9436,"parent_record_offset":9412,"ancestor_record_offset":9088,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1222,"record_offset":10716,"parent_record_offset":10680,"ancestor_record_offset":9088,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1222,"record_offset":10500,"parent_record_offset":10472,"ancestor_record_offset":9088,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1222,"record_offset":10528,"parent_record_offset":10500,"ancestor_record_offset":9088,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1222,"record_offset":10556,"parent_record_offset":10528,"ancestor_record_offset":9088,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1222,"record_offset":10472,"parent_record_offset":10428,"ancestor_record_offset":9088,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1222,"record_offset":10428,"parent_record_offset":10396,"ancestor_record_offset":9088,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1222,"record_offset":10212,"parent_record_offset":10176,"ancestor_record_offset":9088,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1222,"record_offset":10176,"parent_record_offset":10152,"ancestor_record_offset":9088,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1222,"record_offset":13652,"parent_record_offset":13608,"ancestor_record_offset":9088,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1222,"record_offset":13608,"parent_record_offset":13576,"ancestor_record_offset":9088,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1222,"record_offset":10592,"parent_record_offset":10556,"ancestor_record_offset":9088,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1222,"record_offset":10740,"parent_record_offset":10716,"ancestor_record_offset":9088,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1222,"record_offset":13228,"parent_record_offset":13200,"ancestor_record_offset":9088,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1222,"record_offset":12788,"parent_record_offset":12764,"ancestor_record_offset":9088,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1222,"record_offset":12872,"parent_record_offset":12848,"ancestor_record_offset":9088,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1222,"record_offset":13400,"parent_record_offset":12680,"ancestor_record_offset":9088,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1222,"record_offset":10788,"parent_record_offset":10764,"ancestor_record_offset":9088,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1222,"record_offset":11424,"parent_record_offset":11400,"ancestor_record_offset":9088,"inlinee":33894,"source_line":3864,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1222,"record_offset":10764,"parent_record_offset":10740,"ancestor_record_offset":9088,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1222,"record_offset":13020,"parent_record_offset":12992,"ancestor_record_offset":9088,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1222,"record_offset":12992,"parent_record_offset":12964,"ancestor_record_offset":9088,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1222,"record_offset":10956,"parent_record_offset":10556,"ancestor_record_offset":9088,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1222,"record_offset":10980,"parent_record_offset":10956,"ancestor_record_offset":9088,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=60
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1222,"record_offset":9364,"parent_record_offset":9340,"ancestor_record_offset":9088,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1222,"record_offset":13136,"parent_record_offset":13112,"ancestor_record_offset":9088,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1222,"record_offset":10244,"parent_record_offset":10212,"ancestor_record_offset":9088,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1222,"record_offset":13772,"parent_record_offset":13608,"ancestor_record_offset":9088,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1222,"record_offset":9276,"parent_record_offset":9252,"ancestor_record_offset":9088,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1222,"record_offset":13312,"parent_record_offset":13284,"ancestor_record_offset":9088,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1222,"record_offset":13344,"parent_record_offset":13312,"ancestor_record_offset":9088,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1222,"record_offset":13256,"parent_record_offset":13228,"ancestor_record_offset":9088,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1222,"record_offset":10272,"parent_record_offset":10212,"ancestor_record_offset":9088,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1222,"record_offset":10304,"parent_record_offset":10176,"ancestor_record_offset":9088,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1222,"record_offset":9728,"parent_record_offset":9704,"ancestor_record_offset":9088,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1222,"record_offset":12932,"parent_record_offset":12904,"ancestor_record_offset":9088,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1222,"record_offset":12964,"parent_record_offset":12932,"ancestor_record_offset":9088,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1222,"record_offset":12904,"parent_record_offset":12680,"ancestor_record_offset":9088,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1222,"record_offset":12764,"parent_record_offset":12680,"ancestor_record_offset":9088,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1222,"record_offset":13428,"parent_record_offset":12680,"ancestor_record_offset":9088,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1222,"record_offset":12848,"parent_record_offset":12680,"ancestor_record_offset":9088,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1222,"record_offset":12736,"parent_record_offset":12680,"ancestor_record_offset":9088,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1222,"record_offset":13200,"parent_record_offset":13172,"ancestor_record_offset":9088,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1222,"record_offset":12572,"parent_record_offset":12548,"ancestor_record_offset":9088,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1222,"record_offset":13172,"parent_record_offset":13064,"ancestor_record_offset":9088,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1222,"record_offset":12604,"parent_record_offset":12572,"ancestor_record_offset":9088,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1222,"record_offset":13112,"parent_record_offset":13088,"ancestor_record_offset":9088,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1222,"record_offset":13520,"parent_record_offset":12680,"ancestor_record_offset":9088,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1222,"record_offset":13088,"parent_record_offset":13064,"ancestor_record_offset":9088,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1222,"record_offset":12656,"parent_record_offset":12632,"ancestor_record_offset":9088,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1222,"record_offset":12632,"parent_record_offset":12572,"ancestor_record_offset":9088,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1222,"record_offset":12680,"parent_record_offset":12656,"ancestor_record_offset":9088,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1222,"record_offset":13064,"parent_record_offset":12680,"ancestor_record_offset":9088,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1222,"record_offset":12548,"parent_record_offset":12524,"ancestor_record_offset":9088,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::RelativePath::to_relative_path_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/lib.rs
//     evidence={"module":1222,"record_offset":10396,"parent_record_offset":10348,"ancestor_record_offset":9088,"inlinee":21980,"source_line":852,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::impl$27::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/lib.rs
//     evidence={"module":1222,"record_offset":11376,"parent_record_offset":11352,"ancestor_record_offset":9088,"inlinee":33891,"source_line":1757,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::impl$5::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/lib.rs
//     evidence={"module":1222,"record_offset":11400,"parent_record_offset":11376,"ancestor_record_offset":9088,"inlinee":33893,"source_line":554,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":1222,"record_offset":12172,"parent_record_offset":12144,"ancestor_record_offset":9088,"inlinee":13261,"source_line":1196,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1222,"record_offset":9704,"parent_record_offset":9680,"ancestor_record_offset":9088,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::find_extra_files<std::path::PathBuf>
// 记录形式: compiled_procedure; instances=1
// 源码行: 139
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1222,"record_offset":276,"rva":18210880,"va":5386920000}
// 编译实例 RVAs: 0x115e040
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::file_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2230952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386920000,"raw_hex":"e858f5ddff","size":5,"soft_fail":false,"target_va":5384689584,"va":5386920531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2231377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386920000,"raw_hex":"e8aff3ddff","size":5,"soft_fail":false,"target_va":5384689312,"va":5386920684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::ReadDir as core::iter::traits::iterator::Iterator>::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2152133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386920000,"raw_hex":"e83b29dfff","size":5,"soft_fail":false,"target_va":5384768304,"va":5386920432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::fs::windows::FindNextFileHandle as core::ops::drop::Drop>::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2157178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386920000,"raw_hex":"e88615dfff","size":5,"soft_fail":false,"target_va":5384764464,"va":5386921637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11242055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386920000,"raw_hex":"e8b97554ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5386920738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=15
//     evidence={"assembly":"\tcallq\t-11241506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386920000,"raw_hex":"e8de7754ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5386920205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18198526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386920000,"raw_hex":"e80250eafe","size":5,"soft_fail":false,"target_va":5368722192,"va":5386920713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140098ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17588011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386920000,"raw_hex":"e8d5a0f3fe","size":5,"soft_fail":false,"target_va":5369335008,"va":5386923014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17581331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386920000,"raw_hex":"e8edbaf3fe","size":5,"soft_fail":false,"target_va":5369338848,"va":5386920174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13440332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386920000,"raw_hex":"e8b4ea32ff","size":5,"soft_fail":false,"target_va":5373480896,"va":5386921223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409e6e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7828841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386920000,"raw_hex":"e8978a88ff","size":5,"soft_fail":false,"target_va":5379092080,"va":5386920916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29429509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386920000,"raw_hex":"e8050fc101","size":5,"soft_fail":false,"target_va":5416350944,"va":5386921430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30135682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386920000,"raw_hex":"e882d5cb01","size":5,"soft_fail":false,"target_va":5417057040,"va":5386921353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30136893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386920000,"raw_hex":"e83ddacb01","size":5,"soft_fail":false,"target_va":5417057056,"va":5386920158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29699177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386920000,"raw_hex":"e8692cc501","size":5,"soft_fail":false,"target_va":5416620794,"va":5386921612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-4797835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386920000,"raw_hex":"e875cab6ff","size":5,"soft_fail":false,"target_va":5382122800,"va":5386920630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386920000,"raw_hex":"e85b0c0000","size":5,"soft_fail":false,"target_va":5386926048,"va":5386922880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2090490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386920000,"raw_hex":"e8061ae0ff","size":5,"soft_fail":false,"target_va":5384830080,"va":5386920565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] relative_path::relative_path_buf::RelativePathBuf::push
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t17868798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386920000,"raw_hex":"e8fea71001","size":5,"soft_fail":false,"target_va":5404789648,"va":5386920845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::read_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2194958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386920000,"raw_hex":"e8f281deff","size":5,"soft_fail":false,"target_va":5384725152,"va":5386920105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_package_integrity::find_extra_files<std::path::PathBuf>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386920000,"raw_hex":"e8acfbffff","size":5,"soft_fail":false,"target_va":5386920000,"va":5386921103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1222,"record_offset":5140,"parent_record_offset":276,"ancestor_record_offset":276,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1222,"record_offset":508,"parent_record_offset":276,"ancestor_record_offset":276,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_direct] relative_path::RelativePath::join
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/lib.rs
//     evidence={"module":1222,"record_offset":1912,"parent_record_offset":276,"ancestor_record_offset":276,"inlinee":69925,"source_line":792,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_direct] relative_path::impl$91::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/lib.rs
//     evidence={"module":1222,"record_offset":2916,"parent_record_offset":276,"ancestor_record_offset":276,"inlinee":69927,"source_line":2021,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1222,"record_offset":4088,"parent_record_offset":276,"ancestor_record_offset":276,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1222,"record_offset":1244,"parent_record_offset":276,"ancestor_record_offset":276,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_direct] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1222,"record_offset":3708,"parent_record_offset":276,"ancestor_record_offset":276,"inlinee":13260,"source_line":2392,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1222,"record_offset":448,"parent_record_offset":276,"ancestor_record_offset":276,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1222,"record_offset":2416,"parent_record_offset":2392,"ancestor_record_offset":276,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1222,"record_offset":2440,"parent_record_offset":2416,"ancestor_record_offset":276,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1222,"record_offset":736,"parent_record_offset":712,"ancestor_record_offset":276,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1222,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":276,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1222,"record_offset":2464,"parent_record_offset":2440,"ancestor_record_offset":276,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1222,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":276,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1222,"record_offset":2392,"parent_record_offset":2244,"ancestor_record_offset":276,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1222,"record_offset":712,"parent_record_offset":656,"ancestor_record_offset":276,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1222,"record_offset":5248,"parent_record_offset":5216,"ancestor_record_offset":276,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1222,"record_offset":2184,"parent_record_offset":2156,"ancestor_record_offset":276,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1222,"record_offset":684,"parent_record_offset":656,"ancestor_record_offset":276,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1222,"record_offset":656,"parent_record_offset":632,"ancestor_record_offset":276,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1222,"record_offset":5296,"parent_record_offset":5272,"ancestor_record_offset":276,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1222,"record_offset":5272,"parent_record_offset":5248,"ancestor_record_offset":276,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1222,"record_offset":2244,"parent_record_offset":2212,"ancestor_record_offset":276,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1222,"record_offset":2212,"parent_record_offset":2184,"ancestor_record_offset":276,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1222,"record_offset":632,"parent_record_offset":608,"ancestor_record_offset":276,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1222,"record_offset":2280,"parent_record_offset":2244,"ancestor_record_offset":276,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1222,"record_offset":2064,"parent_record_offset":2036,"ancestor_record_offset":276,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1222,"record_offset":2092,"parent_record_offset":2064,"ancestor_record_offset":276,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1222,"record_offset":2120,"parent_record_offset":2092,"ancestor_record_offset":276,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1222,"record_offset":2036,"parent_record_offset":1992,"ancestor_record_offset":276,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1222,"record_offset":1992,"parent_record_offset":1960,"ancestor_record_offset":276,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1222,"record_offset":1776,"parent_record_offset":1740,"ancestor_record_offset":276,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1222,"record_offset":1740,"parent_record_offset":1716,"ancestor_record_offset":276,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1222,"record_offset":5216,"parent_record_offset":5172,"ancestor_record_offset":276,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1222,"record_offset":5172,"parent_record_offset":5140,"ancestor_record_offset":276,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1222,"record_offset":2156,"parent_record_offset":2120,"ancestor_record_offset":276,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1222,"record_offset":2304,"parent_record_offset":2280,"ancestor_record_offset":276,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1222,"record_offset":4792,"parent_record_offset":4764,"ancestor_record_offset":276,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1222,"record_offset":4352,"parent_record_offset":4328,"ancestor_record_offset":276,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1222,"record_offset":4436,"parent_record_offset":4412,"ancestor_record_offset":276,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1222,"record_offset":4964,"parent_record_offset":4244,"ancestor_record_offset":276,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1222,"record_offset":2352,"parent_record_offset":2328,"ancestor_record_offset":276,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1222,"record_offset":2988,"parent_record_offset":2964,"ancestor_record_offset":276,"inlinee":33894,"source_line":3864,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1222,"record_offset":2328,"parent_record_offset":2304,"ancestor_record_offset":276,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1222,"record_offset":4584,"parent_record_offset":4556,"ancestor_record_offset":276,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1222,"record_offset":4556,"parent_record_offset":4528,"ancestor_record_offset":276,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1222,"record_offset":2520,"parent_record_offset":2120,"ancestor_record_offset":276,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1222,"record_offset":2544,"parent_record_offset":2520,"ancestor_record_offset":276,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=70
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1222,"record_offset":560,"parent_record_offset":536,"ancestor_record_offset":276,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1222,"record_offset":4700,"parent_record_offset":4676,"ancestor_record_offset":276,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1222,"record_offset":1808,"parent_record_offset":1776,"ancestor_record_offset":276,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1222,"record_offset":5336,"parent_record_offset":5172,"ancestor_record_offset":276,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1222,"record_offset":472,"parent_record_offset":448,"ancestor_record_offset":276,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1222,"record_offset":4876,"parent_record_offset":4848,"ancestor_record_offset":276,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1222,"record_offset":4908,"parent_record_offset":4876,"ancestor_record_offset":276,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1222,"record_offset":4820,"parent_record_offset":4792,"ancestor_record_offset":276,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1222,"record_offset":1836,"parent_record_offset":1776,"ancestor_record_offset":276,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1222,"record_offset":1868,"parent_record_offset":1740,"ancestor_record_offset":276,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1222,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":276,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1222,"record_offset":4496,"parent_record_offset":4468,"ancestor_record_offset":276,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1222,"record_offset":4528,"parent_record_offset":4496,"ancestor_record_offset":276,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1222,"record_offset":4468,"parent_record_offset":4244,"ancestor_record_offset":276,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1222,"record_offset":4328,"parent_record_offset":4244,"ancestor_record_offset":276,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1222,"record_offset":4992,"parent_record_offset":4244,"ancestor_record_offset":276,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1222,"record_offset":4412,"parent_record_offset":4244,"ancestor_record_offset":276,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1222,"record_offset":4300,"parent_record_offset":4244,"ancestor_record_offset":276,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1222,"record_offset":4764,"parent_record_offset":4736,"ancestor_record_offset":276,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1222,"record_offset":4136,"parent_record_offset":4112,"ancestor_record_offset":276,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1222,"record_offset":4736,"parent_record_offset":4628,"ancestor_record_offset":276,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1222,"record_offset":4168,"parent_record_offset":4136,"ancestor_record_offset":276,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1222,"record_offset":4676,"parent_record_offset":4652,"ancestor_record_offset":276,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1222,"record_offset":5084,"parent_record_offset":4244,"ancestor_record_offset":276,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1222,"record_offset":4652,"parent_record_offset":4628,"ancestor_record_offset":276,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1222,"record_offset":4220,"parent_record_offset":4196,"ancestor_record_offset":276,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1222,"record_offset":4196,"parent_record_offset":4136,"ancestor_record_offset":276,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1222,"record_offset":4244,"parent_record_offset":4220,"ancestor_record_offset":276,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1222,"record_offset":4628,"parent_record_offset":4244,"ancestor_record_offset":276,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1222,"record_offset":4112,"parent_record_offset":4088,"ancestor_record_offset":276,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::RelativePath::to_relative_path_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/lib.rs
//     evidence={"module":1222,"record_offset":1960,"parent_record_offset":1912,"ancestor_record_offset":276,"inlinee":21980,"source_line":852,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::impl$27::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/lib.rs
//     evidence={"module":1222,"record_offset":2940,"parent_record_offset":2916,"ancestor_record_offset":276,"inlinee":33891,"source_line":1757,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::impl$5::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/lib.rs
//     evidence={"module":1222,"record_offset":2964,"parent_record_offset":2940,"ancestor_record_offset":276,"inlinee":33893,"source_line":554,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":1222,"record_offset":3736,"parent_record_offset":3708,"ancestor_record_offset":276,"inlinee":13261,"source_line":1196,"archive":"_pdb/C/modules/01222.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1222,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":276,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01222.jsonl.gz"}
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
// [PDB 命名实体；类型/枚举变体身份待确认] tg_package_integrity::PackageManifest
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tg_package_integrity::PackageManifest>", "constructor": "core::ptr::drop_in_place", "arguments": ["tg_package_integrity::PackageManifest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::from_slice<tg_package_integrity::PackageManifest>", "constructor": "serde_json::de::from_slice", "arguments": ["tg_package_integrity::PackageManifest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tg_package_integrity::PackageManifest,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["tg_package_integrity::PackageManifest", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tg_package_integrity::PackageManifest,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<tg_package_integrity::PackageManifest,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tg_package_integrity::PackageManifest,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tg_package_integrity::PackageManifest,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_package_integrity::PackageVerificationError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tg_package_integrity::PackageVerificationError>", "constructor": "enum2$", "arguments": ["tg_package_integrity::PackageVerificationError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> >", "constructor": "core::result::Result", "arguments": ["tuple$<>", "enum2$<tg_package_integrity::PackageVerificationError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: serde_json::de::Deserializer::parse_exponent<serde_json::read::SliceRead>
// 信息 ID: c2741493b418804ff25d7b76
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::Deserializer::parse_exponent -> ["serde_json::read::SliceRead"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::Deserializer::parse_number<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_number<serde_json::read::SliceRead> @ 0x15d047 -> 0x15d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_number<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_number<serde_json::read::SliceRead> @ 0x15d191 -> 0x15d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead> @ 0xd81e81 -> 0x15d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> @ 0xd821ff -> 0x15d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> @ 0xd8210c -> 0x15d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_long_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_long_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_long_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_long_integer<serde_json::read::SliceRead> @ 0x2d9eda7 -> 0x15d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_decimal_overflow<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal_overflow<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal_overflow<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal_overflow<serde_json::read::SliceRead> @ 0x2d9ef33 -> 0x15d890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead>
// 信息 ID: f282a6b86a913fdf1ac23c39
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::Deserializer::parse_integer -> ["serde_json::read::SliceRead"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 15 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0xd83376 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0xd833bb -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::de::impl$5::deserialize<i64,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0xfa090f -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::de::impl$5::deserialize<i64,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0xfa0944 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::de::impl$5::deserialize<u32,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0x115c9ca -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0x1d5f856 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0x1d5f89b -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2d9e8ef -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2d9e992 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2db2809 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2db2879 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2db8af9 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2db8b69 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2de230f -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2de23b2 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_json::de::impl$7::deserialize_seq<serde_json::read::SliceRead,serde_core::de::impls::impl$19::deserialize::VecVisitor<tuple$<alloc::string::String,alloc::string::String> > >
// 信息 ID: 56ce0127c8bcb9a771056580
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["alloc::string::String", "alloc::string::String"]; 字段布局 = unknown。
// [类型关系] serde_core::de::impls::impl$19::deserialize::VecVisitor -> ["tuple$<alloc::string::String,alloc::string::String>"]; 字段布局 = unknown。
// [类型关系] serde_json::de::impl$7::deserialize_seq -> ["serde_json::read::SliceRead", "serde_core::de::impls::impl$19::deserialize::VecVisitor<tuple$<alloc::string::String,alloc::string::String> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tg_package_integrity::_::impl$0::deserialize::__Visitor> @ 0x115d5c6 -> 0x115caa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tg_package_integrity::_::impl$0::deserialize::__Visitor> @ 0x115d7b0 -> 0x115caa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tg_package_integrity::_::impl$0::deserialize::__Visitor>
// 信息 ID: 426afe2c298a93f55e95b6e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::impl$7::deserialize_struct -> ["serde_json::read::SliceRead", "tg_package_integrity::_::impl$0::deserialize::__Visitor"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::from_slice<tg_package_integrity::PackageManifest> @ 0x115c41f -> 0x115ce90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::_::impl$0::deserialize
// 信息 ID: 659bf9fb1fbdf5000be0d4a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::from_slice<tg_package_integrity::PackageManifest>; module=1220, inline_record=532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 52fd10eb15bce92f9359bd6b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tg_package_integrity::_::impl$0::deserialize::__Visitor>; module=1220, inline_record=23856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 835f9900466945fdaa93131d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tg_package_integrity::_::impl$0::deserialize::__Visitor>; module=1220, inline_record=23668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: a25f9620c3ab4716bf059372
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<tg_package_integrity::_::impl$0::deserialize::__Visitor>; module=1222, inline_record=17324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 05c5970fec247256b4f5e324
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tg_package_integrity::_::impl$0::deserialize::__Visitor>; module=1220, inline_record=22960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: f11947330b7cf8131989eb8f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tg_package_integrity::_::impl$0::deserialize::__Visitor>; module=1220, inline_record=29344; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::check_manifest_file
// 信息 ID: 99cfb88be887bdbcca300724
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_package_integrity::verify_package_manifest; module=341, inline_record=6248; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::find_extra_files<ref$<str$> >
// 信息 ID: 8c3c721285740c7c79b2a017
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tg_package_integrity::find_extra_files -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_package_integrity::verify_package_manifest @ 0x71d6f2 -> 0x115ec30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::find_extra_files<std::path::PathBuf>
// 信息 ID: f8111f4433d5a6409793bfd6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_package_integrity::find_extra_files -> ["std::path::PathBuf"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_package_integrity::find_extra_files<std::path::PathBuf> @ 0x115e48f -> 0x115e040 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_package_integrity::find_extra_files<ref$<str$> > @ 0x115f04b -> 0x115e040 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::is_valid_manifest_checksum
// 信息 ID: f474e369e17af3cc7c1f2e0d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_package_integrity::verify_package_manifest; module=341, inline_record=3276; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::is_valid_manifest_checksum::calculate_fnv_hash
// 信息 ID: 5f946aacb4295381d14f85ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_package_integrity::verify_package_manifest; module=341, inline_record=3304; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::verify_package_manifest
// 信息 ID: 8f265807505e1b6000a4b37e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::integrity_report_main::verify_package_integrity_and_show_report @ 0x522a2e -> 0x71cf60 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x543434 -> 0x71cf60 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::mod_detection::spawn_informational_mods_label_integrity_check::closure_env$0,tuple$<> > @ 0x5e9331 -> 0x71cf60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::verify_package_manifest::closure$0
// 信息 ID: 95a8aa2a5b3e365a6358bfa2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_package_integrity::verify_package_manifest", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_package_integrity::verify_package_manifest; module=341, inline_record=2736; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::verify_package_manifest::closure$1
// 信息 ID: 73e1e20dd327301e6f97b584
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_package_integrity::verify_package_manifest", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_package_integrity::verify_package_manifest; module=341, inline_record=3444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::verify_package_manifest::closure$2
// 信息 ID: 8f47f381aa6f1eb1815583ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tg_package_integrity::verify_package_manifest", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_package_integrity::verify_package_manifest; module=341, inline_record=5712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
