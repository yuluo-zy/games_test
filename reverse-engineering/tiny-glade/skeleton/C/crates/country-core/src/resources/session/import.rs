// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/session/import.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: efd3ddfa141e2bd99148568aaee8d09fc30e1da6245ea582bc37c1f87d81fce5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: import.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 31, 39, 163, 189, 216, 529, 543, 1253, 1267, 1280, 1636, 1666, 1681, 1682, 1770, 1803, 1804, 2893, 2896, 2899, 4234
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.7.6"}]
// use blake3; // namespace candidate
//   package identities: [{"name":"blake3","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.5.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use paths; // namespace candidate
//   package identities: [{"name":"paths","source":"local_or_path","version":"0.1.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"},{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"},{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// use uuid; // namespace candidate
//   package identities: [{"name":"uuid","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.15.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A27
// 首条位置: {"module":216,"inlinee":14887}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1EBC
// 首条位置: {"module":31,"inlinee":7868}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":31,"record_offset":268660,"parent_record_offset":268632,"ancestor_record_offset":268632,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":31,"record_offset":268684,"parent_record_offset":268660,"ancestor_record_offset":268632,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":268708,"parent_record_offset":268684,"ancestor_record_offset":268632,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":268736,"parent_record_offset":268708,"ancestor_record_offset":268632,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1EBA
// 首条位置: {"module":31,"inlinee":7866}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":268476,"parent_record_offset":268452,"ancestor_record_offset":268452,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::de::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":340976,"parent_record_offset":340944,"ancestor_record_offset":340944,"inlinee":37547,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":341596,"parent_record_offset":341572,"ancestor_record_offset":340944,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":341620,"parent_record_offset":341596,"ancestor_record_offset":340944,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":341644,"parent_record_offset":341620,"ancestor_record_offset":340944,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":341572,"parent_record_offset":341500,"ancestor_record_offset":340944,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":341536,"parent_record_offset":341500,"ancestor_record_offset":340944,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":341500,"parent_record_offset":341468,"ancestor_record_offset":340944,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":341468,"parent_record_offset":341436,"ancestor_record_offset":340944,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":31,"record_offset":268604,"parent_record_offset":268500,"ancestor_record_offset":268452,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":31,"record_offset":268684,"parent_record_offset":268660,"ancestor_record_offset":268452,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":341372,"parent_record_offset":341072,"ancestor_record_offset":340944,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":268708,"parent_record_offset":268684,"ancestor_record_offset":268452,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":268736,"parent_record_offset":268708,"ancestor_record_offset":268452,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":31,"record_offset":268660,"parent_record_offset":268632,"ancestor_record_offset":268452,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::import::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":31,"record_offset":268632,"parent_record_offset":268500,"ancestor_record_offset":268452,"inlinee":7868,"source_line":17,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":341072,"parent_record_offset":341040,"ancestor_record_offset":340944,"inlinee":10022,"source_line":1568,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":268548,"parent_record_offset":268500,"ancestor_record_offset":268452,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":268500,"parent_record_offset":268476,"ancestor_record_offset":268452,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":31,"record_offset":268572,"parent_record_offset":268548,"ancestor_record_offset":268452,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$21::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":341008,"parent_record_offset":340976,"ancestor_record_offset":340944,"inlinee":21295,"source_line":1248,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$26::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":341040,"parent_record_offset":341008,"ancestor_record_offset":340944,"inlinee":21297,"source_line":1420,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::_::impl$0::deserialize::impl$2::expecting
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x83AE
// 首条位置: {"module":543,"record_offset":38016,"rva":11022960,"va":5379732080}
// 编译实例 RVAs: 0xa83270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t5091979","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379732080,"raw_hex":"e98bb24d00","size":5,"soft_fail":false,"target_va":5384824080,"va":5379732096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1EB8
// 首条位置: {"module":31,"inlinee":7864}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":31,"record_offset":268916,"parent_record_offset":268312,"ancestor_record_offset":268312,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":270024,"parent_record_offset":268312,"ancestor_record_offset":268312,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":31,"record_offset":269852,"parent_record_offset":268312,"ancestor_record_offset":268312,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":31,"record_offset":269884,"parent_record_offset":268312,"ancestor_record_offset":268312,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":270504,"parent_record_offset":268312,"ancestor_record_offset":268312,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":268364,"parent_record_offset":268312,"ancestor_record_offset":268312,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":268944,"parent_record_offset":268312,"ancestor_record_offset":268312,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":183204,"parent_record_offset":183164,"ancestor_record_offset":183164,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":183296,"parent_record_offset":183164,"ancestor_record_offset":183164,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":270228,"parent_record_offset":270204,"ancestor_record_offset":268312,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":270252,"parent_record_offset":270228,"ancestor_record_offset":268312,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":270276,"parent_record_offset":270252,"ancestor_record_offset":268312,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":270204,"parent_record_offset":270136,"ancestor_record_offset":268312,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":31,"record_offset":270172,"parent_record_offset":270136,"ancestor_record_offset":268312,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":31,"record_offset":270136,"parent_record_offset":270108,"ancestor_record_offset":268312,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":31,"record_offset":270108,"parent_record_offset":270080,"ancestor_record_offset":268312,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":31,"record_offset":268604,"parent_record_offset":268500,"ancestor_record_offset":268312,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":31,"record_offset":268684,"parent_record_offset":268660,"ancestor_record_offset":268312,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":183412,"parent_record_offset":183388,"ancestor_record_offset":183164,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":183388,"parent_record_offset":183352,"ancestor_record_offset":183164,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":270052,"parent_record_offset":270024,"ancestor_record_offset":268312,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":31,"record_offset":269804,"parent_record_offset":269752,"ancestor_record_offset":268312,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":268708,"parent_record_offset":268684,"ancestor_record_offset":268312,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":268736,"parent_record_offset":268708,"ancestor_record_offset":268312,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":31,"record_offset":268660,"parent_record_offset":268632,"ancestor_record_offset":268312,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":31,"record_offset":269752,"parent_record_offset":269728,"ancestor_record_offset":268312,"inlinee":7798,"source_line":122,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::import::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":31,"record_offset":268632,"parent_record_offset":268500,"ancestor_record_offset":268312,"inlinee":7868,"source_line":17,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::import::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":31,"record_offset":268452,"parent_record_offset":268428,"ancestor_record_offset":268312,"inlinee":7866,"source_line":17,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":31,"record_offset":269956,"parent_record_offset":269932,"ancestor_record_offset":268312,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":270408,"parent_record_offset":270384,"ancestor_record_offset":268312,"inlinee":7831,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":269932,"parent_record_offset":269908,"ancestor_record_offset":268312,"inlinee":7748,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":269980,"parent_record_offset":269956,"ancestor_record_offset":268312,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":183228,"parent_record_offset":183204,"ancestor_record_offset":183164,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":183324,"parent_record_offset":183296,"ancestor_record_offset":183164,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":183472,"parent_record_offset":183448,"ancestor_record_offset":183164,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":268428,"parent_record_offset":268388,"ancestor_record_offset":268312,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":31,"record_offset":269328,"parent_record_offset":269304,"ancestor_record_offset":268312,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":268548,"parent_record_offset":268500,"ancestor_record_offset":268312,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":269028,"parent_record_offset":268984,"ancestor_record_offset":268312,"inlinee":7597,"source_line":1061,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":269072,"parent_record_offset":269028,"ancestor_record_offset":268312,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":269112,"parent_record_offset":269072,"ancestor_record_offset":268312,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":268388,"parent_record_offset":268364,"ancestor_record_offset":268312,"inlinee":7724,"source_line":1986,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":268984,"parent_record_offset":268944,"ancestor_record_offset":268312,"inlinee":7596,"source_line":2027,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":268500,"parent_record_offset":268476,"ancestor_record_offset":268312,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":268476,"parent_record_offset":268452,"ancestor_record_offset":268312,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":31,"record_offset":268572,"parent_record_offset":268548,"ancestor_record_offset":268312,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":31,"record_offset":269140,"parent_record_offset":269112,"ancestor_record_offset":268312,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":183352,"parent_record_offset":183324,"ancestor_record_offset":183164,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_string
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":183796,"parent_record_offset":183764,"ancestor_record_offset":183164,"inlinee":33611,"source_line":396,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] uuid::external::serde_support::impl$6::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/uuid-1.15.1/src/external/serde_support.rs
//     evidence={"module":31,"record_offset":269776,"parent_record_offset":269752,"ancestor_record_offset":268312,"inlinee":5914,"source_line":69,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1EBD
// 首条位置: {"module":31,"inlinee":7869}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":270908,"parent_record_offset":270872,"ancestor_record_offset":270872,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$6::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":214332,"parent_record_offset":214300,"ancestor_record_offset":214300,"inlinee":15022,"source_line":1798,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1803,"record_offset":214428,"parent_record_offset":214396,"ancestor_record_offset":214300,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":215068,"parent_record_offset":215024,"ancestor_record_offset":214300,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":214484,"parent_record_offset":214428,"ancestor_record_offset":214300,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":214512,"parent_record_offset":214428,"ancestor_record_offset":214300,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":214456,"parent_record_offset":214428,"ancestor_record_offset":214300,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":214536,"parent_record_offset":214512,"ancestor_record_offset":214300,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":31,"record_offset":271048,"parent_record_offset":270996,"ancestor_record_offset":270872,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":31,"record_offset":270996,"parent_record_offset":270972,"ancestor_record_offset":270872,"inlinee":7798,"source_line":122,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":214364,"parent_record_offset":214332,"ancestor_record_offset":214300,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":270972,"parent_record_offset":270932,"ancestor_record_offset":270872,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":31,"record_offset":271200,"parent_record_offset":271176,"ancestor_record_offset":270872,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":270932,"parent_record_offset":270908,"ancestor_record_offset":270872,"inlinee":7609,"source_line":1933,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$10::next_element_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":214396,"parent_record_offset":214364,"ancestor_record_offset":214300,"inlinee":21316,"source_line":636,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_string
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":215024,"parent_record_offset":214988,"ancestor_record_offset":214300,"inlinee":33611,"source_line":396,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] uuid::external::serde_support::impl$6::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/uuid-1.15.1/src/external/serde_support.rs
//     evidence={"module":31,"record_offset":271020,"parent_record_offset":270996,"ancestor_record_offset":270872,"inlinee":5914,"source_line":69,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A26
// 首条位置: {"module":1636,"inlinee":14886}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1636,"record_offset":77308,"parent_record_offset":77284,"ancestor_record_offset":77284,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":163,"record_offset":19608,"rva":4949920,"va":5373659040}
// 编译实例 RVAs: 0x4b87a0, 0x17cefa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14045ad90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-383642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373659040,"raw_hex":"e86625faff","size":5,"soft_fail":false,"target_va":5373275536,"va":5373659173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14045c8c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-376624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373659040,"raw_hex":"e8d040faff","size":5,"soft_fail":false,"target_va":5373282496,"va":5373659115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b9c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t961696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393674144,"raw_hex":"e8a0ac0e00","size":5,"soft_fail":false,"target_va":5394635920,"va":5393674219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418bd3e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t975798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393674144,"raw_hex":"e8b6e30e00","size":5,"soft_fail":false,"target_va":5394650080,"va":5393674277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42693770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373659040,"raw_hex":"e88a748b02","size":5,"soft_fail":false,"target_va":5416353040,"va":5373659265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142da1cb0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t42898592","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5373659040,"raw_hex":"e9a0948e02","size":5,"soft_fail":false,"target_va":5416557744,"va":5373659147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":163,"record_offset":19892,"parent_record_offset":19608,"ancestor_record_offset":19608,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/00163.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":163,"record_offset":20500,"parent_record_offset":19608,"ancestor_record_offset":19608,"inlinee":4577,"source_line":718,"archive":"_pdb/C/modules/00163.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":163,"record_offset":20436,"parent_record_offset":19608,"ancestor_record_offset":19608,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":163,"record_offset":20288,"parent_record_offset":20256,"ancestor_record_offset":19608,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":163,"record_offset":20156,"parent_record_offset":20128,"ancestor_record_offset":19608,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":163,"record_offset":20216,"parent_record_offset":20184,"ancestor_record_offset":19608,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":163,"record_offset":20336,"parent_record_offset":20312,"ancestor_record_offset":19608,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":163,"record_offset":20312,"parent_record_offset":20288,"ancestor_record_offset":19608,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":163,"record_offset":20184,"parent_record_offset":20156,"ancestor_record_offset":19608,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":163,"record_offset":20088,"parent_record_offset":20060,"ancestor_record_offset":19608,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":163,"record_offset":20256,"parent_record_offset":20088,"ancestor_record_offset":19608,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":163,"record_offset":20032,"parent_record_offset":20004,"ancestor_record_offset":19608,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":163,"record_offset":20376,"parent_record_offset":20088,"ancestor_record_offset":19608,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":163,"record_offset":20128,"parent_record_offset":20088,"ancestor_record_offset":19608,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":163,"record_offset":20060,"parent_record_offset":20032,"ancestor_record_offset":19608,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00163.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":163,"record_offset":19948,"parent_record_offset":19920,"ancestor_record_offset":19608,"inlinee":4570,"source_line":1865,"archive":"_pdb/C/modules/00163.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":163,"record_offset":20560,"parent_record_offset":20532,"ancestor_record_offset":19608,"inlinee":4580,"source_line":1875,"archive":"_pdb/C/modules/00163.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":163,"record_offset":19920,"parent_record_offset":19892,"ancestor_record_offset":19608,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00163.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":163,"record_offset":20532,"parent_record_offset":20500,"ancestor_record_offset":19608,"inlinee":4579,"source_line":669,"archive":"_pdb/C/modules/00163.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":163,"record_offset":20004,"parent_record_offset":19976,"ancestor_record_offset":19608,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00163.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":163,"record_offset":19976,"parent_record_offset":19948,"ancestor_record_offset":19608,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00163.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::impl$1::clone
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x211D
// 首条位置: {"module":39,"inlinee":8477}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::session::impl$20::clone
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":39,"record_offset":40316,"parent_record_offset":40288,"ancestor_record_offset":40288,"inlinee":8479,"source_line":122,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::impl$4::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8373
// 首条位置: {"module":543,"inlinee":33651}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":543,"record_offset":18932,"parent_record_offset":18792,"ancestor_record_offset":18792,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$24::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":543,"record_offset":18820,"parent_record_offset":18792,"ancestor_record_offset":18792,"inlinee":33653,"source_line":122,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":543,"record_offset":18956,"parent_record_offset":18932,"ancestor_record_offset":18792,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":543,"record_offset":18868,"parent_record_offset":18844,"ancestor_record_offset":18792,"inlinee":10467,"source_line":10,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":543,"record_offset":18892,"parent_record_offset":18868,"ancestor_record_offset":18792,"inlinee":10469,"source_line":153,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":543,"record_offset":18980,"parent_record_offset":18956,"ancestor_record_offset":18792,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":543,"record_offset":19004,"parent_record_offset":18980,"ancestor_record_offset":18792,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] uuid::impl$25::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/uuid-1.15.1/src/lib.rs
//     evidence={"module":543,"record_offset":18844,"parent_record_offset":18820,"ancestor_record_offset":18792,"inlinee":11456,"source_line":437,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::impl$5::clone
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11416
// 首条位置: {"module":1267,"inlinee":70678}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::session::impl$20::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1267,"record_offset":15388,"parent_record_offset":15344,"ancestor_record_offset":15344,"inlinee":8479,"source_line":122,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::import::impl$1::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":1267,"record_offset":15416,"parent_record_offset":15344,"ancestor_record_offset":15344,"inlinee":8477,"source_line":17,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$20::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1267,"record_offset":15444,"parent_record_offset":15416,"ancestor_record_offset":15344,"inlinee":8479,"source_line":122,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::impl$6::clone
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114B9
// 首条位置: {"module":1280,"inlinee":70841}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1280,"record_offset":1672,"parent_record_offset":1632,"ancestor_record_offset":1632,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1280,"record_offset":2248,"parent_record_offset":1632,"ancestor_record_offset":1632,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1280,"record_offset":2076,"parent_record_offset":2052,"ancestor_record_offset":1632,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1280,"record_offset":2100,"parent_record_offset":2076,"ancestor_record_offset":1632,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1280,"record_offset":2432,"parent_record_offset":2408,"ancestor_record_offset":1632,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1280,"record_offset":2456,"parent_record_offset":2432,"ancestor_record_offset":1632,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1280,"record_offset":2124,"parent_record_offset":2100,"ancestor_record_offset":1632,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1280,"record_offset":2480,"parent_record_offset":2456,"ancestor_record_offset":1632,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1280,"record_offset":2052,"parent_record_offset":1868,"ancestor_record_offset":1632,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1280,"record_offset":2408,"parent_record_offset":2320,"ancestor_record_offset":1632,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1280,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":1632,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1280,"record_offset":2348,"parent_record_offset":2320,"ancestor_record_offset":1632,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1280,"record_offset":2320,"parent_record_offset":2296,"ancestor_record_offset":1632,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1280,"record_offset":1868,"parent_record_offset":1836,"ancestor_record_offset":1632,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1280,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":1632,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1280,"record_offset":2296,"parent_record_offset":2272,"ancestor_record_offset":1632,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1280,"record_offset":1908,"parent_record_offset":1868,"ancestor_record_offset":1632,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1280,"record_offset":1756,"parent_record_offset":1696,"ancestor_record_offset":1632,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1280,"record_offset":1728,"parent_record_offset":1696,"ancestor_record_offset":1632,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1280,"record_offset":1788,"parent_record_offset":1756,"ancestor_record_offset":1632,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1280,"record_offset":1696,"parent_record_offset":1672,"ancestor_record_offset":1632,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1280,"record_offset":1932,"parent_record_offset":1908,"ancestor_record_offset":1632,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1280,"record_offset":2012,"parent_record_offset":1956,"ancestor_record_offset":1632,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1280,"record_offset":1956,"parent_record_offset":1932,"ancestor_record_offset":1632,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1280,"record_offset":1984,"parent_record_offset":1956,"ancestor_record_offset":1632,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1280,"record_offset":2376,"parent_record_offset":2348,"ancestor_record_offset":1632,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1280,"record_offset":2180,"parent_record_offset":1756,"ancestor_record_offset":1632,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1280,"record_offset":2204,"parent_record_offset":2180,"ancestor_record_offset":1632,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1280,"record_offset":2272,"parent_record_offset":2248,"ancestor_record_offset":1632,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01280.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::source_root
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 39, 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7BF3
// 首条位置: {"module":529,"inlinee":31731}
// 编译实例 RVAs: 0xa82ee0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] paths::beta_saves
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t3404763","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379731168,"raw_hex":"e9dbf33300","size":5,"soft_fail":false,"target_va":5383135936,"va":5379731168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::source_folder
// 记录形式: compiled_procedure; instances=1
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":543,"record_offset":35252,"rva":11022064,"va":5379731184}
// 编译实例 RVAs: 0xa82ef0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4924903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379731184,"raw_hex":"e8e7254b00","size":5,"soft_fail":false,"target_va":5384656240,"va":5379731332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-4052658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379731184,"raw_hex":"e84e29c2ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379731357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ea38d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4327795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379731184,"raw_hex":"e873094200","size":5,"soft_fail":false,"target_va":5384059088,"va":5379731288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36890213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379731184,"raw_hex":"e865e63202","size":5,"soft_fail":false,"target_va":5416621632,"va":5379731414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::beta_saves
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3404712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379731184,"raw_hex":"e8a8f33300","size":5,"soft_fail":false,"target_va":5383135936,"va":5379731219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":543,"record_offset":35428,"parent_record_offset":35252,"ancestor_record_offset":35252,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::import::source_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":543,"record_offset":35400,"parent_record_offset":35252,"ancestor_record_offset":35252,"inlinee":31731,"source_line":39,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":543,"record_offset":35884,"parent_record_offset":35252,"ancestor_record_offset":35252,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":35768,"parent_record_offset":35744,"ancestor_record_offset":35252,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":35792,"parent_record_offset":35768,"ancestor_record_offset":35252,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":35816,"parent_record_offset":35792,"ancestor_record_offset":35252,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":35744,"parent_record_offset":35688,"ancestor_record_offset":35252,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":35716,"parent_record_offset":35688,"ancestor_record_offset":35252,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":35688,"parent_record_offset":35664,"ancestor_record_offset":35252,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":35664,"parent_record_offset":35640,"ancestor_record_offset":35252,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":543,"record_offset":35504,"parent_record_offset":35456,"ancestor_record_offset":35252,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":543,"record_offset":35456,"parent_record_offset":35428,"ancestor_record_offset":35252,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":543,"record_offset":35532,"parent_record_offset":35456,"ancestor_record_offset":35252,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":543,"record_offset":35592,"parent_record_offset":35456,"ancestor_record_offset":35252,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":543,"record_offset":35560,"parent_record_offset":35456,"ancestor_record_offset":35252,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::ImportMenu::read
// 记录形式: compiled_procedure; instances=1
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":543,"record_offset":16820,"rva":11016112,"va":5379725232}
// 编译实例 RVAs: 0xa817b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-4046752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725232,"raw_hex":"e86040c2ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379725451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c1380
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10225071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725232,"raw_hex":"e851fa63ff","size":5,"soft_fail":false,"target_va":5369500544,"va":5379725610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141612320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12126999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725232,"raw_hex":"e8170bb900","size":5,"soft_fail":false,"target_va":5391852320,"va":5379725316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141617710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12148439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725232,"raw_hex":"e8d75eb900","size":5,"soft_fail":false,"target_va":5391873808,"va":5379725364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::enumerate_saved_sessions
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t541577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725232,"raw_hex":"e889430800","size":5,"soft_fail":false,"target_va":5380266864,"va":5379725282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::beta_saves
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3410664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725232,"raw_hex":"e8e80a3400","size":5,"soft_fail":false,"target_va":5383135936,"va":5379725267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":543,"record_offset":17088,"parent_record_offset":16820,"ancestor_record_offset":16820,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":543,"record_offset":17312,"parent_record_offset":16820,"ancestor_record_offset":16820,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":543,"record_offset":17004,"parent_record_offset":16820,"ancestor_record_offset":16820,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::import::source_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":543,"record_offset":16976,"parent_record_offset":16820,"ancestor_record_offset":16820,"inlinee":31731,"source_line":39,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":17640,"parent_record_offset":17616,"ancestor_record_offset":16820,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":17664,"parent_record_offset":17640,"ancestor_record_offset":16820,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":17688,"parent_record_offset":17664,"ancestor_record_offset":16820,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":17616,"parent_record_offset":17552,"ancestor_record_offset":16820,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":17584,"parent_record_offset":17552,"ancestor_record_offset":16820,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":17552,"parent_record_offset":17524,"ancestor_record_offset":16820,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":17524,"parent_record_offset":17496,"ancestor_record_offset":16820,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":543,"record_offset":17112,"parent_record_offset":17088,"ancestor_record_offset":16820,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":543,"record_offset":17336,"parent_record_offset":17312,"ancestor_record_offset":16820,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":543,"record_offset":17276,"parent_record_offset":17252,"ancestor_record_offset":16820,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":543,"record_offset":17868,"parent_record_offset":17840,"ancestor_record_offset":16820,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":543,"record_offset":17360,"parent_record_offset":17336,"ancestor_record_offset":16820,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":543,"record_offset":17052,"parent_record_offset":17028,"ancestor_record_offset":16820,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":543,"record_offset":17028,"parent_record_offset":17004,"ancestor_record_offset":16820,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::impl$0::read::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x141F5
// 首条位置: {"module":1666,"inlinee":82421}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1666,"record_offset":82036,"parent_record_offset":81168,"ancestor_record_offset":81168,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$48::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1666,"record_offset":81300,"parent_record_offset":81168,"ancestor_record_offset":81168,"inlinee":10158,"source_line":2779,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1666,"record_offset":81328,"parent_record_offset":81168,"ancestor_record_offset":81168,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1666,"record_offset":81216,"parent_record_offset":81168,"ancestor_record_offset":81168,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1666,"record_offset":81548,"parent_record_offset":81524,"ancestor_record_offset":81168,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1666,"record_offset":81572,"parent_record_offset":81548,"ancestor_record_offset":81168,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1666,"record_offset":81596,"parent_record_offset":81572,"ancestor_record_offset":81168,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1666,"record_offset":81524,"parent_record_offset":81472,"ancestor_record_offset":81168,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1666,"record_offset":81496,"parent_record_offset":81472,"ancestor_record_offset":81168,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1666,"record_offset":81472,"parent_record_offset":81448,"ancestor_record_offset":81168,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1666,"record_offset":81448,"parent_record_offset":81428,"ancestor_record_offset":81168,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1666,"record_offset":82724,"parent_record_offset":82696,"ancestor_record_offset":81168,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1666,"record_offset":84292,"parent_record_offset":84264,"ancestor_record_offset":81168,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1666,"record_offset":81248,"parent_record_offset":81216,"ancestor_record_offset":81168,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::impl$0::read::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14376
// 首条位置: {"module":1682,"inlinee":82806}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1682,"record_offset":62396,"parent_record_offset":62076,"ancestor_record_offset":62076,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::with_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1682,"record_offset":62148,"parent_record_offset":62076,"ancestor_record_offset":62076,"inlinee":17362,"source_line":58,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":1682,"record_offset":62480,"parent_record_offset":62076,"ancestor_record_offset":62076,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":62508,"parent_record_offset":62076,"ancestor_record_offset":62076,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1682,"record_offset":62956,"parent_record_offset":62076,"ancestor_record_offset":62076,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":62836,"parent_record_offset":62812,"ancestor_record_offset":62076,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":62860,"parent_record_offset":62836,"ancestor_record_offset":62076,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":62884,"parent_record_offset":62860,"ancestor_record_offset":62076,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":62812,"parent_record_offset":62736,"ancestor_record_offset":62076,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1682,"record_offset":62212,"parent_record_offset":62188,"ancestor_record_offset":62076,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1682,"record_offset":62260,"parent_record_offset":62236,"ancestor_record_offset":62076,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":62772,"parent_record_offset":62736,"ancestor_record_offset":62076,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":62736,"parent_record_offset":62704,"ancestor_record_offset":62076,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":62704,"parent_record_offset":62676,"ancestor_record_offset":62076,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1682,"record_offset":63208,"parent_record_offset":63068,"ancestor_record_offset":62076,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":1682,"record_offset":63236,"parent_record_offset":63208,"ancestor_record_offset":62076,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":1682,"record_offset":62324,"parent_record_offset":62300,"ancestor_record_offset":62076,"inlinee":6285,"source_line":398,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1682,"record_offset":62300,"parent_record_offset":62148,"ancestor_record_offset":62076,"inlinee":6338,"source_line":33,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":1682,"record_offset":63144,"parent_record_offset":63120,"ancestor_record_offset":62076,"inlinee":10467,"source_line":10,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":1682,"record_offset":63168,"parent_record_offset":63144,"ancestor_record_offset":62076,"inlinee":10469,"source_line":153,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1682,"record_offset":62236,"parent_record_offset":62212,"ancestor_record_offset":62076,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":62364,"parent_record_offset":62148,"ancestor_record_offset":62076,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1682,"record_offset":63008,"parent_record_offset":62984,"ancestor_record_offset":62076,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1682,"record_offset":63264,"parent_record_offset":63236,"ancestor_record_offset":62076,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1682,"record_offset":63292,"parent_record_offset":63264,"ancestor_record_offset":62076,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1682,"record_offset":62984,"parent_record_offset":62956,"ancestor_record_offset":62076,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$24::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1682,"record_offset":63096,"parent_record_offset":63068,"ancestor_record_offset":62076,"inlinee":33653,"source_line":122,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::import::impl$0::read::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":1682,"record_offset":62188,"parent_record_offset":62148,"ancestor_record_offset":62076,"inlinee":70367,"source_line":67,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::import::impl$0::read::closure$1::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":1682,"record_offset":63044,"parent_record_offset":62956,"ancestor_record_offset":62076,"inlinee":82807,"source_line":69,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::import::impl$4::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":1682,"record_offset":63068,"parent_record_offset":63044,"ancestor_record_offset":62076,"inlinee":33651,"source_line":17,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] uuid::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/uuid-1.15.1/src/lib.rs
//     evidence={"module":1682,"record_offset":63120,"parent_record_offset":63096,"ancestor_record_offset":62076,"inlinee":11456,"source_line":437,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::impl$0::read::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x112DF
// 首条位置: {"module":1253,"inlinee":70367}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1682,"record_offset":62212,"parent_record_offset":62188,"ancestor_record_offset":62188,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1682,"record_offset":62260,"parent_record_offset":62236,"ancestor_record_offset":62188,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1682,"record_offset":62236,"parent_record_offset":62212,"ancestor_record_offset":62188,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::impl$0::read::closure$1::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14377
// 首条位置: {"module":1682,"inlinee":82807}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::session::import::impl$4::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":1682,"record_offset":63068,"parent_record_offset":63044,"ancestor_record_offset":63044,"inlinee":33651,"source_line":17,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1682,"record_offset":63208,"parent_record_offset":63068,"ancestor_record_offset":63044,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":1682,"record_offset":63236,"parent_record_offset":63208,"ancestor_record_offset":63044,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":1682,"record_offset":63144,"parent_record_offset":63120,"ancestor_record_offset":63044,"inlinee":10467,"source_line":10,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":1682,"record_offset":63168,"parent_record_offset":63144,"ancestor_record_offset":63044,"inlinee":10469,"source_line":153,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1682,"record_offset":63264,"parent_record_offset":63236,"ancestor_record_offset":63044,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1682,"record_offset":63292,"parent_record_offset":63264,"ancestor_record_offset":63044,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$24::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1682,"record_offset":63096,"parent_record_offset":63068,"ancestor_record_offset":63044,"inlinee":33653,"source_line":122,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] uuid::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/uuid-1.15.1/src/lib.rs
//     evidence={"module":1682,"record_offset":63120,"parent_record_offset":63096,"ancestor_record_offset":63044,"inlinee":11456,"source_line":437,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::read_origin
// 记录形式: compiled_procedure; instances=1
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":543,"record_offset":30464,"rva":11020192,"va":5379729312}
// 编译实例 RVAs: 0xa827a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::OpenOptions>::_open
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4973937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379729312,"raw_hex":"e871e54b00","size":5,"soft_fail":false,"target_va":5384703536,"va":5379729594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4926854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379729312,"raw_hex":"e8862d4b00","size":5,"soft_fail":false,"target_va":5384656240,"va":5379729381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <str as core::fmt::Display>::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5140635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379729312,"raw_hex":"e89b704e00","size":5,"soft_fail":false,"target_va":5384871088,"va":5379730448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4051405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379729312,"raw_hex":"e8332ec2ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5379730088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-4051145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379729312,"raw_hex":"e8372fc2ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379729844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11007886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379729312,"raw_hex":"e8720858ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5379730073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b2470
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10290904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379729312,"raw_hex":"e828f962ff","size":5,"soft_fail":false,"target_va":5369439344,"va":5379730243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409db1f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-685808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379729312,"raw_hex":"e81089f5ff","size":5,"soft_fail":false,"target_va":5379043824,"va":5379729627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417ceb80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13942931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379729312,"raw_hex":"e893c0d400","size":5,"soft_fail":false,"target_va":5393673088,"va":5379730152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd54f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37038524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379729312,"raw_hex":"e8bc293502","size":5,"soft_fail":false,"target_va":5416768752,"va":5379730223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd56b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37039586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379729312,"raw_hex":"e8e22d3502","size":5,"soft_fail":false,"target_va":5416769200,"va":5379729609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37327049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379729312,"raw_hex":"e8c9903902","size":5,"soft_fail":false,"target_va":5417057056,"va":5379730002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36890183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379729312,"raw_hex":"e847e63202","size":5,"soft_fail":false,"target_va":5416620794,"va":5379730606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4894330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379729312,"raw_hex":"e87aae4a00","size":5,"soft_fail":false,"target_va":5384624080,"va":5379729745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hash::to_hex
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5386221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379729312,"raw_hex":"e8ed2f5200","size":5,"soft_fail":false,"target_va":5385116576,"va":5379730350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hasher::finalize
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5391281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379729312,"raw_hex":"e8b1435200","size":5,"soft_fail":false,"target_va":5385121616,"va":5379730330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hasher::update
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5389895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379729312,"raw_hex":"e8473e5200","size":5,"soft_fail":false,"target_va":5385119680,"va":5379729780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36891039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379729312,"raw_hex":"e89fe93202","size":5,"soft_fail":false,"target_va":5416621632,"va":5379730588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionMeta::load_or_default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t532523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379729312,"raw_hex":"e82b200800","size":5,"soft_fail":false,"target_va":5380262416,"va":5379729888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5022261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379729312,"raw_hex":"e835a24c00","size":5,"soft_fail":false,"target_va":5384751936,"va":5379729670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":543,"record_offset":33420,"parent_record_offset":30464,"ancestor_record_offset":30464,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] blake3::Hasher::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":543,"record_offset":30664,"parent_record_offset":30464,"ancestor_record_offset":30464,"inlinee":33662,"source_line":1010,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":543,"record_offset":31020,"parent_record_offset":30464,"ancestor_record_offset":30464,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":543,"record_offset":30956,"parent_record_offset":30464,"ancestor_record_offset":30464,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":543,"record_offset":30900,"parent_record_offset":30464,"ancestor_record_offset":30464,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_vec
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":543,"record_offset":32164,"parent_record_offset":30464,"ancestor_record_offset":30464,"inlinee":8445,"source_line":2213,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":543,"record_offset":30756,"parent_record_offset":30464,"ancestor_record_offset":30464,"inlinee":7533,"source_line":570,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":543,"record_offset":30928,"parent_record_offset":30464,"ancestor_record_offset":30464,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":543,"record_offset":30636,"parent_record_offset":30464,"ancestor_record_offset":30464,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":32396,"parent_record_offset":32372,"ancestor_record_offset":30464,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":32420,"parent_record_offset":32396,"ancestor_record_offset":30464,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":31228,"parent_record_offset":31204,"ancestor_record_offset":30464,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":31252,"parent_record_offset":31228,"ancestor_record_offset":30464,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":32444,"parent_record_offset":32420,"ancestor_record_offset":30464,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":31276,"parent_record_offset":31252,"ancestor_record_offset":30464,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":32372,"parent_record_offset":32344,"ancestor_record_offset":30464,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":31204,"parent_record_offset":31148,"ancestor_record_offset":30464,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":32284,"parent_record_offset":32256,"ancestor_record_offset":30464,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":31176,"parent_record_offset":31148,"ancestor_record_offset":30464,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":31148,"parent_record_offset":31124,"ancestor_record_offset":30464,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":32344,"parent_record_offset":32312,"ancestor_record_offset":30464,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":32312,"parent_record_offset":32284,"ancestor_record_offset":30464,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":31124,"parent_record_offset":31100,"ancestor_record_offset":30464,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":543,"record_offset":33508,"parent_record_offset":33456,"ancestor_record_offset":30464,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":543,"record_offset":33456,"parent_record_offset":33420,"ancestor_record_offset":30464,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":543,"record_offset":32228,"parent_record_offset":32164,"ancestor_record_offset":30464,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":543,"record_offset":32256,"parent_record_offset":32228,"ancestor_record_offset":30464,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::array_string::ArrayString::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/array_string.rs
//     evidence={"module":543,"record_offset":33612,"parent_record_offset":33588,"ancestor_record_offset":30464,"inlinee":33671,"source_line":421,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::array_string::impl$14::fmt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/array_string.rs
//     evidence={"module":543,"record_offset":33564,"parent_record_offset":33456,"ancestor_record_offset":30464,"inlinee":33667,"source_line":515,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::array_string::impl$2::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/array_string.rs
//     evidence={"module":543,"record_offset":33588,"parent_record_offset":33564,"ancestor_record_offset":30464,"inlinee":33669,"source_line":435,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":543,"record_offset":34052,"parent_record_offset":34028,"ancestor_record_offset":30464,"inlinee":4748,"source_line":254,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":543,"record_offset":34028,"parent_record_offset":34004,"ancestor_record_offset":30464,"inlinee":4746,"source_line":50,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":543,"record_offset":34128,"parent_record_offset":34100,"ancestor_record_offset":30464,"inlinee":4753,"source_line":716,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec_impl.rs
//     evidence={"module":543,"record_offset":34076,"parent_record_offset":34052,"ancestor_record_offset":30464,"inlinee":4750,"source_line":72,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec_impl.rs
//     evidence={"module":543,"record_offset":34100,"parent_record_offset":34076,"ancestor_record_offset":30464,"inlinee":4751,"source_line":76,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] blake3::ChunkState::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":543,"record_offset":30716,"parent_record_offset":30688,"ancestor_record_offset":30464,"inlinee":33665,"source_line":477,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] blake3::Hasher::new_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":543,"record_offset":30688,"parent_record_offset":30664,"ancestor_record_offset":30464,"inlinee":33663,"source_line":1001,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":543,"record_offset":30840,"parent_record_offset":30756,"ancestor_record_offset":30464,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":543,"record_offset":33536,"parent_record_offset":33456,"ancestor_record_offset":30464,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":543,"record_offset":31044,"parent_record_offset":31020,"ancestor_record_offset":30464,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":543,"record_offset":33652,"parent_record_offset":33456,"ancestor_record_offset":30464,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":543,"record_offset":32532,"parent_record_offset":32504,"ancestor_record_offset":30464,"inlinee":8447,"source_line":28,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":543,"record_offset":32556,"parent_record_offset":32532,"ancestor_record_offset":30464,"inlinee":8092,"source_line":52,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":543,"record_offset":32504,"parent_record_offset":32164,"ancestor_record_offset":30464,"inlinee":8446,"source_line":2177,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":543,"record_offset":30780,"parent_record_offset":30756,"ancestor_record_offset":30464,"inlinee":7535,"source_line":1692,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":543,"record_offset":30868,"parent_record_offset":30756,"ancestor_record_offset":30464,"inlinee":7538,"source_line":1915,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":543,"record_offset":30808,"parent_record_offset":30780,"ancestor_record_offset":30464,"inlinee":7537,"source_line":215,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::copy_glade
// 记录形式: compiled_procedure; instances=1
// 源码行: 103
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":543,"record_offset":18324,"rva":11016512,"va":5379725632}
// 编译实例 RVAs: 0xa81940
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5069906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725632,"raw_hex":"e8525c4d00","size":5,"soft_fail":false,"target_va":5384795984,"va":5379726073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4933526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725632,"raw_hex":"e896474b00","size":5,"soft_fail":false,"target_va":5384659264,"va":5379725733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4963909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725632,"raw_hex":"e845be4b00","size":5,"soft_fail":false,"target_va":5384689616,"va":5379725702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4930025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725632,"raw_hex":"e8e9394b00","size":5,"soft_fail":false,"target_va":5384656240,"va":5379726210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4882914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725632,"raw_hex":"e8e2814a00","size":5,"soft_fail":false,"target_va":5384609408,"va":5379726489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-4047290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725632,"raw_hex":"e8463ec2ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379725989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10387659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725632,"raw_hex":"e8357f61ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5379726502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b2470
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10286898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725632,"raw_hex":"e8ce0863ff","size":5,"soft_fail":false,"target_va":5369439344,"va":5379726237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141979720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15694731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725632,"raw_hex":"e88b7bef00","size":5,"soft_fail":false,"target_va":5395420960,"va":5379726224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd38d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37034980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725632,"raw_hex":"e8e41b3502","size":5,"soft_fail":false,"target_va":5416761552,"va":5379726567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd4bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37040571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725632,"raw_hex":"e8bb313502","size":5,"soft_fail":false,"target_va":5416766384,"va":5379725808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37331076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725632,"raw_hex":"e884a03902","size":5,"soft_fail":false,"target_va":5417057040,"va":5379725959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37331008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725632,"raw_hex":"e840a03902","size":5,"soft_fail":false,"target_va":5417057056,"va":5379726043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4897322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725632,"raw_hex":"e82aba4a00","size":5,"soft_fail":false,"target_va":5384624080,"va":5379726753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionMeta::load_or_default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t536407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725632,"raw_hex":"e8572f0800","size":5,"soft_fail":false,"target_va":5380262416,"va":5379726004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::import::copy_folder
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725632,"raw_hex":"e8f1030000","size":5,"soft_fail":false,"target_va":5379726848,"va":5379725834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::import::read_origin
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725632,"raw_hex":"e8600d0000","size":5,"soft_fail":false,"target_va":5379729312,"va":5379725883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::remove_dir_all
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4996963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379725632,"raw_hex":"e8633f4c00","size":5,"soft_fail":false,"target_va":5384723232,"va":5379726264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":543,"record_offset":18576,"parent_record_offset":18324,"ancestor_record_offset":18324,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":543,"record_offset":20364,"parent_record_offset":18324,"ancestor_record_offset":18324,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":543,"record_offset":20532,"parent_record_offset":18324,"ancestor_record_offset":18324,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":543,"record_offset":20196,"parent_record_offset":18324,"ancestor_record_offset":18324,"inlinee":11666,"source_line":646,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::import::copy_glade::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":543,"record_offset":18696,"parent_record_offset":18324,"ancestor_record_offset":18324,"inlinee":33649,"source_line":105,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":543,"record_offset":20420,"parent_record_offset":18324,"ancestor_record_offset":18324,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":543,"record_offset":20280,"parent_record_offset":18324,"ancestor_record_offset":18324,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":543,"record_offset":18492,"parent_record_offset":18324,"ancestor_record_offset":18324,"inlinee":33646,"source_line":3077,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":543,"record_offset":20252,"parent_record_offset":18324,"ancestor_record_offset":18324,"inlinee":13843,"source_line":3232,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":19248,"parent_record_offset":19224,"ancestor_record_offset":18324,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":19272,"parent_record_offset":19248,"ancestor_record_offset":18324,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":19296,"parent_record_offset":19272,"ancestor_record_offset":18324,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":19224,"parent_record_offset":19168,"ancestor_record_offset":18324,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":19196,"parent_record_offset":19168,"ancestor_record_offset":18324,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":19168,"parent_record_offset":19144,"ancestor_record_offset":18324,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":19144,"parent_record_offset":19120,"ancestor_record_offset":18324,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":543,"record_offset":18932,"parent_record_offset":18792,"ancestor_record_offset":18324,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":543,"record_offset":18956,"parent_record_offset":18932,"ancestor_record_offset":18324,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":543,"record_offset":18656,"parent_record_offset":18608,"ancestor_record_offset":18324,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":543,"record_offset":20164,"parent_record_offset":18696,"ancestor_record_offset":18324,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":543,"record_offset":18608,"parent_record_offset":18576,"ancestor_record_offset":18324,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":543,"record_offset":18868,"parent_record_offset":18844,"ancestor_record_offset":18324,"inlinee":10467,"source_line":10,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":543,"record_offset":18892,"parent_record_offset":18868,"ancestor_record_offset":18324,"inlinee":10469,"source_line":153,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":543,"record_offset":20388,"parent_record_offset":20364,"ancestor_record_offset":18324,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":543,"record_offset":19048,"parent_record_offset":18696,"ancestor_record_offset":18324,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":543,"record_offset":20220,"parent_record_offset":20196,"ancestor_record_offset":18324,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":543,"record_offset":18760,"parent_record_offset":18696,"ancestor_record_offset":18324,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":543,"record_offset":18980,"parent_record_offset":18956,"ancestor_record_offset":18324,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":543,"record_offset":19004,"parent_record_offset":18980,"ancestor_record_offset":18324,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":543,"record_offset":20304,"parent_record_offset":20280,"ancestor_record_offset":18324,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":543,"record_offset":20328,"parent_record_offset":20304,"ancestor_record_offset":18324,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::SessionMeta::write_to_folder
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":543,"record_offset":19796,"parent_record_offset":18696,"ancestor_record_offset":18324,"inlinee":33654,"source_line":216,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":543,"record_offset":19424,"parent_record_offset":19396,"ancestor_record_offset":18324,"inlinee":8479,"source_line":122,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$24::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":543,"record_offset":18820,"parent_record_offset":18792,"ancestor_record_offset":18324,"inlinee":33653,"source_line":122,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::import::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":543,"record_offset":19396,"parent_record_offset":18696,"ancestor_record_offset":18324,"inlinee":8477,"source_line":17,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::import::impl$4::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":543,"record_offset":18792,"parent_record_offset":18696,"ancestor_record_offset":18324,"inlinee":33651,"source_line":17,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":543,"record_offset":20468,"parent_record_offset":20444,"ancestor_record_offset":18324,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":543,"record_offset":20444,"parent_record_offset":20420,"ancestor_record_offset":18324,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":543,"record_offset":20492,"parent_record_offset":20468,"ancestor_record_offset":18324,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":543,"record_offset":18544,"parent_record_offset":18492,"ancestor_record_offset":18324,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":543,"record_offset":18516,"parent_record_offset":18492,"ancestor_record_offset":18324,"inlinee":33647,"source_line":3398,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":543,"record_offset":19820,"parent_record_offset":19796,"ancestor_record_offset":18324,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] uuid::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/uuid-1.15.1/src/lib.rs
//     evidence={"module":543,"record_offset":18844,"parent_record_offset":18820,"ancestor_record_offset":18324,"inlinee":11456,"source_line":437,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::copy_glade::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8371
// 首条位置: {"module":543,"inlinee":33649}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":543,"record_offset":20164,"parent_record_offset":18696,"ancestor_record_offset":18696,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":543,"record_offset":19048,"parent_record_offset":18696,"ancestor_record_offset":18696,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":543,"record_offset":18760,"parent_record_offset":18696,"ancestor_record_offset":18696,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::SessionMeta::write_to_folder
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":543,"record_offset":19796,"parent_record_offset":18696,"ancestor_record_offset":18696,"inlinee":33654,"source_line":216,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::import::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":543,"record_offset":19396,"parent_record_offset":18696,"ancestor_record_offset":18696,"inlinee":8477,"source_line":17,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::import::impl$4::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":543,"record_offset":18792,"parent_record_offset":18696,"ancestor_record_offset":18696,"inlinee":33651,"source_line":17,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":19248,"parent_record_offset":19224,"ancestor_record_offset":18696,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":19272,"parent_record_offset":19248,"ancestor_record_offset":18696,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":19296,"parent_record_offset":19272,"ancestor_record_offset":18696,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":19224,"parent_record_offset":19168,"ancestor_record_offset":18696,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":19196,"parent_record_offset":19168,"ancestor_record_offset":18696,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":19168,"parent_record_offset":19144,"ancestor_record_offset":18696,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":19144,"parent_record_offset":19120,"ancestor_record_offset":18696,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":543,"record_offset":18932,"parent_record_offset":18792,"ancestor_record_offset":18696,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":543,"record_offset":18956,"parent_record_offset":18932,"ancestor_record_offset":18696,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":543,"record_offset":18868,"parent_record_offset":18844,"ancestor_record_offset":18696,"inlinee":10467,"source_line":10,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":543,"record_offset":18892,"parent_record_offset":18868,"ancestor_record_offset":18696,"inlinee":10469,"source_line":153,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":543,"record_offset":19072,"parent_record_offset":19048,"ancestor_record_offset":18696,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":543,"record_offset":18980,"parent_record_offset":18956,"ancestor_record_offset":18696,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":543,"record_offset":19004,"parent_record_offset":18980,"ancestor_record_offset":18696,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":543,"record_offset":19424,"parent_record_offset":19396,"ancestor_record_offset":18696,"inlinee":8479,"source_line":122,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$24::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":543,"record_offset":18820,"parent_record_offset":18792,"ancestor_record_offset":18696,"inlinee":33653,"source_line":122,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":543,"record_offset":19820,"parent_record_offset":19796,"ancestor_record_offset":18696,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] uuid::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/uuid-1.15.1/src/lib.rs
//     evidence={"module":543,"record_offset":18844,"parent_record_offset":18820,"ancestor_record_offset":18696,"inlinee":11456,"source_line":437,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::copy_folder
// 记录形式: compiled_procedure; instances=1
// 源码行: 124
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":543,"record_offset":20592,"rva":11017728,"va":5379726848}
// 编译实例 RVAs: 0xa81e00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4962237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379726848,"raw_hex":"e8bdb74b00","size":5,"soft_fail":false,"target_va":5384689616,"va":5379727374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::file_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4962494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379726848,"raw_hex":"e8beb84b00","size":5,"soft_fail":false,"target_va":5384689584,"va":5379727085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::path
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t4962203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379726848,"raw_hex":"e89bb74b00","size":5,"soft_fail":false,"target_va":5384689312,"va":5379727104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::ReadDir as core::iter::traits::iterator::Iterator>::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5041300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379726848,"raw_hex":"e894ec4c00","size":5,"soft_fail":false,"target_va":5384768304,"va":5379726999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4928970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379726848,"raw_hex":"e8ca354b00","size":5,"soft_fail":false,"target_va":5384656240,"va":5379727265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::extension
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4929776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379726848,"raw_hex":"e8f0384b00","size":5,"soft_fail":false,"target_va":5384656912,"va":5379727131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::fs::windows::FindNextFileHandle as core::ops::drop::Drop>::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5036628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379726848,"raw_hex":"e854da4c00","size":5,"soft_fail":false,"target_va":5384764464,"va":5379727831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=19
//     evidence={"assembly":"\tcallq\t-4048508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379726848,"raw_hex":"e88439c2ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379727207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140098ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10394280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379726848,"raw_hex":"e8586561ff","size":5,"soft_fail":false,"target_va":5369335008,"va":5379729283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd3910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37033575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379726848,"raw_hex":"e867163502","size":5,"soft_fail":false,"target_va":5416761616,"va":5379728036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd56b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37041323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379726848,"raw_hex":"e8ab343502","size":5,"soft_fail":false,"target_va":5416769200,"va":5379727872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: import.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37330109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379726848,"raw_hex":"e8bd9c3902","size":5,"soft_fail":false,"target_va":5417057056,"va":5379726942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5066901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379726848,"raw_hex":"e895504d00","size":5,"soft_fail":false,"target_va":5384794864,"va":5379727958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t2395083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379726848,"raw_hex":"e8cb8b2400","size":5,"soft_fail":false,"target_va":5382122800,"va":5379727712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::import::copy_folder
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379726848,"raw_hex":"e8b0fdffff","size":5,"soft_fail":false,"target_va":5379726848,"va":5379727435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4996113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379726848,"raw_hex":"e8113c4c00","size":5,"soft_fail":false,"target_va":5384723728,"va":5379727610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::read_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4998245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379726848,"raw_hex":"e865444c00","size":5,"soft_fail":false,"target_va":5384725152,"va":5379726902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":543,"record_offset":26364,"parent_record_offset":20592,"ancestor_record_offset":20592,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":543,"record_offset":26816,"parent_record_offset":20592,"ancestor_record_offset":20592,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":543,"record_offset":20876,"parent_record_offset":20592,"ancestor_record_offset":20592,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":543,"record_offset":21076,"parent_record_offset":20592,"ancestor_record_offset":20592,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":543,"record_offset":20844,"parent_record_offset":20592,"ancestor_record_offset":20592,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":543,"record_offset":26336,"parent_record_offset":20592,"ancestor_record_offset":20592,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":543,"record_offset":22160,"parent_record_offset":20592,"ancestor_record_offset":20592,"inlinee":13260,"source_line":2392,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] std::fs::FileType::is_file
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":543,"record_offset":23076,"parent_record_offset":20592,"ancestor_record_offset":20592,"inlinee":33659,"source_line":2425,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":543,"record_offset":23132,"parent_record_offset":20592,"ancestor_record_offset":20592,"inlinee":8451,"source_line":2880,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":543,"record_offset":22224,"parent_record_offset":20592,"ancestor_record_offset":20592,"inlinee":33646,"source_line":3077,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":543,"record_offset":20776,"parent_record_offset":20592,"ancestor_record_offset":20592,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":543,"record_offset":21444,"parent_record_offset":20592,"ancestor_record_offset":20592,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":21324,"parent_record_offset":21300,"ancestor_record_offset":20592,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":21348,"parent_record_offset":21324,"ancestor_record_offset":20592,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":21372,"parent_record_offset":21348,"ancestor_record_offset":20592,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":21300,"parent_record_offset":21244,"ancestor_record_offset":20592,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":543,"record_offset":26412,"parent_record_offset":26388,"ancestor_record_offset":20592,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":21272,"parent_record_offset":21244,"ancestor_record_offset":20592,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":21244,"parent_record_offset":21220,"ancestor_record_offset":20592,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":21220,"parent_record_offset":21196,"ancestor_record_offset":20592,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":543,"record_offset":24752,"parent_record_offset":24716,"ancestor_record_offset":20592,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":543,"record_offset":24716,"parent_record_offset":24692,"ancestor_record_offset":20592,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":543,"record_offset":20928,"parent_record_offset":20904,"ancestor_record_offset":20592,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":543,"record_offset":22276,"parent_record_offset":22224,"ancestor_record_offset":20592,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":543,"record_offset":26388,"parent_record_offset":26364,"ancestor_record_offset":20592,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=115
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":543,"record_offset":21100,"parent_record_offset":21076,"ancestor_record_offset":20592,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":543,"record_offset":24784,"parent_record_offset":24752,"ancestor_record_offset":20592,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":543,"record_offset":20804,"parent_record_offset":20776,"ancestor_record_offset":20592,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":543,"record_offset":21000,"parent_record_offset":20976,"ancestor_record_offset":20592,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":543,"record_offset":21024,"parent_record_offset":21000,"ancestor_record_offset":20592,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":543,"record_offset":24812,"parent_record_offset":24752,"ancestor_record_offset":20592,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":543,"record_offset":24844,"parent_record_offset":24716,"ancestor_record_offset":20592,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::import::copy_folder::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":543,"record_offset":20904,"parent_record_offset":20876,"ancestor_record_offset":20592,"inlinee":33656,"source_line":130,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$44::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":543,"record_offset":20976,"parent_record_offset":20952,"ancestor_record_offset":20592,"inlinee":14923,"source_line":1498,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$45::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":543,"record_offset":20952,"parent_record_offset":20928,"ancestor_record_offset":20592,"inlinee":33658,"source_line":1506,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":543,"record_offset":22304,"parent_record_offset":22224,"ancestor_record_offset":20592,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":543,"record_offset":22248,"parent_record_offset":22224,"ancestor_record_offset":20592,"inlinee":33647,"source_line":3398,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":543,"record_offset":22188,"parent_record_offset":22160,"ancestor_record_offset":20592,"inlinee":13261,"source_line":1196,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::is_file
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":543,"record_offset":23100,"parent_record_offset":23076,"ancestor_record_offset":20592,"inlinee":33660,"source_line":1199,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::session::import::copy_folder::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8378
// 首条位置: {"module":543,"inlinee":33656}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":543,"record_offset":20928,"parent_record_offset":20904,"ancestor_record_offset":20904,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":543,"record_offset":21000,"parent_record_offset":20976,"ancestor_record_offset":20904,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":543,"record_offset":21024,"parent_record_offset":21000,"ancestor_record_offset":20904,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$44::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":543,"record_offset":20976,"parent_record_offset":20952,"ancestor_record_offset":20904,"inlinee":14923,"source_line":1498,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$45::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":543,"record_offset":20952,"parent_record_offset":20928,"ancestor_record_offset":20904,"inlinee":33658,"source_line":1506,"archive":"_pdb/C/modules/00543.jsonl.gz"}
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
// 完整结构及原始类型记录: import.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::session::import::ImportMenu
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::resources::session::import::ImportMenu>", "constructor": "core::option::Option", "arguments": ["country_core::resources::session::import::ImportMenu"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::resources::session::import::ImportMenu> >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::resources::session::import::ImportMenu>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<country_core::resources::session::import::ImportMenu> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<country_core::resources::session::import::ImportMenu> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::session::import::ImportMenu>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::session::import::ImportMenu"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::session::import::ImportOrigin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::resources::session::import::ImportOrigin>", "constructor": "core::option::Option", "arguments": ["country_core::resources::session::import::ImportOrigin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::resources::session::import::ImportOrigin> >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::resources::session::import::ImportOrigin>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<core::option::Option<country_core::resources::session::import::ImportOrigin> > >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "enum2$<core::option::Option<country_core::resources::session::import::ImportOrigin> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,enum2$<core::option::Option<country_core::resources::session::import::ImportOrigin> > >", "constructor": "serde_json::ser::impl$9::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "enum2$<core::option::Option<country_core::resources::session::import::ImportOrigin> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$16::deserialize<country_core::resources::session::import::ImportOrigin,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >", "constructor": "serde_core::de::impls::impl$16::deserialize", "arguments": ["country_core::resources::session::import::ImportOrigin", "ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::session::import::ImportOrigin,country_core::resources::session::SessionId>", "constructor": "tuple$", "arguments": ["country_core::resources::session::import::ImportOrigin", "country_core::resources::session::SessionId"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<country_core::resources::session::import::ImportOrigin,country_core::resources::session::SessionId>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<country_core::resources::session::import::ImportOrigin,country_core::resources::session::SessionId>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<country_core::resources::session::import::ImportOrigin,country_core::resources::session::SessionId>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<country_core::resources::session::import::ImportOrigin,country_core::resources::session::SessionId>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<enum2$<core::option::Option<country_core::resources::session::import::ImportOrigin> > >", "constructor": "ref$", "arguments": ["enum2$<core::option::Option<country_core::resources::session::import::ImportOrigin> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::to_value<ref$<enum2$<core::option::Option<country_core::resources::session::import::ImportOrigin> > > >", "constructor": "serde_json::value::to_value", "arguments": ["ref$<enum2$<core::option::Option<country_core::resources::session::import::ImportOrigin> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<country_core::resources::session::import::ImportOrigin,country_core::resources::session::SessionId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["tuple$<country_core::resources::session::import::ImportOrigin,country_core::resources::session::SessionId>", "core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$16::deserialize<country_core::resources::session::import::ImportOrigin,enum2$<serde_json::value::Value> >", "constructor": "serde_core::de::impls::impl$16::deserialize", "arguments": ["country_core::resources::session::import::ImportOrigin", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<enum2$<core::option::Option<country_core::resources::session::import::ImportOrigin> > >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["enum2$<core::option::Option<country_core::resources::session::import::ImportOrigin> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::session::import::ImportState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::resources::session::import::ImportState>", "constructor": "enum2$", "arguments": ["country_core::resources::session::import::ImportState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<country_core::resources::session::import::ImportState>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<country_core::resources::session::import::ImportState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<enum2$<country_core::resources::session::import::ImportState>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<enum2$<country_core::resources::session::import::ImportState>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$12::clone<enum2$<country_core::resources::session::import::ImportState>,alloc::alloc::Global>", "constructor": "alloc::vec::impl$12::clone", "arguments": ["enum2$<country_core::resources::session::import::ImportState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<country_core::resources::session::import::ImportState>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["enum2$<country_core::resources::session::import::ImportState>", "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1> >", "constructor": "alloc::vec::impl$21::extend_trusted::closure_env$0", "arguments": ["enum2$<country_core::resources::session::import::ImportState>", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["enum2$<country_core::resources::session::import::ImportState>", "alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<enum2$<country_core::resources::session::import::ImportState>,core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["enum2$<country_core::resources::session::import::ImportState>", "core::slice::iter::Iter<country_core::resources::session::SavedSession>", "country_core::resources::session::import::impl$0::read::closure_env$1", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: import.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: import.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 21; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustClosed>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustClosed>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustClosed>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::FileDroppedOnWindow>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::UninstallModClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_publish::PublishModCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::FileDroppedOnWindow>", "bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::UninstallModClutterCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_publish::PublishModCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::FileDroppedOnWindow>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::UninstallModClutterCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_publish::PublishModCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::DeleteSessionCmd>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::DeleteSessionCmd>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::DeleteSessionCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState> > >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState> > >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>
// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] f32 (*)(f32,f32)
// 候选参数类型: ["f32", "f32"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::ImportMenu::read
// 信息 ID: f870f4609fa9ebfe325dcfa0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::import::entry_ui @ 0x2999c77 -> 0xa817b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::_::impl$0::deserialize
// 信息 ID: 0faa7223a4b26c2c56611220
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impls::impl$16::deserialize<country_core::resources::session::import::ImportOrigin,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=216, inline_record=1092; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impls::impl$16::deserialize<country_core::resources::session::import::ImportOrigin,enum2$<serde_json::value::Value> >; module=1770, inline_record=832; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::session::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=11792; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 138424e1c8e97e2dcda2fa8d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::session::import::_::impl$0::deserialize::__Visitor>; module=31, inline_record=268632; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::session::import::_::impl$0::deserialize::__Field> >; module=1803, inline_record=341112; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: e4b8f7801b62c3b360786a77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::session::import::_::impl$0::deserialize::__Visitor>; module=31, inline_record=268452; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::session::import::_::impl$0::deserialize::__Field> >; module=1803, inline_record=340944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: d1da4a0097aa5e4a5b6a529c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::de::impl$1::fmt<country_core::resources::session::import::_::impl$0::deserialize::__Visitor> @ 0x4b8cf0 -> 0xa83270 (direct_jump_outside_procedure); 同地址别名未裁定。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::session::import::_::impl$0::deserialize::__Visitor>; module=1666, inline_record=72628; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: d04f52e5c02fef05f2a020c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::session::import::_::impl$0::deserialize::__Visitor>; module=31, inline_record=268312; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::session::import::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=183164; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 2aa90a76ce1d555a84e0238c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::session::import::_::impl$0::deserialize::__Visitor>; module=31, inline_record=270872; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::session::import::_::impl$0::deserialize::__Visitor>; module=1803, inline_record=214300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::_::impl$0::serialize
// 信息 ID: 66d1b4bd52fb1ce5fddaa98b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::to_value<ref$<enum2$<core::option::Option<country_core::resources::session::import::ImportOrigin> > > >; module=1636, inline_record=77284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: 95bca855c5452a134b434203
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::session::import::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<core::option::Option<country_core::resources::session::import::ImportOrigin> > > @ 0x45ca9c -> 0x4b87a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<core::option::Option<country_core::resources::session::import::ImportOrigin> > > @ 0x18bc9cc -> 0x17cefa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::copy_folder
// 信息 ID: b69ebb4f012d1b7db42e840d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::session::import::copy_glade @ 0xa81a0a -> 0xa81e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::session::import::copy_folder @ 0xa8204b -> 0xa81e00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::copy_folder::closure$0
// 信息 ID: 6b43318a27d401b17119726d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::session::import::copy_folder", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::session::import::copy_folder; module=543, inline_record=20904; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::copy_glade
// 信息 ID: 5c2ac59afaa04677e43d2636
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::import::ui @ 0x2998195 -> 0xa81940 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::copy_glade::closure$0
// 信息 ID: c62c4f005c077172b3379e5e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::session::import::copy_glade", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::session::import::copy_glade; module=543, inline_record=18696; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::impl$0::read::closure$0
// 信息 ID: 8561748cdef34c9d7e03dddf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::session::import::impl$0::read", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::session::import::impl$0::read::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<country_core::resources::session::import::ImportOrigin,country_core::resources::session::SessionId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::session::import::impl$0::read::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<ref$<country_core::resources::session::SavedSession> >,country_core::resources::session::import::impl$0::read::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<country_core::resources::session::SavedSession> >,country_core::resources::session::import::impl$0::read::closure_env$0>; module=1666, inline_record=81168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::impl$0::read::closure$1
// 信息 ID: 01a8d22994d9cfba5a1086dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::session::import::impl$0::read", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::session::import::impl$0::read::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<country_core::resources::session::import::ImportState>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::session::import::impl$0::read::closure_env$1 <- core::iter::adapters::map::impl$2::fold<enum2$<country_core::resources::session::import::ImportState>,core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::session::import::impl$0::read::closure_env$1 <- core::iter::adapters::map::impl$2::fold<enum2$<country_core::resources::session::import::ImportState>,core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<enum2$<country_core::resources::session::import::ImportState>,core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1> > > >; module=1682, inline_record=62076; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::impl$0::read::closure$1::closure$0
// 信息 ID: 238feb630c237ddd2bbd1c6f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::session::import::impl$0::read", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<enum2$<country_core::resources::session::import::ImportState>,core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1> > > >; module=1682, inline_record=62188; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::impl$0::read::closure$1::closure$1
// 信息 ID: 50628ac7be5f93b099377c73
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::session::import::impl$0::read", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<enum2$<country_core::resources::session::import::ImportState>,core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1> > > >; module=1682, inline_record=63044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::impl$1::clone
// 信息 ID: 2d01b762be9a1b4aca598a0a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_dump::dump_save; module=39, inline_record=40288; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=189, inline_record=7480; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::session::import::copy_glade; module=543, inline_record=19396; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$12::clone<enum2$<country_core::resources::session::import::ImportState>,alloc::alloc::Global>; module=1267, inline_record=15416; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_session::resolve_next_session; module=1681, inline_record=35456; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2893, inline_record=27224; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::mods_glade_tab::apply_button; module=2896, inline_record=4456; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::import::ui; module=4234, inline_record=2760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::impl$4::eq
// 信息 ID: faf6abe14279876d47f0be2f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::session::import::copy_glade; module=543, inline_record=18792; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<enum2$<country_core::resources::session::import::ImportState>,core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1> > > >; module=1682, inline_record=63068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::impl$5::clone
// 信息 ID: 49efcb91ceaa68178382eb5a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$12::clone<enum2$<country_core::resources::session::import::ImportState>,alloc::alloc::Global>; module=1267, inline_record=15344; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::import::ui; module=4234, inline_record=2672; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::impl$6::clone
// 信息 ID: 342d0612f308bfd9bd6205f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::option::Option::cloned<enum2$<country_core::resources::gui::EscMenuState> >; module=1280, inline_record=1632; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::mods::ui; module=2899, inline_record=4732; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::read_origin
// 信息 ID: c8fc4a9d4aca4ebe7f555623
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::session::import::copy_glade @ 0xa81a3b -> 0xa827a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<enum2$<country_core::resources::session::import::ImportState>,core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1> > > > @ 0x180fd1d -> 0xa827a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::source_folder
// 信息 ID: 288b6634171a572d774ac55c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<enum2$<country_core::resources::session::import::ImportState>,core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<country_core::resources::session::import::ImportState>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::session::SavedSession>,country_core::resources::session::import::impl$0::read::closure_env$1> > > > @ 0x180fd07 -> 0xa82ef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::import::ui @ 0x2998141 -> 0xa82ef0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::session::import::source_root
// 信息 ID: 7f56348352a323ca5acb0b5a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::import::entry_ui @ 0x299959b -> 0xa82ee0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::render_loop::session_selection::render_session_selection_previews; module=529, inline_record=17684; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::session::import::ImportMenu::read; module=543, inline_record=16976; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::session::import::source_folder; module=543, inline_record=35400; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
