// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/startup/clearing.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: ff7c5f94e5acd553989b40000ea54e6c0ff0cbe2a8b2a1140676b2f915ab308c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: clearing.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 578, 592, 1643, 1645, 1665
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use frozen; // namespace candidate
//   package identities: [{"name":"frozen","source":"local_or_path","version":"0.1.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use libfmod; // namespace candidate
//   package identities: [{"name":"libfmod","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.222.6"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use relative_path; // namespace candidate
//   package identities: [{"name":"relative-path","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.0.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"},{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"},{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use slotmap; // namespace candidate
//   package identities: [{"name":"slotmap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.1.1"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9386
// 首条位置: {"module":1645,"inlinee":37766}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":592,"record_offset":173432,"rva":12685872,"va":5381394992}
// 编译实例 RVAs: 0xc19230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":592,"record_offset":173616,"parent_record_offset":173432,"ancestor_record_offset":173432,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":592,"record_offset":173636,"parent_record_offset":173616,"ancestor_record_offset":173432,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":592,"record_offset":173656,"parent_record_offset":173636,"ancestor_record_offset":173432,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":592,"record_offset":173680,"parent_record_offset":173656,"ancestor_record_offset":173432,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13E5F
// 首条位置: {"module":1645,"inlinee":81503}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":316236,"parent_record_offset":316212,"ancestor_record_offset":316212,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1645,"record_offset":316384,"parent_record_offset":316260,"ancestor_record_offset":316212,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":316300,"parent_record_offset":316260,"ancestor_record_offset":316212,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":316260,"parent_record_offset":316236,"ancestor_record_offset":316212,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":316324,"parent_record_offset":316300,"ancestor_record_offset":316212,"inlinee":7606,"source_line":693,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":316348,"parent_record_offset":316324,"ancestor_record_offset":316212,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8FFB
// 首条位置: {"module":592,"inlinee":36859}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13E5D
// 首条位置: {"module":1645,"inlinee":81501}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1645,"record_offset":316968,"parent_record_offset":316004,"ancestor_record_offset":316004,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1645,"record_offset":318000,"parent_record_offset":316004,"ancestor_record_offset":316004,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1645,"record_offset":317968,"parent_record_offset":316004,"ancestor_record_offset":316004,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1645,"record_offset":319204,"parent_record_offset":316004,"ancestor_record_offset":316004,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":320640,"parent_record_offset":316004,"ancestor_record_offset":316004,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":316100,"parent_record_offset":316004,"ancestor_record_offset":316004,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":316436,"parent_record_offset":316004,"ancestor_record_offset":316004,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":319560,"parent_record_offset":319536,"ancestor_record_offset":316004,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":319584,"parent_record_offset":319560,"ancestor_record_offset":316004,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":319608,"parent_record_offset":319584,"ancestor_record_offset":316004,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":319536,"parent_record_offset":319512,"ancestor_record_offset":316004,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":320196,"parent_record_offset":320168,"ancestor_record_offset":316004,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":319512,"parent_record_offset":319488,"ancestor_record_offset":316004,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":319488,"parent_record_offset":319464,"ancestor_record_offset":316004,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1645,"record_offset":316384,"parent_record_offset":316260,"ancestor_record_offset":316004,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1645,"record_offset":319368,"parent_record_offset":319344,"ancestor_record_offset":316004,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::clearing::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/clearing.rs
//     evidence={"module":1645,"record_offset":316212,"parent_record_offset":316188,"ancestor_record_offset":316004,"inlinee":81503,"source_line":40,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::relative_path_buf::impl$22::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/relative_path_buf.rs
//     evidence={"module":1645,"record_offset":318560,"parent_record_offset":318536,"ancestor_record_offset":316004,"inlinee":81505,"source_line":524,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1645,"record_offset":319276,"parent_record_offset":319252,"ancestor_record_offset":316004,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":319980,"parent_record_offset":319956,"ancestor_record_offset":316004,"inlinee":7831,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":319252,"parent_record_offset":319228,"ancestor_record_offset":316004,"inlinee":7748,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":319300,"parent_record_offset":319276,"ancestor_record_offset":316004,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1645,"record_offset":316900,"parent_record_offset":316876,"ancestor_record_offset":316004,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":316188,"parent_record_offset":316132,"ancestor_record_offset":316004,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1645,"record_offset":319100,"parent_record_offset":319076,"ancestor_record_offset":316004,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":316300,"parent_record_offset":316260,"ancestor_record_offset":316004,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":316512,"parent_record_offset":316472,"ancestor_record_offset":316004,"inlinee":7597,"source_line":1061,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":316556,"parent_record_offset":316512,"ancestor_record_offset":316004,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":316596,"parent_record_offset":316556,"ancestor_record_offset":316004,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":316132,"parent_record_offset":316100,"ancestor_record_offset":316004,"inlinee":7724,"source_line":1986,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":316472,"parent_record_offset":316436,"ancestor_record_offset":316004,"inlinee":7596,"source_line":2027,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":316260,"parent_record_offset":316236,"ancestor_record_offset":316004,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":316236,"parent_record_offset":316212,"ancestor_record_offset":316004,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$7::deserialize_ignored_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":316924,"parent_record_offset":316900,"ancestor_record_offset":316004,"inlinee":7602,"source_line":1910,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":316324,"parent_record_offset":316300,"ancestor_record_offset":316004,"inlinee":7606,"source_line":693,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":316624,"parent_record_offset":316596,"ancestor_record_offset":316004,"inlinee":7605,"source_line":688,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":316348,"parent_record_offset":316324,"ancestor_record_offset":316004,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":316652,"parent_record_offset":316624,"ancestor_record_offset":316004,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13E62
// 首条位置: {"module":1645,"inlinee":81506}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1645,"record_offset":322588,"parent_record_offset":321996,"ancestor_record_offset":321996,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":322064,"parent_record_offset":321996,"ancestor_record_offset":321996,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":322780,"parent_record_offset":322756,"ancestor_record_offset":321996,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":322804,"parent_record_offset":322780,"ancestor_record_offset":321996,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":322828,"parent_record_offset":322804,"ancestor_record_offset":321996,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":322756,"parent_record_offset":322732,"ancestor_record_offset":321996,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":323044,"parent_record_offset":323016,"ancestor_record_offset":321996,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":322732,"parent_record_offset":322708,"ancestor_record_offset":321996,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":322708,"parent_record_offset":322684,"ancestor_record_offset":321996,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1645,"record_offset":322612,"parent_record_offset":322588,"ancestor_record_offset":321996,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::relative_path_buf::impl$22::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/relative_path_buf.rs
//     evidence={"module":1645,"record_offset":322336,"parent_record_offset":322312,"ancestor_record_offset":321996,"inlinee":81505,"source_line":524,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":322160,"parent_record_offset":322092,"ancestor_record_offset":321996,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1645,"record_offset":322184,"parent_record_offset":322160,"ancestor_record_offset":321996,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":322092,"parent_record_offset":322064,"ancestor_record_offset":321996,"inlinee":7609,"source_line":1933,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::init_clearing
// 记录形式: compiled_procedure; instances=1
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":578,"record_offset":23844,"rva":12218624,"va":5380927744}
// 编译实例 RVAs: 0xba7100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t3726886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e826de3800","size":5,"soft_fail":false,"target_va":5384656240,"va":5380929349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3681352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e8482c3800","size":5,"soft_fail":false,"target_va":5384609408,"va":5380928051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..raw..respool..HandleRef$LT$rhapsody..raw..buffer..BufferSchema$GT$$GT$::cold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2528653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e88d952600","size":5,"soft_fail":false,"target_va":5383464432,"va":5380935774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5256000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e8c0ccafff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380934683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=28
//     evidence={"assembly":"\tcallq\t-5252275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e84ddbafff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380930974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12212477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e803a745ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380934664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ae10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11601271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e889fa4eff","size":5,"soft_fail":false,"target_va":5369343504,"va":5380944770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140870fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3384497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e84f5bccff","size":5,"soft_fail":false,"target_va":5377560512,"va":5380945004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096eb80
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2338496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e84051dcff","size":5,"soft_fail":false,"target_va":5378599808,"va":5380938299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140972520
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2320943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e8d195dcff","size":5,"soft_fail":false,"target_va":5378614560,"va":5380935498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990120
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2198605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e8b373deff","size":5,"soft_fail":false,"target_va":5378736416,"va":5380935016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a92b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2101861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e89beddfff","size":5,"soft_fail":false,"target_va":5378839216,"va":5380941072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409e61f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1845349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e89bd7e3ff","size":5,"soft_fail":false,"target_va":5379088880,"va":5380934224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409e64e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1844808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e8b8d9e3ff","size":5,"soft_fail":false,"target_va":5379089632,"va":5380934435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b366f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-463022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e852eff8ff","size":5,"soft_fail":false,"target_va":5380466416,"va":5380929433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b992e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-62607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e8710bffff","size":5,"soft_fail":false,"target_va":5380870880,"va":5380933482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c19320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t460085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e835050700","size":5,"soft_fail":false,"target_va":5381395232,"va":5380935142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2389130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e88a742400","size":5,"soft_fail":false,"target_va":5383324784,"va":5380935649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415fa810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10820894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e81e1da500","size":5,"soft_fail":false,"target_va":5391755280,"va":5380934381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162c5d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11027061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e87542a800","size":5,"soft_fail":false,"target_va":5391959504,"va":5380932438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416305c0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11047416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e8f891a800","size":5,"soft_fail":false,"target_va":5391975872,"va":5380928451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416318f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11051507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e8f3a1a800","size":5,"soft_fail":false,"target_va":5391980784,"va":5380929272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141651af0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11178833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e85193aa00","size":5,"soft_fail":false,"target_va":5392112368,"va":5380933530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14181c970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13057765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e8e53ec700","size":5,"soft_fail":false,"target_va":5393992048,"va":5380934278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141aed3d0
//     owner=unresolved_namespace; count=6
//     target_aliases=3 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16015712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e86061f400","size":5,"soft_fail":false,"target_va":5396943824,"va":5380928107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141aedab0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16011128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e8784ff400","size":5,"soft_fail":false,"target_va":5396945584,"va":5380934451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b2c520
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16270676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e85445f800","size":5,"soft_fail":false,"target_va":5397202208,"va":5380931527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b2e140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16273121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e8e14ef800","size":5,"soft_fail":false,"target_va":5397209408,"va":5380936282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b2e830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16282083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e8e371f800","size":5,"soft_fail":false,"target_va":5397211184,"va":5380929096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35423283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e833841c02","size":5,"soft_fail":false,"target_va":5416353040,"va":5380929752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36128133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e885452702","size":5,"soft_fail":false,"target_va":5417057696,"va":5380929558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36128458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e8ca462702","size":5,"soft_fail":false,"target_va":5417058032,"va":5380929569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35450303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e8bfed1c02","size":5,"soft_fail":false,"target_va":5416381232,"va":5380930924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35685313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e8c1832002","size":5,"soft_fail":false,"target_va":5416620794,"va":5380935476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-4984771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e83df0b3ff","size":5,"soft_fail":false,"target_va":5375949040,"va":5380933806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3686489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e859403800","size":5,"soft_fail":false,"target_va":5384624080,"va":5380937586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::reserve_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3296587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e84b4d3200","size":5,"soft_fail":false,"target_va":5384226192,"va":5380929600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t35684979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e873822002","size":5,"soft_fail":false,"target_va":5416621632,"va":5380936648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_world_raster_pipelines
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1406573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e89389eaff","size":5,"soft_fail":false,"target_va":5379521760,"va":5380928328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name_hash
//     owner=package_namespace_candidate; count=18
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1649919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e801d3e6ff","size":5,"soft_fail":false,"target_va":5379278928,"va":5380928842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions_without_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t530808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e878190800","size":5,"soft_fail":false,"target_va":5381465408,"va":5380934595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mesh_into_library
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1278619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e8657decff","size":5,"soft_fail":false,"target_va":5379649664,"va":5380928278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::update_buffer_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t2784016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e8107b2a00","size":5,"soft_fail":false,"target_va":5383719888,"va":5380935867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2389616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e870762400","size":5,"soft_fail":false,"target_va":5383320512,"va":5380930891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=package_namespace_candidate; count=18
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2395908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380927744,"raw_hex":"e8048f2400","size":5,"soft_fail":false,"target_va":5383324736,"va":5380928823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5380927744,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5380935726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$93::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":578,"record_offset":34056,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":21307,"source_line":2666,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":32984,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":33840,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":34508,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":578,"record_offset":33896,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":578,"record_offset":24588,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::remove_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":578,"record_offset":31920,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":11390,"source_line":861,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":578,"record_offset":40616,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":578,"record_offset":24176,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":578,"record_offset":34636,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":578,"record_offset":32332,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":578,"record_offset":32216,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":578,"record_offset":24428,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":578,"record_offset":34424,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":578,"record_offset":36788,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":578,"record_offset":39216,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":26382,"source_line":294,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":578,"record_offset":40776,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":12354,"source_line":298,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":578,"record_offset":24344,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":27991,"source_line":42,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":578,"record_offset":40436,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":27015,"source_line":168,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::assets_root
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":578,"record_offset":31792,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":8070,"source_line":291,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_flowery
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":578,"record_offset":24532,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":36802,"source_line":311,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_olden
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":578,"record_offset":31728,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":26709,"source_line":306,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_winter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":578,"record_offset":24372,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":16149,"source_line":296,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::tree_storage::InteractableForestPlacements::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/tree_storage.rs
//     evidence={"module":578,"record_offset":33072,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":36812,"source_line":128,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::glade_themes::flowery::startup_flowery
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/glade_themes/flowery.rs
//     evidence={"module":578,"record_offset":24648,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":36804,"source_line":18,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::glade_themes::olden::startup_olden
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/glade_themes/olden.rs
//     evidence={"module":578,"record_offset":27372,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":36806,"source_line":18,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":578,"record_offset":24232,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":578,"record_offset":24092,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":578,"record_offset":32188,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::identity
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":578,"record_offset":24504,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":27995,"source_line":15,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":35288,"parent_record_offset":35264,"ancestor_record_offset":23844,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":35312,"parent_record_offset":35288,"ancestor_record_offset":23844,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":25964,"parent_record_offset":25940,"ancestor_record_offset":23844,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":25988,"parent_record_offset":25964,"ancestor_record_offset":23844,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":35336,"parent_record_offset":35312,"ancestor_record_offset":23844,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":26012,"parent_record_offset":25988,"ancestor_record_offset":23844,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":35264,"parent_record_offset":35108,"ancestor_record_offset":23844,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":25940,"parent_record_offset":25852,"ancestor_record_offset":23844,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":28184,"parent_record_offset":28152,"ancestor_record_offset":23844,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":25604,"parent_record_offset":25576,"ancestor_record_offset":23844,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":35036,"parent_record_offset":35000,"ancestor_record_offset":23844,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":25688,"parent_record_offset":25664,"ancestor_record_offset":23844,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":25880,"parent_record_offset":25852,"ancestor_record_offset":23844,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":25852,"parent_record_offset":25828,"ancestor_record_offset":23844,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":25664,"parent_record_offset":25632,"ancestor_record_offset":23844,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":28232,"parent_record_offset":28208,"ancestor_record_offset":23844,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":28208,"parent_record_offset":28184,"ancestor_record_offset":23844,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":25632,"parent_record_offset":25604,"ancestor_record_offset":23844,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":35108,"parent_record_offset":35072,"ancestor_record_offset":23844,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":35072,"parent_record_offset":35036,"ancestor_record_offset":23844,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":25828,"parent_record_offset":25804,"ancestor_record_offset":23844,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":35152,"parent_record_offset":35108,"ancestor_record_offset":23844,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":578,"record_offset":33436,"parent_record_offset":33396,"ancestor_record_offset":23844,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":578,"record_offset":33396,"parent_record_offset":33368,"ancestor_record_offset":23844,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":28152,"parent_record_offset":28108,"ancestor_record_offset":23844,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":33144,"parent_record_offset":33108,"ancestor_record_offset":23844,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":33108,"parent_record_offset":33072,"ancestor_record_offset":23844,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":36952,"parent_record_offset":36928,"ancestor_record_offset":23844,"inlinee":10726,"source_line":3961,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":35424,"parent_record_offset":35396,"ancestor_record_offset":23844,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":37144,"parent_record_offset":36952,"ancestor_record_offset":23844,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":28080,"parent_record_offset":27372,"ancestor_record_offset":23844,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":28108,"parent_record_offset":28080,"ancestor_record_offset":23844,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":25576,"parent_record_offset":25548,"ancestor_record_offset":23844,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":37384,"parent_record_offset":36952,"ancestor_record_offset":23844,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":34964,"parent_record_offset":34744,"ancestor_record_offset":23844,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":35000,"parent_record_offset":34964,"ancestor_record_offset":23844,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":34672,"parent_record_offset":34636,"ancestor_record_offset":23844,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":35668,"parent_record_offset":35636,"ancestor_record_offset":23844,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":578,"record_offset":35744,"parent_record_offset":35668,"ancestor_record_offset":23844,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":578,"record_offset":36928,"parent_record_offset":36876,"ancestor_record_offset":23844,"inlinee":10725,"source_line":17,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":578,"record_offset":35396,"parent_record_offset":34744,"ancestor_record_offset":23844,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":578,"record_offset":34708,"parent_record_offset":34672,"ancestor_record_offset":23844,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":578,"record_offset":34744,"parent_record_offset":34708,"ancestor_record_offset":23844,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":578,"record_offset":24616,"parent_record_offset":24588,"ancestor_record_offset":23844,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":578,"record_offset":25520,"parent_record_offset":25492,"ancestor_record_offset":23844,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":578,"record_offset":25548,"parent_record_offset":25520,"ancestor_record_offset":23844,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":578,"record_offset":25460,"parent_record_offset":24648,"ancestor_record_offset":23844,"inlinee":11391,"source_line":380,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":578,"record_offset":25492,"parent_record_offset":25460,"ancestor_record_offset":23844,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":578,"record_offset":35176,"parent_record_offset":35152,"ancestor_record_offset":23844,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":578,"record_offset":27636,"parent_record_offset":27372,"ancestor_record_offset":23844,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":578,"record_offset":27660,"parent_record_offset":27636,"ancestor_record_offset":23844,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":578,"record_offset":27716,"parent_record_offset":27684,"ancestor_record_offset":23844,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":578,"record_offset":34104,"parent_record_offset":34080,"ancestor_record_offset":23844,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":578,"record_offset":28772,"parent_record_offset":28748,"ancestor_record_offset":23844,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":578,"record_offset":39696,"parent_record_offset":39672,"ancestor_record_offset":23844,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":578,"record_offset":39780,"parent_record_offset":39756,"ancestor_record_offset":23844,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":578,"record_offset":29208,"parent_record_offset":29152,"ancestor_record_offset":23844,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":578,"record_offset":28804,"parent_record_offset":28724,"ancestor_record_offset":23844,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":578,"record_offset":28748,"parent_record_offset":28724,"ancestor_record_offset":23844,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":578,"record_offset":29564,"parent_record_offset":29540,"ancestor_record_offset":23844,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":578,"record_offset":40256,"parent_record_offset":39592,"ancestor_record_offset":23844,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":578,"record_offset":35224,"parent_record_offset":35200,"ancestor_record_offset":23844,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":578,"record_offset":35576,"parent_record_offset":35536,"ancestor_record_offset":23844,"inlinee":10907,"source_line":36,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":578,"record_offset":35508,"parent_record_offset":35480,"ancestor_record_offset":23844,"inlinee":10905,"source_line":77,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":578,"record_offset":34820,"parent_record_offset":34796,"ancestor_record_offset":23844,"inlinee":10972,"source_line":64,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":578,"record_offset":36996,"parent_record_offset":36952,"ancestor_record_offset":23844,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":578,"record_offset":35480,"parent_record_offset":35452,"ancestor_record_offset":23844,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":578,"record_offset":34796,"parent_record_offset":34744,"ancestor_record_offset":23844,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":578,"record_offset":35604,"parent_record_offset":35576,"ancestor_record_offset":23844,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":578,"record_offset":34532,"parent_record_offset":34508,"ancestor_record_offset":23844,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":578,"record_offset":35452,"parent_record_offset":35424,"ancestor_record_offset":23844,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":578,"record_offset":35636,"parent_record_offset":35604,"ancestor_record_offset":23844,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":578,"record_offset":35200,"parent_record_offset":35176,"ancestor_record_offset":23844,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":578,"record_offset":25908,"parent_record_offset":25880,"ancestor_record_offset":23844,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":578,"record_offset":39928,"parent_record_offset":39900,"ancestor_record_offset":23844,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":578,"record_offset":39900,"parent_record_offset":39872,"ancestor_record_offset":23844,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":578,"record_offset":28484,"parent_record_offset":28460,"ancestor_record_offset":23844,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":578,"record_offset":27684,"parent_record_offset":27660,"ancestor_record_offset":23844,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":578,"record_offset":24200,"parent_record_offset":24176,"ancestor_record_offset":23844,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":578,"record_offset":34916,"parent_record_offset":34892,"ancestor_record_offset":23844,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":578,"record_offset":27800,"parent_record_offset":27740,"ancestor_record_offset":23844,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":578,"record_offset":39720,"parent_record_offset":39696,"ancestor_record_offset":23844,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=109
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":578,"record_offset":24472,"parent_record_offset":24428,"ancestor_record_offset":23844,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":578,"record_offset":33864,"parent_record_offset":33840,"ancestor_record_offset":23844,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":578,"record_offset":34892,"parent_record_offset":34868,"ancestor_record_offset":23844,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":578,"record_offset":40048,"parent_record_offset":40024,"ancestor_record_offset":23844,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":578,"record_offset":33968,"parent_record_offset":33896,"ancestor_record_offset":23844,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":578,"record_offset":33468,"parent_record_offset":33436,"ancestor_record_offset":23844,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":578,"record_offset":34868,"parent_record_offset":34844,"ancestor_record_offset":23844,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":578,"record_offset":33996,"parent_record_offset":33896,"ancestor_record_offset":23844,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":578,"record_offset":25240,"parent_record_offset":25212,"ancestor_record_offset":23844,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":578,"record_offset":27824,"parent_record_offset":27800,"ancestor_record_offset":23844,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":578,"record_offset":28272,"parent_record_offset":28108,"ancestor_record_offset":23844,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":578,"record_offset":40572,"parent_record_offset":40468,"ancestor_record_offset":23844,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":578,"record_offset":24856,"parent_record_offset":24648,"ancestor_record_offset":23844,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":578,"record_offset":34128,"parent_record_offset":34104,"ancestor_record_offset":23844,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":578,"record_offset":34156,"parent_record_offset":34128,"ancestor_record_offset":23844,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":578,"record_offset":27740,"parent_record_offset":27716,"ancestor_record_offset":23844,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":578,"record_offset":40516,"parent_record_offset":40468,"ancestor_record_offset":23844,"inlinee":12073,"source_line":136,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":578,"record_offset":27772,"parent_record_offset":27740,"ancestor_record_offset":23844,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":578,"record_offset":34448,"parent_record_offset":34424,"ancestor_record_offset":23844,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":578,"record_offset":37020,"parent_record_offset":36996,"ancestor_record_offset":23844,"inlinee":6772,"source_line":360,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":578,"record_offset":35536,"parent_record_offset":35508,"ancestor_record_offset":23844,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":578,"record_offset":25212,"parent_record_offset":24648,"ancestor_record_offset":23844,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":578,"record_offset":34844,"parent_record_offset":34820,"ancestor_record_offset":23844,"inlinee":6665,"source_line":218,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":578,"record_offset":34080,"parent_record_offset":34056,"ancestor_record_offset":23844,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":578,"record_offset":39384,"parent_record_offset":39360,"ancestor_record_offset":23844,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":578,"record_offset":24116,"parent_record_offset":24092,"ancestor_record_offset":23844,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":578,"record_offset":24140,"parent_record_offset":24116,"ancestor_record_offset":23844,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":578,"record_offset":33496,"parent_record_offset":33436,"ancestor_record_offset":23844,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":578,"record_offset":33528,"parent_record_offset":33396,"ancestor_record_offset":23844,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_mut_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":578,"record_offset":39248,"parent_record_offset":39216,"ancestor_record_offset":23844,"inlinee":26383,"source_line":253,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$18::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":578,"record_offset":40212,"parent_record_offset":40188,"ancestor_record_offset":23844,"inlinee":15350,"source_line":139,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name
//     owner=private_source_ownership_candidate; count=17
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":578,"record_offset":24932,"parent_record_offset":24648,"ancestor_record_offset":23844,"inlinee":12354,"source_line":298,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":578,"record_offset":40468,"parent_record_offset":40436,"ancestor_record_offset":23844,"inlinee":26713,"source_line":215,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::placement_data::PlacementDataSsbo::to_transform
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/placement_data.rs
//     evidence={"module":578,"record_offset":25276,"parent_record_offset":24648,"ancestor_record_offset":23844,"inlinee":28807,"source_line":75,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::preplaced_autoclutter::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/preplaced_autoclutter.rs
//     evidence={"module":578,"record_offset":27608,"parent_record_offset":27372,"ancestor_record_offset":23844,"inlinee":36808,"source_line":5,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::ActiveSession::assets_root
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":578,"record_offset":25072,"parent_record_offset":24648,"ancestor_record_offset":23844,"inlinee":8070,"source_line":291,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::assets_folder
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":578,"record_offset":25120,"parent_record_offset":25096,"ancestor_record_offset":23844,"inlinee":27998,"source_line":52,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_flowery
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":578,"record_offset":24556,"parent_record_offset":24532,"ancestor_record_offset":23844,"inlinee":35101,"source_line":101,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_olden
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":578,"record_offset":31756,"parent_record_offset":31728,"ancestor_record_offset":23844,"inlinee":35103,"source_line":93,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_winter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":578,"record_offset":24396,"parent_record_offset":24372,"ancestor_record_offset":23844,"inlinee":27994,"source_line":117,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::SessionMeta::assets_root
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":578,"record_offset":25096,"parent_record_offset":25072,"ancestor_record_offset":23844,"inlinee":27997,"source_line":208,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] frozen::FrozenMap::as_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"frozen","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/frozen/src/lib.rs
//     evidence={"module":578,"record_offset":39284,"parent_record_offset":39248,"ancestor_record_offset":23844,"inlinee":26384,"source_line":114,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":578,"record_offset":25336,"parent_record_offset":25308,"ancestor_record_offset":23844,"inlinee":11738,"source_line":182,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":578,"record_offset":28552,"parent_record_offset":28524,"ancestor_record_offset":23844,"inlinee":27094,"source_line":226,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::mul_vec4
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":578,"record_offset":29124,"parent_record_offset":27372,"ancestor_record_offset":23844,"inlinee":20184,"source_line":1357,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::quat_to_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":578,"record_offset":28588,"parent_record_offset":28552,"ancestor_record_offset":23844,"inlinee":27095,"source_line":192,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_axis_angle
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":578,"record_offset":25308,"parent_record_offset":25276,"ancestor_record_offset":23844,"inlinee":26854,"source_line":146,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":578,"record_offset":25424,"parent_record_offset":25308,"ancestor_record_offset":23844,"inlinee":11856,"source_line":81,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":578,"record_offset":28636,"parent_record_offset":28588,"ancestor_record_offset":23844,"inlinee":11831,"source_line":110,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$103::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":578,"record_offset":29060,"parent_record_offset":28552,"ancestor_record_offset":23844,"inlinee":27101,"source_line":1962,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$19::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":578,"record_offset":29152,"parent_record_offset":29124,"ancestor_record_offset":23844,"inlinee":36810,"source_line":1280,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":578,"record_offset":28724,"parent_record_offset":28552,"ancestor_record_offset":23844,"inlinee":7188,"source_line":1326,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":578,"record_offset":29508,"parent_record_offset":29476,"ancestor_record_offset":23844,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":578,"record_offset":29476,"parent_record_offset":29444,"ancestor_record_offset":23844,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":578,"record_offset":29444,"parent_record_offset":29420,"ancestor_record_offset":23844,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":578,"record_offset":29420,"parent_record_offset":27372,"ancestor_record_offset":23844,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":578,"record_offset":28460,"parent_record_offset":28344,"ancestor_record_offset":23844,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":578,"record_offset":39840,"parent_record_offset":39812,"ancestor_record_offset":23844,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":578,"record_offset":39872,"parent_record_offset":39840,"ancestor_record_offset":23844,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":578,"record_offset":39812,"parent_record_offset":39592,"ancestor_record_offset":23844,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":578,"record_offset":39672,"parent_record_offset":39592,"ancestor_record_offset":23844,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":578,"record_offset":40284,"parent_record_offset":39592,"ancestor_record_offset":23844,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":578,"record_offset":39756,"parent_record_offset":39592,"ancestor_record_offset":23844,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":578,"record_offset":40164,"parent_record_offset":40140,"ancestor_record_offset":23844,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":578,"record_offset":39516,"parent_record_offset":39492,"ancestor_record_offset":23844,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":578,"record_offset":40140,"parent_record_offset":39972,"ancestor_record_offset":23844,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":40080,"parent_record_offset":39996,"ancestor_record_offset":23844,"inlinee":8504,"source_line":435,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":40024,"parent_record_offset":39996,"ancestor_record_offset":23844,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":40376,"parent_record_offset":39592,"ancestor_record_offset":23844,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":39996,"parent_record_offset":39972,"ancestor_record_offset":23844,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":39568,"parent_record_offset":39544,"ancestor_record_offset":23844,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":39544,"parent_record_offset":39516,"ancestor_record_offset":23844,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":39592,"parent_record_offset":39568,"ancestor_record_offset":23844,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":39644,"parent_record_offset":39592,"ancestor_record_offset":23844,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":39972,"parent_record_offset":39592,"ancestor_record_offset":23844,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":578,"record_offset":24280,"parent_record_offset":24256,"ancestor_record_offset":23844,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":578,"record_offset":24256,"parent_record_offset":24232,"ancestor_record_offset":23844,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":578,"record_offset":24304,"parent_record_offset":24280,"ancestor_record_offset":23844,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::BufferHandleExt::desc
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":578,"record_offset":40544,"parent_record_offset":40468,"ancestor_record_offset":23844,"inlinee":26714,"source_line":109,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":578,"record_offset":39492,"parent_record_offset":39248,"ancestor_record_offset":23844,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":578,"record_offset":25384,"parent_record_offset":25360,"ancestor_record_offset":23844,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":578,"record_offset":28420,"parent_record_offset":28396,"ancestor_record_offset":23844,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":578,"record_offset":25360,"parent_record_offset":25336,"ancestor_record_offset":23844,"inlinee":11739,"source_line":910,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":578,"record_offset":29540,"parent_record_offset":29508,"ancestor_record_offset":23844,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":578,"record_offset":25148,"parent_record_offset":25096,"ancestor_record_offset":23844,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":578,"record_offset":39336,"parent_record_offset":39312,"ancestor_record_offset":23844,"inlinee":4343,"source_line":112,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":578,"record_offset":39360,"parent_record_offset":39336,"ancestor_record_offset":23844,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLock::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\rwlock.rs
//     evidence={"module":578,"record_offset":39312,"parent_record_offset":39284,"ancestor_record_offset":23844,"inlinee":4341,"source_line":678,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] utils::components::transform::Transform::compute_matrix
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":578,"record_offset":28524,"parent_record_offset":27372,"ancestor_record_offset":23844,"inlinee":27093,"source_line":24,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::init_clearing::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x936F
// 首条位置: {"module":592,"inlinee":37743}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::impl$8::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":1643,"record_offset":298768,"parent_record_offset":298700,"ancestor_record_offset":298700,"inlinee":28802,"source_line":10,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1643,"record_offset":298740,"parent_record_offset":298700,"ancestor_record_offset":298700,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1643,"record_offset":298796,"parent_record_offset":298768,"ancestor_record_offset":298700,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1643,"record_offset":298832,"parent_record_offset":298796,"ancestor_record_offset":298700,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1643,"record_offset":298856,"parent_record_offset":298832,"ancestor_record_offset":298700,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::init_clearing::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9370
// 首条位置: {"module":592,"inlinee":37744}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::placement_data::PlacementDataSsbo::to_transform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/placement_data.rs
//     evidence={"module":592,"record_offset":174924,"parent_record_offset":174764,"ancestor_record_offset":174764,"inlinee":28807,"source_line":75,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine3a::Affine3A::transform_point3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":592,"record_offset":175580,"parent_record_offset":174764,"ancestor_record_offset":174764,"inlinee":27157,"source_line":392,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":592,"record_offset":174804,"parent_record_offset":174764,"ancestor_record_offset":174764,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":592,"record_offset":175132,"parent_record_offset":174764,"ancestor_record_offset":174764,"inlinee":26436,"source_line":29,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":592,"record_offset":175396,"parent_record_offset":175372,"ancestor_record_offset":174764,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":592,"record_offset":175748,"parent_record_offset":175620,"ancestor_record_offset":174764,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":592,"record_offset":175428,"parent_record_offset":175348,"ancestor_record_offset":174764,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":592,"record_offset":175372,"parent_record_offset":175348,"ancestor_record_offset":174764,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":592,"record_offset":174884,"parent_record_offset":174860,"ancestor_record_offset":174764,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":592,"record_offset":175092,"parent_record_offset":175068,"ancestor_record_offset":174764,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":592,"record_offset":175156,"parent_record_offset":175132,"ancestor_record_offset":174764,"inlinee":26438,"source_line":253,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":592,"record_offset":174980,"parent_record_offset":174952,"ancestor_record_offset":174764,"inlinee":11738,"source_line":182,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":592,"record_offset":175188,"parent_record_offset":175156,"ancestor_record_offset":174764,"inlinee":26440,"source_line":278,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_axis_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":592,"record_offset":174952,"parent_record_offset":174924,"ancestor_record_offset":174764,"inlinee":26854,"source_line":146,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":592,"record_offset":175260,"parent_record_offset":175188,"ancestor_record_offset":174764,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":592,"record_offset":175348,"parent_record_offset":175156,"ancestor_record_offset":174764,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$35::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":592,"record_offset":176028,"parent_record_offset":175580,"ancestor_record_offset":174764,"inlinee":27159,"source_line":1577,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":592,"record_offset":174828,"parent_record_offset":174804,"ancestor_record_offset":174764,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":592,"record_offset":175068,"parent_record_offset":174952,"ancestor_record_offset":174764,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":592,"record_offset":175028,"parent_record_offset":175004,"ancestor_record_offset":174764,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":592,"record_offset":175004,"parent_record_offset":174980,"ancestor_record_offset":174764,"inlinee":11739,"source_line":910,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":592,"record_offset":174860,"parent_record_offset":174828,"ancestor_record_offset":174764,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00592.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::uninit_clearing
// 记录形式: compiled_procedure; instances=1
// 源码行: 255
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":578,"record_offset":44172,"rva":12236464,"va":5380945584}
// 编译实例 RVAs: 0xbab6b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5267781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380945584,"raw_hex":"e8bb9eafff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380946464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5268073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380945584,"raw_hex":"e8979dafff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380946772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12224262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380945584,"raw_hex":"e8fa7845ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380946449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b45a0
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11499420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380945584,"raw_hex":"e8648850ff","size":5,"soft_fail":false,"target_va":5369447840,"va":5380947255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409ab250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2099436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380945584,"raw_hex":"e814f7dfff","size":5,"soft_fail":false,"target_va":5378847312,"va":5380946743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7cbd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1241257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380945584,"raw_hex":"e8570fedff","size":5,"soft_fail":false,"target_va":5379705808,"va":5380947060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9d9e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t991893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380945584,"raw_hex":"e895220f00","size":5,"soft_fail":false,"target_va":5381937632,"va":5380945734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca7da0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1033883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380945584,"raw_hex":"e89bc60f00","size":5,"soft_fail":false,"target_va":5381979552,"va":5380945664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca7f00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1034223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380945584,"raw_hex":"e8efc70f00","size":5,"soft_fail":false,"target_va":5381979904,"va":5380945676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca83b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1035339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380945584,"raw_hex":"e84bcc0f00","size":5,"soft_fail":false,"target_va":5381981104,"va":5380945760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36111277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380945584,"raw_hex":"e8ad032702","size":5,"soft_fail":false,"target_va":5417057088,"va":5380945806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35673874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380945584,"raw_hex":"e812572002","size":5,"soft_fail":false,"target_va":5416620794,"va":5380946915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4996978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380945584,"raw_hex":"e88ec0b3ff","size":5,"soft_fail":false,"target_va":5375949040,"va":5380946013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3676779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380945584,"raw_hex":"e86b1a3800","size":5,"soft_fail":false,"target_va":5384624080,"va":5380947296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35675189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380945584,"raw_hex":"e8355c2002","size":5,"soft_fail":false,"target_va":5416621632,"va":5380946438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCoreInner::instantiate_audio
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1261585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380945584,"raw_hex":"e8efbfecff","size":5,"soft_fail":false,"target_va":5379684800,"va":5380946380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::start
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15707680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380945584,"raw_hex":"e820aeef00","size":5,"soft_fail":false,"target_va":5396654304,"va":5380946619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":578,"record_offset":44348,"parent_record_offset":44172,"ancestor_record_offset":44172,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":578,"record_offset":46868,"parent_record_offset":44172,"ancestor_record_offset":44172,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":578,"record_offset":47768,"parent_record_offset":44172,"ancestor_record_offset":44172,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":578,"record_offset":47524,"parent_record_offset":44172,"ancestor_record_offset":44172,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":578,"record_offset":46752,"parent_record_offset":44172,"ancestor_record_offset":44172,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/audio_trigger_library.rs
//     evidence={"module":578,"record_offset":44600,"parent_record_offset":44172,"ancestor_record_offset":44172,"inlinee":32639,"source_line":76,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/placement_data_library.rs
//     evidence={"module":578,"record_offset":44404,"parent_record_offset":44172,"ancestor_record_offset":44172,"inlinee":32107,"source_line":41,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorld::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":578,"record_offset":45116,"parent_record_offset":44172,"ancestor_record_offset":44172,"inlinee":30910,"source_line":795,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::instantiate_audio
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":578,"record_offset":46924,"parent_record_offset":44172,"ancestor_record_offset":44172,"inlinee":29635,"source_line":321,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::start
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":578,"record_offset":47600,"parent_record_offset":44172,"ancestor_record_offset":44172,"inlinee":33513,"source_line":505,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::rt::RtWorld::reset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/mod.rs
//     evidence={"module":578,"record_offset":44968,"parent_record_offset":44172,"ancestor_record_offset":44172,"inlinee":29737,"source_line":156,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::tree_storage::InteractableForestPlacements::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/tree_storage.rs
//     evidence={"module":578,"record_offset":45940,"parent_record_offset":44172,"ancestor_record_offset":44172,"inlinee":36812,"source_line":128,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":47312,"parent_record_offset":47288,"ancestor_record_offset":44172,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":47336,"parent_record_offset":47312,"ancestor_record_offset":44172,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":48000,"parent_record_offset":47976,"ancestor_record_offset":44172,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":48024,"parent_record_offset":48000,"ancestor_record_offset":44172,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":47360,"parent_record_offset":47336,"ancestor_record_offset":44172,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":48048,"parent_record_offset":48024,"ancestor_record_offset":44172,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":47288,"parent_record_offset":47260,"ancestor_record_offset":44172,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":47976,"parent_record_offset":47920,"ancestor_record_offset":44172,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":46060,"parent_record_offset":46036,"ancestor_record_offset":44172,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":47200,"parent_record_offset":47172,"ancestor_record_offset":44172,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":47948,"parent_record_offset":47920,"ancestor_record_offset":44172,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":47920,"parent_record_offset":47896,"ancestor_record_offset":44172,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":46108,"parent_record_offset":46084,"ancestor_record_offset":44172,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":46084,"parent_record_offset":46060,"ancestor_record_offset":44172,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":47260,"parent_record_offset":47228,"ancestor_record_offset":44172,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":47228,"parent_record_offset":47200,"ancestor_record_offset":44172,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":47896,"parent_record_offset":47872,"ancestor_record_offset":44172,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":578,"record_offset":47080,"parent_record_offset":47052,"ancestor_record_offset":44172,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":578,"record_offset":47108,"parent_record_offset":47080,"ancestor_record_offset":44172,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":578,"record_offset":47136,"parent_record_offset":47108,"ancestor_record_offset":44172,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":578,"record_offset":47052,"parent_record_offset":47024,"ancestor_record_offset":44172,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":578,"record_offset":47024,"parent_record_offset":46924,"ancestor_record_offset":44172,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":578,"record_offset":46304,"parent_record_offset":46264,"ancestor_record_offset":44172,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":578,"record_offset":46996,"parent_record_offset":46924,"ancestor_record_offset":44172,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":578,"record_offset":46264,"parent_record_offset":46236,"ancestor_record_offset":44172,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":46036,"parent_record_offset":46012,"ancestor_record_offset":44172,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":46012,"parent_record_offset":45976,"ancestor_record_offset":44172,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":45696,"parent_record_offset":45184,"ancestor_record_offset":44172,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":47172,"parent_record_offset":47136,"ancestor_record_offset":44172,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Ok
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":578,"record_offset":47492,"parent_record_offset":46924,"ancestor_record_offset":44172,"inlinee":29637,"source_line":653,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":578,"record_offset":44372,"parent_record_offset":44348,"ancestor_record_offset":44172,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":578,"record_offset":45732,"parent_record_offset":45116,"ancestor_record_offset":44172,"inlinee":10940,"source_line":63,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$46::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":578,"record_offset":45756,"parent_record_offset":45732,"ancestor_record_offset":44172,"inlinee":11836,"source_line":851,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":578,"record_offset":45808,"parent_record_offset":45784,"ancestor_record_offset":44172,"inlinee":10753,"source_line":380,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":578,"record_offset":45784,"parent_record_offset":45756,"ancestor_record_offset":44172,"inlinee":10751,"source_line":49,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":578,"record_offset":44748,"parent_record_offset":44724,"ancestor_record_offset":44172,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":578,"record_offset":44844,"parent_record_offset":44820,"ancestor_record_offset":44172,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":578,"record_offset":44772,"parent_record_offset":44748,"ancestor_record_offset":44172,"inlinee":5043,"source_line":4237,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$4::spec_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\mod.rs
//     evidence={"module":578,"record_offset":44796,"parent_record_offset":44772,"ancestor_record_offset":44172,"inlinee":5045,"source_line":374,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$4::spec_try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":578,"record_offset":44820,"parent_record_offset":44796,"ancestor_record_offset":44172,"inlinee":5046,"source_line":4292,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":578,"record_offset":45832,"parent_record_offset":45808,"ancestor_record_offset":44172,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":578,"record_offset":46968,"parent_record_offset":46924,"ancestor_record_offset":44172,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":578,"record_offset":46892,"parent_record_offset":46868,"ancestor_record_offset":44172,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":578,"record_offset":47416,"parent_record_offset":47136,"ancestor_record_offset":44172,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":578,"record_offset":47440,"parent_record_offset":47416,"ancestor_record_offset":44172,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":578,"record_offset":44700,"parent_record_offset":44648,"ancestor_record_offset":44172,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":578,"record_offset":45544,"parent_record_offset":45520,"ancestor_record_offset":44172,"inlinee":5028,"source_line":1432,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":578,"record_offset":46832,"parent_record_offset":46752,"ancestor_record_offset":44172,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":578,"record_offset":46336,"parent_record_offset":46304,"ancestor_record_offset":44172,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":578,"record_offset":46800,"parent_record_offset":46752,"ancestor_record_offset":44172,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":578,"record_offset":45568,"parent_record_offset":45544,"ancestor_record_offset":44172,"inlinee":5029,"source_line":697,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":578,"record_offset":47668,"parent_record_offset":47600,"ancestor_record_offset":44172,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":578,"record_offset":46364,"parent_record_offset":46304,"ancestor_record_offset":44172,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":578,"record_offset":46396,"parent_record_offset":46264,"ancestor_record_offset":44172,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::audio_trigger_library::AudioTriggerCollection::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/audio_trigger_library.rs
//     evidence={"module":578,"record_offset":44624,"parent_record_offset":44600,"ancestor_record_offset":44172,"inlinee":32563,"source_line":23,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":578,"record_offset":45148,"parent_record_offset":45116,"ancestor_record_offset":44172,"inlinee":30854,"source_line":1093,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::impl$2::fill_tag
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":578,"record_offset":45520,"parent_record_offset":45460,"ancestor_record_offset":44172,"inlinee":5027,"source_line":79,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":578,"record_offset":44452,"parent_record_offset":44428,"ancestor_record_offset":44172,"inlinee":19433,"source_line":1009,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":45272,"parent_record_offset":45244,"ancestor_record_offset":44172,"inlinee":12661,"source_line":872,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":45392,"parent_record_offset":45364,"ancestor_record_offset":44172,"inlinee":12651,"source_line":866,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":45420,"parent_record_offset":45392,"ancestor_record_offset":44172,"inlinee":5023,"source_line":3207,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::ctrl_slice
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":45460,"parent_record_offset":45420,"ancestor_record_offset":44172,"inlinee":5024,"source_line":2669,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":45636,"parent_record_offset":45420,"ancestor_record_offset":44172,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::num_ctrl_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":45492,"parent_record_offset":45460,"ancestor_record_offset":44172,"inlinee":5025,"source_line":2691,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":45608,"parent_record_offset":45420,"ancestor_record_offset":44172,"inlinee":5031,"source_line":184,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::clear::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":45364,"parent_record_offset":45336,"ancestor_record_offset":44172,"inlinee":12650,"source_line":878,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::scopeguard::impl$3::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\scopeguard.rs
//     evidence={"module":578,"record_offset":45336,"parent_record_offset":45308,"ancestor_record_offset":44172,"inlinee":12554,"source_line":69,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::SlotMap::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.1.1/src/basic.rs
//     evidence={"module":578,"record_offset":44648,"parent_record_offset":44624,"ancestor_record_offset":44172,"inlinee":32564,"source_line":615,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::SlotMap::drain
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.1.1/src/basic.rs
//     evidence={"module":578,"record_offset":44672,"parent_record_offset":44648,"ancestor_record_offset":44172,"inlinee":36813,"source_line":640,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::impl$14::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.1.1/src/basic.rs
//     evidence={"module":578,"record_offset":44724,"parent_record_offset":44700,"ancestor_record_offset":44172,"inlinee":30075,"source_line":1134,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":578,"record_offset":44428,"parent_record_offset":44404,"ancestor_record_offset":44172,"inlinee":21905,"source_line":829,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] utils::index_condenser::IndexCondenser::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/index_condenser.rs
//     evidence={"module":578,"record_offset":45184,"parent_record_offset":45148,"ancestor_record_offset":44172,"inlinee":30855,"source_line":172,"archive":"_pdb/C/modules/00578.jsonl.gz"}
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
// 完整结构及原始类型记录: clearing.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::clearing::PlacementConfigEntry
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["country_core::startup::clearing::PlacementConfigEntry", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::startup::clearing::PlacementConfigEntry>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::startup::clearing::PlacementConfigEntry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::startup::clearing::PlacementConfigEntry", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::from_str<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global> >", "constructor": "serde_json::de::from_str", "arguments": ["alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<country_core::startup::clearing::PlacementConfigEntry,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["country_core::startup::clearing::PlacementConfigEntry", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<country_core::startup::clearing::PlacementConfigEntry,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<country_core::startup::clearing::PlacementConfigEntry,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<country_core::startup::clearing::PlacementConfigEntry,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<country_core::startup::clearing::PlacementConfigEntry,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::clearing::PlacementConfigEntry>", "constructor": "serde_core::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["country_core::startup::clearing::PlacementConfigEntry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$7::deserialize_seq<serde_json::read::StrRead,serde_core::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::clearing::PlacementConfigEntry> >", "constructor": "serde_json::de::impl$7::deserialize_seq", "arguments": ["serde_json::read::StrRead", "serde_core::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::clearing::PlacementConfigEntry>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<country_core::startup::clearing::PlacementConfigEntry,serde_json::de::SeqAccess<serde_json::read::StrRead> >", "constructor": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["country_core::startup::clearing::PlacementConfigEntry", "serde_json::de::SeqAccess<serde_json::read::StrRead>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: clearing.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: clearing.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 22; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)
// 候选参数类型: ["ref$<tg_mods::overlay::Mod>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] country_core::systems::wall::structure_outline::WallOutline (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// [文件级接口候选，具体函数归属 unknown] tuple$<bool,tuple$<> > (*)(ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>,enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >,ref$<bevy_ecs::component::info::Components>,ref$<slice2$<bevy_ecs::component::info::ComponentId> >)
// 候选参数类型: ["ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>", "enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >", "ref$<bevy_ecs::component::info::Components>", "ref$<slice2$<bevy_ecs::component::info::ComponentId> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] relative_path::relative_path_buf::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::_::impl$0::deserialize
// 信息 ID: d3b9707f695d011caec86e6d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<country_core::startup::clearing::PlacementConfigEntry,serde_json::de::SeqAccess<serde_json::read::StrRead> >; module=1665, inline_record=72720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: fe400392318de009f89bd254
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::startup::clearing::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::startup::clearing::_::impl$0::deserialize::__Visitor> @ 0x1669eb2 -> 0xc19230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: b7dca194fc85c0b9cfaa48af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::startup::clearing::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=316212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 6935f78d747668c1f23041e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::startup::clearing::_::impl$0::deserialize::__Visitor>; module=592, inline_record=174508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 154b630d710305209e0340e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::startup::clearing::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=316004; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: c9fd8646e560c1d567f7c658
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::startup::clearing::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=321996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::init_clearing
// 信息 ID: 4163a6ff9e3d7e9be08392e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 6169c3dec6c9d99e476b7cb9: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)> @ 0xc16218 -> 0xba7100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)> > @ 0x1924b12 -> 0xba7100 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::init_clearing::closure$0
// 信息 ID: ea574943e76970484c791c98
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::clearing::init_clearing", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::clearing::init_clearing::closure_env$0 <- alloc::vec::spec_extend::impl$0::spec_extend<country_core::resources::tree_storage::ForestTree,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::placement_data::PlacementDataSsbo>,country_core::startup::clearing::init_clearing::closure_env$0>,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_extend::impl$0::spec_extend<country_core::resources::tree_storage::ForestTree,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::placement_data::PlacementDataSsbo>,country_core::startup::clearing::init_clearing::closure_env$0>,alloc::alloc::Global>; module=1643, inline_record=298700; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::init_clearing::closure$1
// 信息 ID: e99a2d0997a20ed2ad07db55
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::clearing::init_clearing", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::clearing::init_clearing::closure_env$1 <- core::ops::function::impls::impl$3::call_mut<tuple$<ref$<glam::f32::vec3::Vec3> >,country_core::startup::clearing::init_clearing::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<glam::f32::vec3::Vec3> >,country_core::startup::clearing::init_clearing::closure_env$1>; module=592, inline_record=174764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::uninit_clearing
// 信息 ID: 5fd11ed84e0b402f8bb364b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f1571263866702a2d9c5663c: void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)> @ 0xc17396 -> 0xbab6b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)> > @ 0x1912a41 -> 0xbab6b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
