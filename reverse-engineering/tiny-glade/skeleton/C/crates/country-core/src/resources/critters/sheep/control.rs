// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/critters/sheep/control.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: d3fb9b99883122116286ad9e1f610a189c1b704c3d0af821e9a45301a218891e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: control.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 25, 43, 210, 545, 552, 562, 564, 590, 1636, 1643, 1645, 1665, 1700, 1803, 1804
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"},{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"},{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::critters::sheep::control::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 11, 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8A34
// 首条位置: {"module":564,"inlinee":35380}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::critters::sheep::control::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 11, 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8A3B
// 首条位置: {"module":1645,"inlinee":35387}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1645,"record_offset":425096,"parent_record_offset":425072,"ancestor_record_offset":425072,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1645,"record_offset":425120,"parent_record_offset":425096,"ancestor_record_offset":425072,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1645,"record_offset":425144,"parent_record_offset":425120,"ancestor_record_offset":425072,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1645,"record_offset":425172,"parent_record_offset":425144,"ancestor_record_offset":425072,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::critters::sheep::control::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 11, 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13E87
// 首条位置: {"module":1645,"inlinee":81543}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":424892,"parent_record_offset":424868,"ancestor_record_offset":424868,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::de::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":275744,"parent_record_offset":275720,"ancestor_record_offset":275720,"inlinee":37547,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":276164,"parent_record_offset":276140,"ancestor_record_offset":275720,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":276188,"parent_record_offset":276164,"ancestor_record_offset":275720,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":276212,"parent_record_offset":276188,"ancestor_record_offset":275720,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":276140,"parent_record_offset":276084,"ancestor_record_offset":275720,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":276112,"parent_record_offset":276084,"ancestor_record_offset":275720,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":276084,"parent_record_offset":276060,"ancestor_record_offset":275720,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":276060,"parent_record_offset":276036,"ancestor_record_offset":275720,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1645,"record_offset":425044,"parent_record_offset":424916,"ancestor_record_offset":424868,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1645,"record_offset":425120,"parent_record_offset":425096,"ancestor_record_offset":424868,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":275988,"parent_record_offset":275816,"ancestor_record_offset":275720,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1645,"record_offset":425144,"parent_record_offset":425120,"ancestor_record_offset":424868,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1645,"record_offset":425172,"parent_record_offset":425144,"ancestor_record_offset":424868,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1645,"record_offset":425096,"parent_record_offset":425072,"ancestor_record_offset":424868,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::critters::sheep::control::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters/sheep/control.rs
//     evidence={"module":1645,"record_offset":425072,"parent_record_offset":424916,"ancestor_record_offset":424868,"inlinee":35387,"source_line":11,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":275816,"parent_record_offset":275792,"ancestor_record_offset":275720,"inlinee":10022,"source_line":1568,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":424960,"parent_record_offset":424916,"ancestor_record_offset":424868,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":424916,"parent_record_offset":424892,"ancestor_record_offset":424868,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":424984,"parent_record_offset":424960,"ancestor_record_offset":424868,"inlinee":7606,"source_line":693,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":425008,"parent_record_offset":424984,"ancestor_record_offset":424868,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$21::deserialize_any
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":275768,"parent_record_offset":275744,"ancestor_record_offset":275720,"inlinee":21295,"source_line":1248,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$26::deserialize_any
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":275792,"parent_record_offset":275768,"ancestor_record_offset":275720,"inlinee":21297,"source_line":1420,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::critters::sheep::control::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 11, 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x847E
// 首条位置: {"module":562,"inlinee":33918}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::critters::sheep::control::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 11, 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13E85
// 首条位置: {"module":1645,"inlinee":81541}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1645,"record_offset":425772,"parent_record_offset":424716,"ancestor_record_offset":424716,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1645,"record_offset":426336,"parent_record_offset":424716,"ancestor_record_offset":424716,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1645,"record_offset":426304,"parent_record_offset":424716,"ancestor_record_offset":424716,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1645,"record_offset":426624,"parent_record_offset":424716,"ancestor_record_offset":424716,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":426764,"parent_record_offset":424716,"ancestor_record_offset":424716,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":424768,"parent_record_offset":424716,"ancestor_record_offset":424716,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":425240,"parent_record_offset":424716,"ancestor_record_offset":424716,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":228804,"parent_record_offset":228744,"ancestor_record_offset":228744,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":228896,"parent_record_offset":228744,"ancestor_record_offset":228744,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":426520,"parent_record_offset":426496,"ancestor_record_offset":424716,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":426544,"parent_record_offset":426520,"ancestor_record_offset":424716,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":426568,"parent_record_offset":426544,"ancestor_record_offset":424716,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":426496,"parent_record_offset":426408,"ancestor_record_offset":424716,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":426436,"parent_record_offset":426408,"ancestor_record_offset":424716,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":426408,"parent_record_offset":426384,"ancestor_record_offset":424716,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":426384,"parent_record_offset":426360,"ancestor_record_offset":424716,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1645,"record_offset":425044,"parent_record_offset":424916,"ancestor_record_offset":424716,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1645,"record_offset":425120,"parent_record_offset":425096,"ancestor_record_offset":424716,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":229012,"parent_record_offset":228988,"ancestor_record_offset":228744,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1645,"record_offset":426464,"parent_record_offset":426436,"ancestor_record_offset":424716,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":228988,"parent_record_offset":228952,"ancestor_record_offset":228744,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1645,"record_offset":426360,"parent_record_offset":426336,"ancestor_record_offset":424716,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":229956,"parent_record_offset":229904,"ancestor_record_offset":228744,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1645,"record_offset":425144,"parent_record_offset":425120,"ancestor_record_offset":424716,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1645,"record_offset":425172,"parent_record_offset":425144,"ancestor_record_offset":424716,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1645,"record_offset":425096,"parent_record_offset":425072,"ancestor_record_offset":424716,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::critters::sheep::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters/sheep.rs
//     evidence={"module":1804,"record_offset":230068,"parent_record_offset":230044,"ancestor_record_offset":228744,"inlinee":85749,"source_line":25,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::critters::sheep::control::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters/sheep/control.rs
//     evidence={"module":1645,"record_offset":425072,"parent_record_offset":424916,"ancestor_record_offset":424716,"inlinee":35387,"source_line":11,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::critters::sheep::control::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters/sheep/control.rs
//     evidence={"module":1645,"record_offset":424868,"parent_record_offset":424844,"ancestor_record_offset":424716,"inlinee":81543,"source_line":11,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$15::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/features/impl_serde.rs
//     evidence={"module":1645,"record_offset":430792,"parent_record_offset":430768,"ancestor_record_offset":429484,"inlinee":7648,"source_line":90,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1645,"record_offset":426696,"parent_record_offset":426672,"ancestor_record_offset":424716,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":230092,"parent_record_offset":230068,"ancestor_record_offset":228744,"inlinee":7799,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":426672,"parent_record_offset":426648,"ancestor_record_offset":424716,"inlinee":7805,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_tuple_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":230232,"parent_record_offset":230208,"ancestor_record_offset":228744,"inlinee":34837,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":426720,"parent_record_offset":426696,"ancestor_record_offset":424716,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":228828,"parent_record_offset":228804,"ancestor_record_offset":228744,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":228924,"parent_record_offset":228896,"ancestor_record_offset":228744,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1645,"record_offset":425704,"parent_record_offset":425680,"ancestor_record_offset":424716,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":424844,"parent_record_offset":424796,"ancestor_record_offset":424716,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":229904,"parent_record_offset":229880,"ancestor_record_offset":228744,"inlinee":10077,"source_line":901,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":229856,"parent_record_offset":229832,"ancestor_record_offset":228744,"inlinee":9918,"source_line":920,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1645,"record_offset":426264,"parent_record_offset":426240,"ancestor_record_offset":424716,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=14
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":424960,"parent_record_offset":424916,"ancestor_record_offset":424716,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":425316,"parent_record_offset":425276,"ancestor_record_offset":424716,"inlinee":7597,"source_line":1061,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":425360,"parent_record_offset":425316,"ancestor_record_offset":424716,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":425400,"parent_record_offset":425360,"ancestor_record_offset":424716,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":424796,"parent_record_offset":424768,"ancestor_record_offset":424716,"inlinee":7724,"source_line":1986,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":425276,"parent_record_offset":425240,"ancestor_record_offset":424716,"inlinee":7596,"source_line":2027,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":424916,"parent_record_offset":424892,"ancestor_record_offset":424716,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":424892,"parent_record_offset":424868,"ancestor_record_offset":424716,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$7::deserialize_ignored_any
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":425728,"parent_record_offset":425704,"ancestor_record_offset":424716,"inlinee":7602,"source_line":1910,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=git_dependency_definition; count=14
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":424984,"parent_record_offset":424960,"ancestor_record_offset":424716,"inlinee":7606,"source_line":693,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":425428,"parent_record_offset":425400,"ancestor_record_offset":424716,"inlinee":7605,"source_line":688,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=14
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":425008,"parent_record_offset":424984,"ancestor_record_offset":424716,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":425456,"parent_record_offset":425428,"ancestor_record_offset":424716,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":228952,"parent_record_offset":228924,"ancestor_record_offset":228744,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_option
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":229880,"parent_record_offset":229856,"ancestor_record_offset":228744,"inlinee":85976,"source_line":317,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::critters::sheep::control::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 11, 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13E88
// 首条位置: {"module":1645,"inlinee":81544}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":427196,"parent_record_offset":427164,"ancestor_record_offset":427164,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$6::next_element
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":54832,"parent_record_offset":54808,"ancestor_record_offset":54808,"inlinee":15022,"source_line":1798,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1803,"record_offset":54916,"parent_record_offset":54888,"ancestor_record_offset":54808,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":54972,"parent_record_offset":54916,"ancestor_record_offset":54808,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":55000,"parent_record_offset":54916,"ancestor_record_offset":54808,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":54944,"parent_record_offset":54916,"ancestor_record_offset":54808,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":55024,"parent_record_offset":55000,"ancestor_record_offset":54808,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":55116,"parent_record_offset":54888,"ancestor_record_offset":54808,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$15::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/features/impl_serde.rs
//     evidence={"module":1645,"record_offset":432944,"parent_record_offset":432920,"ancestor_record_offset":432816,"inlinee":7648,"source_line":90,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":54860,"parent_record_offset":54832,"ancestor_record_offset":54808,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":427280,"parent_record_offset":427224,"ancestor_record_offset":427164,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":162676,"parent_record_offset":162652,"ancestor_record_offset":162004,"inlinee":10077,"source_line":901,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":162628,"parent_record_offset":162604,"ancestor_record_offset":162004,"inlinee":9918,"source_line":920,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1645,"record_offset":427304,"parent_record_offset":427280,"ancestor_record_offset":427164,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":427224,"parent_record_offset":427196,"ancestor_record_offset":427164,"inlinee":7609,"source_line":1933,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$10::next_element_seed
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":54888,"parent_record_offset":54860,"ancestor_record_offset":54808,"inlinee":21316,"source_line":636,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_option
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":162652,"parent_record_offset":162628,"ancestor_record_offset":162004,"inlinee":85976,"source_line":317,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::critters::sheep::control::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 11, 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":562,"record_offset":41668,"rva":11650144,"va":5380359264}
// 编译实例 RVAs: 0xb1c460, 0xb1c760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f9710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8531507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380359264,"raw_hex":"e8cdd17dff","size":5,"soft_fail":false,"target_va":5371827984,"va":5380359486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b1440
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14240917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380360032,"raw_hex":"e8954cd900","size":5,"soft_fail":false,"target_va":5394601024,"va":5380360102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b4790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14254816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380359264,"raw_hex":"e8e082d900","size":5,"soft_fail":false,"target_va":5394614160,"va":5380359339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418bc550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14286950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380359264,"raw_hex":"e86600da00","size":5,"soft_fail":false,"target_va":5394646352,"va":5380359397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418bcfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14289580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380359264,"raw_hex":"e8ac0ada00","size":5,"soft_fail":false,"target_va":5394649024,"va":5380359439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35993507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380359264,"raw_hex":"e8a3372502","size":5,"soft_fail":false,"target_va":5416353040,"va":5380359528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142da1cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t36198368","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5380359264,"raw_hex":"e9e0572802","size":5,"soft_fail":false,"target_va":5416557744,"va":5380359371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":562,"record_offset":41960,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":562,"record_offset":42596,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":4577,"source_line":718,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":562,"record_offset":42504,"parent_record_offset":41668,"ancestor_record_offset":41668,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":562,"record_offset":42356,"parent_record_offset":42324,"ancestor_record_offset":41668,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":562,"record_offset":42224,"parent_record_offset":42196,"ancestor_record_offset":41668,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":562,"record_offset":42284,"parent_record_offset":42252,"ancestor_record_offset":41668,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":562,"record_offset":42404,"parent_record_offset":42380,"ancestor_record_offset":41668,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":562,"record_offset":42380,"parent_record_offset":42356,"ancestor_record_offset":41668,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":562,"record_offset":42252,"parent_record_offset":42224,"ancestor_record_offset":41668,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":562,"record_offset":42156,"parent_record_offset":42128,"ancestor_record_offset":41668,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":562,"record_offset":42324,"parent_record_offset":42156,"ancestor_record_offset":41668,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":562,"record_offset":42100,"parent_record_offset":42072,"ancestor_record_offset":41668,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":562,"record_offset":42444,"parent_record_offset":42156,"ancestor_record_offset":41668,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":562,"record_offset":42196,"parent_record_offset":42156,"ancestor_record_offset":41668,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":562,"record_offset":42128,"parent_record_offset":42100,"ancestor_record_offset":41668,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":562,"record_offset":42016,"parent_record_offset":41988,"ancestor_record_offset":41668,"inlinee":4570,"source_line":1865,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":562,"record_offset":42648,"parent_record_offset":42624,"ancestor_record_offset":41668,"inlinee":4580,"source_line":1875,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":562,"record_offset":41988,"parent_record_offset":41960,"ancestor_record_offset":41668,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":562,"record_offset":42624,"parent_record_offset":42596,"ancestor_record_offset":41668,"inlinee":4579,"source_line":669,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":562,"record_offset":42072,"parent_record_offset":42044,"ancestor_record_offset":41668,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":562,"record_offset":42044,"parent_record_offset":42016,"ancestor_record_offset":41668,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::critters::sheep::control::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 11, 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":562,"record_offset":39756,"rva":11649392,"va":5380358512}
// 编译实例 RVAs: 0xb1c170, 0xb1c580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f9710
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8530795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380358512,"raw_hex":"e895d47dff","size":5,"soft_fail":false,"target_va":5371827984,"va":5380358774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b0f10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14241092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380358512,"raw_hex":"e8444dd900","size":5,"soft_fail":false,"target_va":5394599696,"va":5380358599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418bbe80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14284963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380359552,"raw_hex":"e8a3f8d900","size":5,"soft_fail":false,"target_va":5394644608,"va":5380359640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418bc6c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14288050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380358512,"raw_hex":"e8b204da00","size":5,"soft_fail":false,"target_va":5394646720,"va":5380358665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418be700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14296264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380358512,"raw_hex":"e8c824da00","size":5,"soft_fail":false,"target_va":5394654976,"va":5380358707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35994167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380358512,"raw_hex":"e8373a2502","size":5,"soft_fail":false,"target_va":5416353040,"va":5380358868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36698226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380358512,"raw_hex":"e872f82f02","size":5,"soft_fail":false,"target_va":5417057056,"va":5380358825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142da1cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t36199100","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5380358512,"raw_hex":"e9bc5a2802","size":5,"soft_fail":false,"target_va":5416557744,"va":5380358639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":562,"record_offset":40048,"parent_record_offset":39756,"ancestor_record_offset":39756,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":562,"record_offset":40688,"parent_record_offset":39756,"ancestor_record_offset":39756,"inlinee":4577,"source_line":718,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":562,"record_offset":40596,"parent_record_offset":39756,"ancestor_record_offset":39756,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":562,"record_offset":40448,"parent_record_offset":40416,"ancestor_record_offset":39756,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":562,"record_offset":40316,"parent_record_offset":40288,"ancestor_record_offset":39756,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":562,"record_offset":41232,"parent_record_offset":41208,"ancestor_record_offset":39756,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":562,"record_offset":40376,"parent_record_offset":40344,"ancestor_record_offset":39756,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":562,"record_offset":40496,"parent_record_offset":40472,"ancestor_record_offset":39756,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":562,"record_offset":40472,"parent_record_offset":40448,"ancestor_record_offset":39756,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":562,"record_offset":40344,"parent_record_offset":40316,"ancestor_record_offset":39756,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":562,"record_offset":40248,"parent_record_offset":40220,"ancestor_record_offset":39756,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":562,"record_offset":40416,"parent_record_offset":40248,"ancestor_record_offset":39756,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":562,"record_offset":40192,"parent_record_offset":40164,"ancestor_record_offset":39756,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":562,"record_offset":40536,"parent_record_offset":40248,"ancestor_record_offset":39756,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":562,"record_offset":40288,"parent_record_offset":40248,"ancestor_record_offset":39756,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":562,"record_offset":40220,"parent_record_offset":40192,"ancestor_record_offset":39756,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":562,"record_offset":40924,"parent_record_offset":40896,"ancestor_record_offset":39756,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":562,"record_offset":40896,"parent_record_offset":40868,"ancestor_record_offset":39756,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":562,"record_offset":40868,"parent_record_offset":40832,"ancestor_record_offset":39756,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":562,"record_offset":41284,"parent_record_offset":41076,"ancestor_record_offset":39756,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$16::begin_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":562,"record_offset":40104,"parent_record_offset":40076,"ancestor_record_offset":39756,"inlinee":12492,"source_line":2018,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$16::end_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":562,"record_offset":40740,"parent_record_offset":40716,"ancestor_record_offset":39756,"inlinee":12493,"source_line":2028,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":562,"record_offset":40076,"parent_record_offset":40048,"ancestor_record_offset":39756,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":562,"record_offset":40716,"parent_record_offset":40688,"ancestor_record_offset":39756,"inlinee":4579,"source_line":669,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::indent
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":562,"record_offset":40832,"parent_record_offset":40740,"ancestor_record_offset":39756,"inlinee":8104,"source_line":2276,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":562,"record_offset":40164,"parent_record_offset":40136,"ancestor_record_offset":39756,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":562,"record_offset":40136,"parent_record_offset":40104,"ancestor_record_offset":39756,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00562.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::critters::sheep::control::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=2
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":25,"record_offset":49648,"rva":1352144,"va":5370061264}
// 编译实例 RVAs: 0x14a1d0, 0xb1c330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$::end
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t13166286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370061264,"raw_hex":"e8cee6c800","size":5,"soft_fail":false,"target_va":5383227792,"va":5370061501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014a380
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370061264,"raw_hex":"e8f4000000","size":5,"soft_fail":false,"target_va":5370061696,"va":5370061447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d6f50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4771108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370061264,"raw_hex":"e824cd4800","size":5,"soft_fail":false,"target_va":5374832464,"va":5370061351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d7830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4773345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370061264,"raw_hex":"e8e1d54800","size":5,"soft_fail":false,"target_va":5374834736,"va":5370061386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d90b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4779578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370061264,"raw_hex":"e83aee4800","size":5,"soft_fail":false,"target_va":5374841008,"va":5370061425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b1d7f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380358960,"raw_hex":"e804140000","size":5,"soft_fail":false,"target_va":5380364272,"va":5380359143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b80ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17188810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380358960,"raw_hex":"e8ca470601","size":5,"soft_fail":false,"target_va":5397547936,"va":5380359121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b82700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17195857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380358960,"raw_hex":"e851630601","size":5,"soft_fail":false,"target_va":5397554944,"va":5380359082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b8b0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: control.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17231124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380358960,"raw_hex":"e814ed0601","size":5,"soft_fail":false,"target_va":5397590176,"va":5380359047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":25,"record_offset":49840,"parent_record_offset":49648,"ancestor_record_offset":49648,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00025.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::critters::sheep::control::impl$0::clone
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x86AE
// 首条位置: {"module":552,"inlinee":34478}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$6::clone
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":552,"record_offset":22412,"parent_record_offset":22384,"ancestor_record_offset":22384,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$102::clone
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":552,"record_offset":22440,"parent_record_offset":22412,"ancestor_record_offset":22384,"inlinee":9872,"source_line":20,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::critters::sheep::control::impl$2::eq
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x86AC
// 首条位置: {"module":552,"inlinee":34476}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":552,"record_offset":26528,"parent_record_offset":26168,"ancestor_record_offset":26168,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::critters::sheep::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters/sheep.rs
//     evidence={"module":552,"record_offset":26256,"parent_record_offset":26168,"ancestor_record_offset":26168,"inlinee":34480,"source_line":25,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$105::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":552,"record_offset":26216,"parent_record_offset":26168,"ancestor_record_offset":26168,"inlinee":11117,"source_line":20,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$19::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":552,"record_offset":26492,"parent_record_offset":26468,"ancestor_record_offset":26168,"inlinee":13199,"source_line":1878,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":552,"record_offset":26324,"parent_record_offset":26300,"ancestor_record_offset":26168,"inlinee":7407,"source_line":1878,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":552,"record_offset":26300,"parent_record_offset":26256,"ancestor_record_offset":26168,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$105::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":552,"record_offset":26556,"parent_record_offset":26528,"ancestor_record_offset":26168,"inlinee":11117,"source_line":20,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::critters::sheep::control::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1BB7
// 首条位置: {"module":43,"inlinee":7095}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::critters::sheep::control::impl$3::clone
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x86A2
// 首条位置: {"module":552,"inlinee":34466}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":552,"record_offset":21700,"parent_record_offset":21660,"ancestor_record_offset":21660,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":552,"record_offset":22136,"parent_record_offset":22112,"ancestor_record_offset":21660,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":552,"record_offset":22160,"parent_record_offset":22136,"ancestor_record_offset":21660,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":552,"record_offset":22184,"parent_record_offset":22160,"ancestor_record_offset":21660,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":552,"record_offset":22112,"parent_record_offset":21928,"ancestor_record_offset":21660,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":552,"record_offset":21864,"parent_record_offset":21836,"ancestor_record_offset":21660,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":552,"record_offset":21928,"parent_record_offset":21892,"ancestor_record_offset":21660,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":552,"record_offset":21892,"parent_record_offset":21864,"ancestor_record_offset":21660,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":552,"record_offset":21968,"parent_record_offset":21928,"ancestor_record_offset":21660,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":552,"record_offset":21740,"parent_record_offset":21700,"ancestor_record_offset":21660,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":552,"record_offset":21780,"parent_record_offset":21740,"ancestor_record_offset":21660,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":590,"record_offset":74804,"parent_record_offset":74768,"ancestor_record_offset":74712,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":552,"record_offset":21836,"parent_record_offset":21780,"ancestor_record_offset":21660,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":590,"record_offset":74768,"parent_record_offset":74740,"ancestor_record_offset":74712,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":552,"record_offset":21992,"parent_record_offset":21968,"ancestor_record_offset":21660,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":552,"record_offset":22072,"parent_record_offset":22016,"ancestor_record_offset":21660,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":552,"record_offset":22288,"parent_record_offset":22240,"ancestor_record_offset":21660,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":552,"record_offset":22240,"parent_record_offset":21780,"ancestor_record_offset":21660,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":552,"record_offset":22476,"parent_record_offset":21780,"ancestor_record_offset":21660,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":552,"record_offset":22016,"parent_record_offset":21992,"ancestor_record_offset":21660,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":552,"record_offset":22044,"parent_record_offset":22016,"ancestor_record_offset":21660,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":552,"record_offset":22412,"parent_record_offset":22384,"ancestor_record_offset":21660,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":552,"record_offset":22344,"parent_record_offset":22316,"ancestor_record_offset":21660,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":552,"record_offset":22316,"parent_record_offset":22288,"ancestor_record_offset":21660,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::critters::sheep::control::impl$0::clone
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters/sheep/control.rs
//     evidence={"module":552,"record_offset":22384,"parent_record_offset":21780,"ancestor_record_offset":21660,"inlinee":34478,"source_line":11,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$102::clone
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":552,"record_offset":22440,"parent_record_offset":22412,"ancestor_record_offset":21660,"inlinee":9872,"source_line":20,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::critters::sheep::control::impl$5::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x86AA
// 首条位置: {"module":552,"inlinee":34474}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":552,"record_offset":26028,"parent_record_offset":25996,"ancestor_record_offset":25996,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":552,"record_offset":26132,"parent_record_offset":26092,"ancestor_record_offset":25996,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$19::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":552,"record_offset":26492,"parent_record_offset":26468,"ancestor_record_offset":25996,"inlinee":13199,"source_line":1878,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":552,"record_offset":26324,"parent_record_offset":26300,"ancestor_record_offset":25996,"inlinee":7407,"source_line":1878,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":552,"record_offset":26300,"parent_record_offset":26256,"ancestor_record_offset":25996,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":552,"record_offset":26528,"parent_record_offset":26168,"ancestor_record_offset":25996,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":552,"record_offset":26056,"parent_record_offset":26028,"ancestor_record_offset":25996,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":552,"record_offset":26092,"parent_record_offset":26056,"ancestor_record_offset":25996,"inlinee":6223,"source_line":122,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::critters::sheep::control::impl$2::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters/sheep/control.rs
//     evidence={"module":552,"record_offset":26168,"parent_record_offset":26132,"ancestor_record_offset":25996,"inlinee":34476,"source_line":11,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::critters::sheep::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters/sheep.rs
//     evidence={"module":552,"record_offset":26256,"parent_record_offset":26168,"ancestor_record_offset":25996,"inlinee":34480,"source_line":25,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$105::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":552,"record_offset":26216,"parent_record_offset":26168,"ancestor_record_offset":25996,"inlinee":11117,"source_line":20,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::critters::sheep::control::capture_sheep_snapshot
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8472
// 首条位置: {"module":552,"inlinee":33906}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":552,"record_offset":16660,"parent_record_offset":16632,"ancestor_record_offset":16632,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":552,"record_offset":16880,"parent_record_offset":16632,"ancestor_record_offset":16632,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":590,"record_offset":43192,"parent_record_offset":43168,"ancestor_record_offset":42868,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":590,"record_offset":43240,"parent_record_offset":43216,"ancestor_record_offset":42868,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":590,"record_offset":43216,"parent_record_offset":43192,"ancestor_record_offset":42868,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":590,"record_offset":43168,"parent_record_offset":43144,"ancestor_record_offset":42868,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":590,"record_offset":43144,"parent_record_offset":43112,"ancestor_record_offset":42868,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":552,"record_offset":16904,"parent_record_offset":16880,"ancestor_record_offset":16632,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":590,"record_offset":43112,"parent_record_offset":43080,"ancestor_record_offset":42868,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":552,"record_offset":16796,"parent_record_offset":16684,"ancestor_record_offset":16632,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":590,"record_offset":43080,"parent_record_offset":43048,"ancestor_record_offset":42868,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":552,"record_offset":16736,"parent_record_offset":16712,"ancestor_record_offset":16632,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":552,"record_offset":16760,"parent_record_offset":16736,"ancestor_record_offset":16632,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":552,"record_offset":16684,"parent_record_offset":16660,"ancestor_record_offset":16632,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":552,"record_offset":16712,"parent_record_offset":16684,"ancestor_record_offset":16632,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":552,"record_offset":16840,"parent_record_offset":16796,"ancestor_record_offset":16632,"inlinee":11874,"source_line":264,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":552,"record_offset":16928,"parent_record_offset":16904,"ancestor_record_offset":16632,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":590,"record_offset":43308,"parent_record_offset":43284,"ancestor_record_offset":42868,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":590,"record_offset":43284,"parent_record_offset":43112,"ancestor_record_offset":42868,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::critters::sheep::control::capture_sheep_snapshot::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D0E
// 首条位置: {"module":1643,"inlinee":81166}
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
// 完整结构及原始类型记录: control.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::critters::sheep::control::SheepControlSnapshot
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::resources::critters::sheep::control::SheepControlSnapshot>", "constructor": "core::option::Option", "arguments": ["country_core::resources::critters::sheep::control::SheepControlSnapshot"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::resources::critters::sheep::control::SheepControlSnapshot> >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::resources::critters::sheep::control::SheepControlSnapshot>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<country_core::resources::critters::sheep::control::SheepControlSnapshot> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<country_core::resources::critters::sheep::control::SheepControlSnapshot> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<country_core::resources::critters::sheep::control::SheepControlSnapshot,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["country_core::resources::critters::sheep::control::SheepControlSnapshot", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<country_core::resources::critters::sheep::control::SheepControlSnapshot,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<country_core::resources::critters::sheep::control::SheepControlSnapshot,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<country_core::resources::critters::sheep::control::SheepControlSnapshot,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<country_core::resources::critters::sheep::control::SheepControlSnapshot,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result::inspect_err<country_core::resources::critters::sheep::control::SheepControlSnapshot,serde_json::error::Error,country_core::systems::save::load_game::parse_saved_game_data::closure_env$2>", "constructor": "core::result::Result::inspect_err", "arguments": ["country_core::resources::critters::sheep::control::SheepControlSnapshot", "serde_json::error::Error", "country_core::systems::save::load_game::parse_saved_game_data::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::from_str<country_core::resources::critters::sheep::control::SheepControlSnapshot>", "constructor": "serde_json::de::from_str", "arguments": ["country_core::resources::critters::sheep::control::SheepControlSnapshot"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::save::save_game::impl$2::to_json_string<enum2$<core::option::Option<country_core::resources::critters::sheep::control::SheepControlSnapshot> > >", "constructor": "country_core::systems::save::save_game::impl$2::to_json_string", "arguments": ["enum2$<core::option::Option<country_core::resources::critters::sheep::control::SheepControlSnapshot> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::critters::sheep::control::SheepSnapshot
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 14
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::resources::critters::sheep::control::SheepSnapshot,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::resources::critters::sheep::control::SheepSnapshot", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::vec::Vec<country_core::resources::critters::sheep::control::SheepSnapshot,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::vec::Vec<country_core::resources::critters::sheep::control::SheepSnapshot,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<country_core::resources::critters::sheep::control::SheepSnapshot,alloc::alloc::Global> > >", "constructor": "serde_core::ser::Serializer::collect_seq", "arguments": ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<country_core::resources::critters::sheep::control::SheepSnapshot,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<alloc::vec::Vec<country_core::resources::critters::sheep::control::SheepSnapshot,alloc::alloc::Global> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["alloc::vec::Vec<country_core::resources::critters::sheep::control::SheepSnapshot,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::critters::sheep::control::SheepSnapshot>", "constructor": "serde_core::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["country_core::resources::critters::sheep::control::SheepSnapshot"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$4::deserialize_seq<serde_core::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::critters::sheep::control::SheepSnapshot> >", "constructor": "serde_json::value::de::impl$4::deserialize_seq", "arguments": ["serde_core::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::critters::sheep::control::SheepSnapshot>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::next<country_core::resources::critters::sheep::control::SheepSnapshot,bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,country_core::resources::critters::sheep::control::capture_sheep_snapshot::closure_env$0>", "constructor": "core::iter::adapters::map::impl$2::next", "arguments": ["country_core::resources::critters::sheep::control::SheepSnapshot", "bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >", "country_core::resources::critters::sheep::control::capture_sheep_snapshot::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$12::clone<country_core::resources::critters::sheep::control::SheepSnapshot,alloc::alloc::Global>", "constructor": "alloc::vec::impl$12::clone", "arguments": ["country_core::resources::critters::sheep::control::SheepSnapshot", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::critters::sheep::control::SheepSnapshot,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,country_core::resources::critters::sheep::control::capture_sheep_snapshot::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::resources::critters::sheep::control::SheepSnapshot", "core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,country_core::resources::critters::sheep::control::capture_sheep_snapshot::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$7::deserialize_seq<serde_json::read::StrRead,serde_core::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::critters::sheep::control::SheepSnapshot> >", "constructor": "serde_json::de::impl$7::deserialize_seq", "arguments": ["serde_json::read::StrRead", "serde_core::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::critters::sheep::control::SheepSnapshot>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<country_core::resources::critters::sheep::control::SheepSnapshot,serde_json::de::SeqAccess<serde_json::read::StrRead> >", "constructor": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["country_core::resources::critters::sheep::control::SheepSnapshot", "serde_json::de::SeqAccess<serde_json::read::StrRead>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<country_core::resources::critters::sheep::control::SheepSnapshot,ref_mut$<serde_json::value::de::SeqDeserializer> >", "constructor": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["country_core::resources::critters::sheep::control::SheepSnapshot", "ref_mut$<serde_json::value::de::SeqDeserializer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,alloc::vec::Vec<country_core::resources::critters::sheep::control::SheepSnapshot,alloc::alloc::Global> >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "alloc::vec::Vec<country_core::resources::critters::sheep::control::SheepSnapshot,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,alloc::vec::Vec<country_core::resources::critters::sheep::control::SheepSnapshot,alloc::alloc::Global> >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >", "str$", "alloc::vec::Vec<country_core::resources::critters::sheep::control::SheepSnapshot,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde_core::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::critters::sheep::control::SheepSnapshot> >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde_core::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::critters::sheep::control::SheepSnapshot>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: control.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: control.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 35; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)
// 候选参数类型: ["tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >", "tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState> >", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SaveGameCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SavingGameComplete>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SaveGameCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SavingGameComplete>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >
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
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::critters::sheep::control::_::impl$0::deserialize
// 信息 ID: 4b37bf75c88e15edcf78581d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::load_game::parse_saved_game_data; module=564, inline_record=11832; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::from_str<country_core::resources::critters::sheep::control::SheepControlSnapshot>; module=1645, inline_record=60824; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<country_core::resources::critters::sheep::control::SheepSnapshot,serde_json::de::SeqAccess<serde_json::read::StrRead> >; module=1665, inline_record=82516; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<country_core::resources::critters::sheep::control::SheepSnapshot,ref_mut$<serde_json::value::de::SeqDeserializer> >; module=1665, inline_record=99024; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::critters::sheep::control::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 04ad8dd2dbc6ef7bb0c7b559
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::critters::sheep::control::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=425072; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::critters::sheep::control::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=429884; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::critters::sheep::control::_::impl$0::deserialize::__Field> >; module=1803, inline_record=275844; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::critters::sheep::control::_::impl$0::deserialize::__Field> >; module=1803, inline_record=406852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::critters::sheep::control::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: a1d487a097f8535927d2ddab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::critters::sheep::control::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=424868; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::critters::sheep::control::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=429656; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::critters::sheep::control::_::impl$0::deserialize::__Field> >; module=1803, inline_record=275720; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::critters::sheep::control::_::impl$0::deserialize::__Field> >; module=1803, inline_record=406676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::critters::sheep::control::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: c549d7e287a126ac8d6a27c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::critters::sheep::control::_::impl$0::deserialize::__Visitor>; module=562, inline_record=55096; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::critters::sheep::control::_::impl$0::deserialize::__Visitor>; module=562, inline_record=55544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::critters::sheep::control::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 22f5c484f631ece2bc399bf7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::critters::sheep::control::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=424716; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::critters::sheep::control::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=429484; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::critters::sheep::control::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=228744; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::critters::sheep::control::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=288672; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::critters::sheep::control::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: ebf0fd321a73ead28def19b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::critters::sheep::control::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=427164; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::critters::sheep::control::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=432816; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::critters::sheep::control::_::impl$0::deserialize::__Visitor>; module=1803, inline_record=54808; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::critters::sheep::control::_::impl$0::deserialize::__Visitor>; module=1803, inline_record=162004; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::critters::sheep::control::_::impl$0::serialize
// 信息 ID: 05318d6b2dd50d9fb1af65dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_game::save_game_to_file<std::fs::File,core::slice::iter::Iter<country_core::resources::history::MultiEdit> >; module=43, inline_record=47408; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_game::save_game_to_file<std::io::cursor::Cursor<alloc::vec::Vec<u8,alloc::alloc::Global> >,core::slice::iter::Iter<country_core::resources::history::MultiEdit> >; module=1700, inline_record=141480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::critters::sheep::control::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: d507278e1e5e3410c6f5d853
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::critters::sheep::control::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,alloc::vec::Vec<country_core::resources::critters::sheep::control::SheepSnapshot,alloc::alloc::Global> > @ 0x18b14ed -> 0xb1c460 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,alloc::vec::Vec<country_core::resources::critters::sheep::control::SheepSnapshot,alloc::alloc::Global> > @ 0x18b1536 -> 0xb1c460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_game::impl$2::to_json_string<enum2$<core::option::Option<country_core::resources::critters::sheep::control::SheepControlSnapshot> > > @ 0x1ae41ae -> 0xb1c760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::critters::sheep::control::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >
// 信息 ID: 11d61e4a46380e5a554afebd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::PrettyFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::critters::sheep::control::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,alloc::vec::Vec<country_core::resources::critters::sheep::control::SheepSnapshot,alloc::alloc::Global> > @ 0x18bc0fa -> 0xb1c170 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::debug::sheep_controls::sheep_controls::closure$0::closure$0::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0xc12459 -> 0xb1c580 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::critters::sheep::control::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: f67d180e6380ba0283ca2592
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::critters::sheep::control::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<country_core::resources::critters::sheep::control::SheepSnapshot,alloc::alloc::Global> > > @ 0x5d6002 -> 0x14a1d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<country_core::resources::critters::sheep::control::SheepSnapshot,alloc::alloc::Global> > > @ 0x1b7f1f2 -> 0xb1c330 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::critters::sheep::control::capture_sheep_snapshot
// 信息 ID: 3f4d3329e6020cd7ad2a7c77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_game::save_game; module=552, inline_record=16632; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::debug::sheep_controls::sheep_controls; module=590, inline_record=42868; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::debug::sheep_controls::sheep_controls; module=590, inline_record=44692; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::debug::sheep_controls::sheep_controls::closure$0::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > >; module=590, inline_record=72988; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::critters::sheep::control::capture_sheep_snapshot::closure$0
// 信息 ID: 387bf70c9c292ca502dbdca2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::critters::sheep::control::capture_sheep_snapshot", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::critters::sheep::control::capture_sheep_snapshot::closure_env$0 <- core::iter::adapters::map::impl$2::next<country_core::resources::critters::sheep::control::SheepSnapshot,bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,country_core::resources::critters::sheep::control::capture_sheep_snapshot::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::critters::sheep::control::capture_sheep_snapshot::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::critters::sheep::control::SheepSnapshot,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,country_core::resources::critters::sheep::control::capture_sheep_snapshot::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::next<country_core::resources::critters::sheep::control::SheepSnapshot,bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,country_core::resources::critters::sheep::control::capture_sheep_snapshot::closure_env$0>; module=1643, inline_record=60484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::critters::sheep::control::impl$0::clone
// 信息 ID: 6655fdc94dc1b2151f8c5c50
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_game::save_game; module=552, inline_record=22384; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_game::save_game; module=552, inline_record=27344; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_game::save_game; module=552, inline_record=28524; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::load_game::load_game; module=564, inline_record=42108; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::debug::sheep_controls::sheep_controls::closure$0::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > >; module=590, inline_record=75412; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$12::clone<country_core::resources::critters::sheep::control::SheepSnapshot,alloc::alloc::Global>; module=1643, inline_record=250564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::critters::sheep::control::impl$2::eq
// 信息 ID: 5914c2586786ac3156bf3b75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_game::save_game; module=552, inline_record=26168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::critters::sheep::control::impl$3::clone
// 信息 ID: 1c8278417f262aa3c2f40eb8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_game::save_game; module=552, inline_record=21660; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_game::save_game; module=552, inline_record=26668; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_game::save_game; module=552, inline_record=27888; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::load_game::parse_saved_game_data; module=564, inline_record=23952; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::load_game::load_game; module=564, inline_record=41328; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::debug::sheep_controls::sheep_controls::closure$0::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > >; module=590, inline_record=74712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::critters::sheep::control::impl$5::eq
// 信息 ID: 2648678da2cbb063a9b849c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_game::save_game; module=552, inline_record=25996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
