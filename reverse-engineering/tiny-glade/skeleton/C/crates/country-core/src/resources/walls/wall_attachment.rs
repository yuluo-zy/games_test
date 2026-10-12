// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/walls/wall_attachment.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 0d8a772b905af4e51e3256bbeaf2d4856bae5167ab0f0de0084cae457d7e4828
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_attachment.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 126, 144, 439, 558, 607, 1610, 1636, 1658, 1669, 1694, 1802, 1803, 1804, 1812
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
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use itoa; // namespace candidate
//   package identities: [{"name":"itoa","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.3"}]
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
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 6, 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14260
// 首条位置: {"module":1803,"inlinee":82528}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":157048,"parent_record_offset":156992,"ancestor_record_offset":156992,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":96460,"parent_record_offset":96436,"ancestor_record_offset":96436,"inlinee":7758,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impls::impl$64::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":157020,"parent_record_offset":156992,"ancestor_record_offset":156992,"inlinee":7684,"source_line":132,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":96484,"parent_record_offset":96460,"ancestor_record_offset":96436,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":226740,"parent_record_offset":226716,"ancestor_record_offset":226692,"inlinee":82522,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":96508,"parent_record_offset":96484,"ancestor_record_offset":96436,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1669,"record_offset":32052,"rva":25015024,"va":5393724144}
// 编译实例 RVAs: 0x17db2f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1669,"record_offset":32252,"parent_record_offset":32052,"ancestor_record_offset":32052,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1669,"record_offset":32276,"parent_record_offset":32252,"ancestor_record_offset":32052,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1669,"record_offset":32300,"parent_record_offset":32276,"ancestor_record_offset":32052,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1669,"record_offset":32328,"parent_record_offset":32300,"ancestor_record_offset":32052,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 6, 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156FB
// 首条位置: {"module":1803,"inlinee":87803}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_core::de::value::impl$30::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":510056,"parent_record_offset":510028,"ancestor_record_offset":510028,"inlinee":87849,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::de::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":387004,"parent_record_offset":386980,"ancestor_record_offset":386980,"inlinee":37547,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":387280,"parent_record_offset":387256,"ancestor_record_offset":386980,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":387304,"parent_record_offset":387280,"ancestor_record_offset":386980,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":387328,"parent_record_offset":387304,"ancestor_record_offset":386980,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":387256,"parent_record_offset":387200,"ancestor_record_offset":386980,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":387228,"parent_record_offset":387200,"ancestor_record_offset":386980,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":387200,"parent_record_offset":387176,"ancestor_record_offset":386980,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":387176,"parent_record_offset":387152,"ancestor_record_offset":386980,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":510212,"parent_record_offset":510188,"ancestor_record_offset":510028,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":387104,"parent_record_offset":387076,"ancestor_record_offset":386980,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":510236,"parent_record_offset":510212,"ancestor_record_offset":510028,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":510260,"parent_record_offset":510236,"ancestor_record_offset":510028,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":510188,"parent_record_offset":510152,"ancestor_record_offset":510028,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":1803,"record_offset":510152,"parent_record_offset":510112,"ancestor_record_offset":510028,"inlinee":82536,"source_line":32,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":387076,"parent_record_offset":387052,"ancestor_record_offset":386980,"inlinee":10022,"source_line":1568,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::value::impl$30::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/value.rs
//     evidence={"module":1803,"record_offset":510084,"parent_record_offset":510056,"ancestor_record_offset":510028,"inlinee":87850,"source_line":683,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$21::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":387028,"parent_record_offset":387004,"ancestor_record_offset":386980,"inlinee":21295,"source_line":1248,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$26::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":387052,"parent_record_offset":387028,"ancestor_record_offset":386980,"inlinee":21297,"source_line":1420,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x88A0
// 首条位置: {"module":1669,"inlinee":34976}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15781
// 首条位置: {"module":1804,"inlinee":87937}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":224984,"parent_record_offset":224640,"ancestor_record_offset":224640,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":224728,"parent_record_offset":224640,"ancestor_record_offset":224640,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":226668,"parent_record_offset":224640,"ancestor_record_offset":224640,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":226836,"parent_record_offset":224640,"ancestor_record_offset":224640,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":224672,"parent_record_offset":224640,"ancestor_record_offset":224640,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":224764,"parent_record_offset":224640,"ancestor_record_offset":224640,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":224880,"parent_record_offset":224856,"ancestor_record_offset":224640,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":224856,"parent_record_offset":224820,"ancestor_record_offset":224640,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":225776,"parent_record_offset":225724,"ancestor_record_offset":224640,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":226036,"parent_record_offset":226012,"ancestor_record_offset":224640,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":226084,"parent_record_offset":226060,"ancestor_record_offset":224640,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_attachment::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":1804,"record_offset":225724,"parent_record_offset":225700,"ancestor_record_offset":224640,"inlinee":82528,"source_line":44,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$15::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/features/impl_serde.rs
//     evidence={"module":1804,"record_offset":226340,"parent_record_offset":226316,"ancestor_record_offset":224640,"inlinee":7648,"source_line":90,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":226764,"parent_record_offset":226740,"ancestor_record_offset":224640,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":226740,"parent_record_offset":226716,"ancestor_record_offset":224640,"inlinee":82522,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":226788,"parent_record_offset":226764,"ancestor_record_offset":224640,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":224696,"parent_record_offset":224672,"ancestor_record_offset":224640,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":224792,"parent_record_offset":224764,"ancestor_record_offset":224640,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":224940,"parent_record_offset":224916,"ancestor_record_offset":224640,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":224916,"parent_record_offset":224820,"ancestor_record_offset":224640,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":225472,"parent_record_offset":225448,"ancestor_record_offset":224640,"inlinee":7727,"source_line":71,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$64::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":225748,"parent_record_offset":225724,"ancestor_record_offset":224640,"inlinee":7684,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":226108,"parent_record_offset":226084,"ancestor_record_offset":224640,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":225192,"parent_record_offset":225168,"ancestor_record_offset":224640,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":224820,"parent_record_offset":224792,"ancestor_record_offset":224640,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":226060,"parent_record_offset":226036,"ancestor_record_offset":224640,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156AD
// 首条位置: {"module":1803,"inlinee":87725}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_core::de::impl$6::next_element
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":156032,"parent_record_offset":155992,"ancestor_record_offset":155992,"inlinee":15022,"source_line":1798,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1803,"record_offset":156108,"parent_record_offset":156080,"ancestor_record_offset":155992,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":156164,"parent_record_offset":156108,"ancestor_record_offset":155992,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":156192,"parent_record_offset":156108,"ancestor_record_offset":155992,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":156136,"parent_record_offset":156108,"ancestor_record_offset":155992,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":156216,"parent_record_offset":156192,"ancestor_record_offset":155992,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":156308,"parent_record_offset":156080,"ancestor_record_offset":155992,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1803,"record_offset":157356,"parent_record_offset":157332,"ancestor_record_offset":155992,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1803,"record_offset":157404,"parent_record_offset":157380,"ancestor_record_offset":155992,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_attachment::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":1803,"record_offset":156992,"parent_record_offset":156964,"ancestor_record_offset":155992,"inlinee":82528,"source_line":6,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$15::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/features/impl_serde.rs
//     evidence={"module":1803,"record_offset":156620,"parent_record_offset":156596,"ancestor_record_offset":155992,"inlinee":7648,"source_line":90,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":156056,"parent_record_offset":156032,"ancestor_record_offset":155992,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":156252,"parent_record_offset":156080,"ancestor_record_offset":155992,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":157740,"parent_record_offset":157716,"ancestor_record_offset":155992,"inlinee":7727,"source_line":71,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$64::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":157020,"parent_record_offset":156992,"ancestor_record_offset":155992,"inlinee":7684,"source_line":132,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":157428,"parent_record_offset":157404,"ancestor_record_offset":155992,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":156276,"parent_record_offset":156252,"ancestor_record_offset":155992,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$10::next_element_seed
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":156080,"parent_record_offset":156056,"ancestor_record_offset":155992,"inlinee":21316,"source_line":636,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":157380,"parent_record_offset":157356,"ancestor_record_offset":155992,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 6, 32, 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x33A6
// 首条位置: {"module":144,"inlinee":13222}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::wall_attachment::is_default
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":1802,"record_offset":86404,"parent_record_offset":86380,"ancestor_record_offset":86380,"inlinee":13197,"source_line":40,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_direct] serde_core::ser::impls::impl$46::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":144,"record_offset":355108,"parent_record_offset":355080,"ancestor_record_offset":355080,"inlinee":7414,"source_line":11,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_unit_variant
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":144,"record_offset":94280,"parent_record_offset":94260,"ancestor_record_offset":94260,"inlinee":7408,"source_line":214,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$1::serialize_unit_variant
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":1802,"record_offset":258832,"parent_record_offset":258804,"ancestor_record_offset":258804,"inlinee":7422,"source_line":188,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1802,"record_offset":259212,"parent_record_offset":259188,"ancestor_record_offset":258804,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1802,"record_offset":259236,"parent_record_offset":259212,"ancestor_record_offset":258804,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1802,"record_offset":259260,"parent_record_offset":259236,"ancestor_record_offset":258804,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1802,"record_offset":259188,"parent_record_offset":259152,"ancestor_record_offset":258804,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":144,"record_offset":355588,"parent_record_offset":355560,"ancestor_record_offset":355080,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1802,"record_offset":259088,"parent_record_offset":259056,"ancestor_record_offset":258804,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":144,"record_offset":355684,"parent_record_offset":355656,"ancestor_record_offset":355080,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":144,"record_offset":355656,"parent_record_offset":355620,"ancestor_record_offset":355080,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":144,"record_offset":355620,"parent_record_offset":355588,"ancestor_record_offset":355080,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1802,"record_offset":259152,"parent_record_offset":259120,"ancestor_record_offset":258804,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1802,"record_offset":259120,"parent_record_offset":259088,"ancestor_record_offset":258804,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1802,"record_offset":258960,"parent_record_offset":258928,"ancestor_record_offset":258804,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1802,"record_offset":258992,"parent_record_offset":258960,"ancestor_record_offset":258804,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1802,"record_offset":259024,"parent_record_offset":258992,"ancestor_record_offset":258804,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1802,"record_offset":258928,"parent_record_offset":258896,"ancestor_record_offset":258804,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1802,"record_offset":258896,"parent_record_offset":258864,"ancestor_record_offset":258804,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":144,"record_offset":355516,"parent_record_offset":355484,"ancestor_record_offset":355080,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":144,"record_offset":355452,"parent_record_offset":355420,"ancestor_record_offset":355080,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":144,"record_offset":355776,"parent_record_offset":355516,"ancestor_record_offset":355080,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":144,"record_offset":355560,"parent_record_offset":355516,"ancestor_record_offset":355080,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1802,"record_offset":259056,"parent_record_offset":259024,"ancestor_record_offset":258804,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":144,"record_offset":355484,"parent_record_offset":355452,"ancestor_record_offset":355080,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1802,"record_offset":86428,"parent_record_offset":86404,"ancestor_record_offset":86380,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1802,"record_offset":259676,"parent_record_offset":259512,"ancestor_record_offset":258804,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":144,"record_offset":355280,"parent_record_offset":355228,"ancestor_record_offset":355080,"inlinee":7419,"source_line":349,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1802,"record_offset":259700,"parent_record_offset":259676,"ancestor_record_offset":258804,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":144,"record_offset":355740,"parent_record_offset":355516,"ancestor_record_offset":355080,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":144,"record_offset":355352,"parent_record_offset":355228,"ancestor_record_offset":355080,"inlinee":7420,"source_line":352,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":144,"record_offset":94328,"parent_record_offset":94304,"ancestor_record_offset":94260,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] itoa::Buffer::format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itoa","version":"1.0.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itoa-1.0.3/src/lib.rs
//     evidence={"module":144,"record_offset":355200,"parent_record_offset":355164,"ancestor_record_offset":355080,"inlinee":7394,"source_line":86,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] itoa::impl$6::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itoa","version":"1.0.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itoa-1.0.3/src/lib.rs
//     evidence={"module":144,"record_offset":355228,"parent_record_offset":355200,"ancestor_record_offset":355080,"inlinee":7418,"source_line":125,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::write_u8
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":144,"record_offset":355164,"parent_record_offset":355136,"ancestor_record_offset":355080,"inlinee":7416,"source_line":1622,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":144,"record_offset":94304,"parent_record_offset":94280,"ancestor_record_offset":94260,"inlinee":7400,"source_line":190,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_u8
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":144,"record_offset":355136,"parent_record_offset":355108,"ancestor_record_offset":355080,"inlinee":7415,"source_line":121,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_str
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":1802,"record_offset":258864,"parent_record_offset":258832,"ancestor_record_offset":258804,"inlinee":7423,"source_line":168,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":144,"record_offset":355420,"parent_record_offset":355388,"ancestor_record_offset":355080,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":144,"record_offset":355388,"parent_record_offset":355164,"ancestor_record_offset":355080,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":126,"record_offset":1224,"rva":4191280,"va":5372900400}
// 编译实例 RVAs: 0x3ff430, 0x17d8ad0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140104100
//     owner=unresolved_namespace; count=2
//     target_aliases=583 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3126138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372900400,"raw_hex":"e8864cd0ff","size":5,"soft_fail":false,"target_va":5369774336,"va":5372900469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f9710
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1072772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372900400,"raw_hex":"e87ca1efff","size":5,"soft_fail":false,"target_va":5371827984,"va":5372900751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403fff60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372900400,"raw_hex":"e8d8080000","size":5,"soft_fail":false,"target_va":5372903264,"va":5372900995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14045f200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t392368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372900400,"raw_hex":"e8b0fc0500","size":5,"soft_fail":false,"target_va":5373293056,"va":5372900683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14045f950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t394053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372900400,"raw_hex":"e845030600","size":5,"soft_fail":false,"target_va":5373294928,"va":5372900870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140464a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t414847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372900400,"raw_hex":"e87f540600","size":5,"soft_fail":false,"target_va":5373315680,"va":5372900828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140469060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t432627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372900400,"raw_hex":"e8f3990600","size":5,"soft_fail":false,"target_va":5373333600,"va":5372900968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140469b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t435774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372900400,"raw_hex":"e83ea60600","size":5,"soft_fail":false,"target_va":5373336400,"va":5372900621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417dbc20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393713872,"raw_hex":"e80b2f0000","size":5,"soft_fail":false,"target_va":5393726496,"va":5393714448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b4050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t898214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393713872,"raw_hex":"e8a6b40d00","size":5,"soft_fail":false,"target_va":5394612304,"va":5393714085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b4790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t899785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393713872,"raw_hex":"e8c9ba0d00","size":5,"soft_fail":false,"target_va":5394614160,"va":5393714370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b79e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t912899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393713872,"raw_hex":"e803ee0d00","size":5,"soft_fail":false,"target_va":5394627040,"va":5393714136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b95c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t919987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393713872,"raw_hex":"e8b3090e00","size":5,"soft_fail":false,"target_va":5394634176,"va":5393714184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b9f80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t922339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393713872,"raw_hex":"e8e3120e00","size":5,"soft_fail":false,"target_va":5394636672,"va":5393714328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43452001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372900400,"raw_hex":"e861069702","size":5,"soft_fail":false,"target_va":5416353040,"va":5372901034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142da1cb0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t43656938","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5372900400,"raw_hex":"e9ea269a02","size":5,"soft_fail":false,"target_va":5416557744,"va":5372900801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::walls::wall_attachment::is_default
//     owner=private_source_ownership_candidate; count=14
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":126,"record_offset":1520,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":13197,"source_line":40,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":1896,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":2716,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":4577,"source_line":718,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":2556,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":126,"record_offset":2324,"parent_record_offset":2292,"ancestor_record_offset":1224,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":126,"record_offset":2192,"parent_record_offset":2164,"ancestor_record_offset":1224,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":126,"record_offset":2252,"parent_record_offset":2220,"ancestor_record_offset":1224,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":126,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":1224,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":126,"record_offset":2348,"parent_record_offset":2324,"ancestor_record_offset":1224,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":126,"record_offset":2220,"parent_record_offset":2192,"ancestor_record_offset":1224,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":126,"record_offset":2120,"parent_record_offset":2088,"ancestor_record_offset":1224,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":126,"record_offset":2292,"parent_record_offset":2120,"ancestor_record_offset":1224,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":126,"record_offset":2056,"parent_record_offset":2024,"ancestor_record_offset":1224,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":126,"record_offset":2412,"parent_record_offset":2120,"ancestor_record_offset":1224,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":126,"record_offset":2164,"parent_record_offset":2120,"ancestor_record_offset":1224,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":126,"record_offset":2088,"parent_record_offset":2056,"ancestor_record_offset":1224,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$19::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":126,"record_offset":1804,"parent_record_offset":1776,"ancestor_record_offset":1224,"inlinee":13199,"source_line":1878,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":126,"record_offset":1576,"parent_record_offset":1548,"ancestor_record_offset":1224,"inlinee":7407,"source_line":1878,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":126,"record_offset":1548,"parent_record_offset":1520,"ancestor_record_offset":1224,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":126,"record_offset":2648,"parent_record_offset":2624,"ancestor_record_offset":1224,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$105::eq
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":126,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":1224,"inlinee":11117,"source_line":20,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":1960,"parent_record_offset":1928,"ancestor_record_offset":1224,"inlinee":4570,"source_line":1865,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":2768,"parent_record_offset":2744,"ancestor_record_offset":1224,"inlinee":4580,"source_line":1875,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":1928,"parent_record_offset":1896,"ancestor_record_offset":1224,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":2744,"parent_record_offset":2716,"ancestor_record_offset":1224,"inlinee":4579,"source_line":669,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":126,"record_offset":2024,"parent_record_offset":1992,"ancestor_record_offset":1224,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":126,"record_offset":1992,"parent_record_offset":1960,"ancestor_record_offset":1224,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1669,"record_offset":6092,"rva":25004128,"va":5393713248}
// 编译实例 RVAs: 0x17d8860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$::end
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10485835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393713248,"raw_hex":"e8b5ff5fff","size":5,"soft_fail":false,"target_va":5383227792,"va":5393713622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417db6a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393713248,"raw_hex":"e88a2c0000","size":5,"soft_fail":false,"target_va":5393725088,"va":5393713681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b83a40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3846344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393713248,"raw_hex":"e8c8b03a00","size":5,"soft_fail":false,"target_va":5397559872,"va":5393713523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b84570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3849076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393713248,"raw_hex":"e874bb3a00","size":5,"soft_fail":false,"target_va":5397562736,"va":5393713655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b85bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3854948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393713248,"raw_hex":"e864d23a00","size":5,"soft_fail":false,"target_va":5397568432,"va":5393713479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b892b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3868939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393713248,"raw_hex":"e80b093b00","size":5,"soft_fail":false,"target_va":5397582512,"va":5393713568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b8b0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3876732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393713248,"raw_hex":"e87c273b00","size":5,"soft_fail":false,"target_va":5397590176,"va":5393713439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b8b980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3878697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393713248,"raw_hex":"e8292f3b00","size":5,"soft_fail":false,"target_va":5397592448,"va":5393713746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1669,"record_offset":6824,"parent_record_offset":6092,"ancestor_record_offset":6092,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_attachment::is_default
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":1669,"record_offset":6288,"parent_record_offset":6092,"ancestor_record_offset":6092,"inlinee":13197,"source_line":40,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$19::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1669,"record_offset":6564,"parent_record_offset":6536,"ancestor_record_offset":6092,"inlinee":13199,"source_line":1878,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1669,"record_offset":6336,"parent_record_offset":6312,"ancestor_record_offset":6092,"inlinee":7407,"source_line":1878,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1669,"record_offset":6312,"parent_record_offset":6288,"ancestor_record_offset":6092,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1669,"record_offset":6788,"parent_record_offset":6764,"ancestor_record_offset":6092,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$105::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1669,"record_offset":6416,"parent_record_offset":6392,"ancestor_record_offset":6092,"inlinee":11117,"source_line":20,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::impl$2::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13BBD
// 首条位置: {"module":1610,"inlinee":80829}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::impl$7::eq
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x694B
// 首条位置: {"module":439,"inlinee":26955}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":439,"record_offset":28376,"parent_record_offset":28276,"ancestor_record_offset":28276,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00439.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_attachment::impl$20::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":439,"record_offset":28348,"parent_record_offset":28276,"ancestor_record_offset":28276,"inlinee":26957,"source_line":44,"archive":"_pdb/C/modules/00439.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$105::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":439,"record_offset":28316,"parent_record_offset":28276,"ancestor_record_offset":28276,"inlinee":11117,"source_line":20,"archive":"_pdb/C/modules/00439.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14268
// 首条位置: {"module":1803,"inlinee":82536}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":510188,"parent_record_offset":510152,"ancestor_record_offset":510152,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":510212,"parent_record_offset":510188,"ancestor_record_offset":510152,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":510236,"parent_record_offset":510212,"ancestor_record_offset":510152,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":510260,"parent_record_offset":510236,"ancestor_record_offset":510152,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14259
// 首条位置: {"module":1669,"inlinee":82521}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1669,"record_offset":6020,"parent_record_offset":5964,"ancestor_record_offset":5964,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1669,"record_offset":6052,"parent_record_offset":5964,"ancestor_record_offset":5964,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1669,"record_offset":5992,"parent_record_offset":5964,"ancestor_record_offset":5964,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1669,"record_offset":5712,"rva":25003696,"va":5393712816}
// 编译实例 RVAs: 0x17d86b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde_core..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t22847404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393712816,"raw_hex":"e8ac9f5c01","size":5,"soft_fail":false,"target_va":5416560512,"va":5393713103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..de..VariantDeserializer$u20$as$u20$serde_core..de..VariantAccess$GT$::unit_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10462460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393712816,"raw_hex":"e8045b60ff","size":5,"soft_fail":false,"target_va":5383250560,"va":5393713015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409de280
//     owner=unresolved_namespace; count=3
//     target_aliases=16 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14656873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393712816,"raw_hex":"e8975a20ff","size":5,"soft_fail":false,"target_va":5379056256,"va":5393713124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bae980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4022835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393712816,"raw_hex":"e833623d00","size":5,"soft_fail":false,"target_va":5397735808,"va":5393712968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141be6e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4253434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393712816,"raw_hex":"e8fae64000","size":5,"soft_fail":false,"target_va":5397966368,"va":5393712929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::de::_$LT$impl$u20$serde_json..value..Value$GT$::unexpected
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t22848100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393712816,"raw_hex":"e864a25c01","size":5,"soft_fail":false,"target_va":5416561184,"va":5393713079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::value::de::impl$4::deserialize_enum
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1669,"record_offset":5904,"parent_record_offset":5712,"ancestor_record_offset":5712,"inlinee":33271,"source_line":328,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1669,"record_offset":6020,"parent_record_offset":5964,"ancestor_record_offset":5712,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1669,"record_offset":6052,"parent_record_offset":5964,"ancestor_record_offset":5712,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":1669,"record_offset":5964,"parent_record_offset":5904,"ancestor_record_offset":5712,"inlinee":82521,"source_line":32,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1669,"record_offset":5992,"parent_record_offset":5964,"ancestor_record_offset":5712,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::impl$8::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13BBB
// 首条位置: {"module":1610,"inlinee":80827}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::is_default
// 记录形式: inline_instance, inlinee_declaration; instances=37
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x338D
// 首条位置: {"module":126,"inlinee":13197}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":126,"record_offset":1548,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$19::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":126,"record_offset":1804,"parent_record_offset":1776,"ancestor_record_offset":1748,"inlinee":13199,"source_line":1878,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":126,"record_offset":1576,"parent_record_offset":1548,"ancestor_record_offset":1520,"inlinee":7407,"source_line":1878,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":126,"record_offset":2648,"parent_record_offset":2624,"ancestor_record_offset":2600,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$105::eq
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":126,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":1608,"inlinee":11117,"source_line":20,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::impl$15::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1610,"record_offset":372,"rva":21994000,"va":5390703120}
// 编译实例 RVAs: 0x14f9a10, 0x17d7ee0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_tuple_field1_finish
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5885506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390703120,"raw_hex":"e8be31a6ff","size":5,"soft_fail":false,"target_va":5384817664,"va":5390703165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::impl$20::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x694D
// 首条位置: {"module":439,"inlinee":26957}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::WallCornerAttachment::is_attached
// 记录形式: compiled_procedure; instances=1
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":558,"record_offset":72988,"rva":11507136,"va":5380216256}
// 编译实例 RVAs: 0xaf95c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::WallCornerAttachment::from_rectangle_corner_id
// 记录形式: compiled_procedure; instances=1
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":558,"record_offset":73320,"rva":11507168,"va":5380216288}
// 编译实例 RVAs: 0xaf95e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36405877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380216288,"raw_hex":"e875822b02","size":5,"soft_fail":false,"target_va":5416622208,"va":5380216326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::WallCornerAttachment::to_rectangle_corner_id
// 记录形式: compiled_procedure; instances=1
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":558,"record_offset":73148,"rva":11507152,"va":5380216272}
// 编译实例 RVAs: 0xaf95d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::stairs::visual::impl$6::clone_behavior
// 记录形式: compiled_procedure; instances=1
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":558,"record_offset":268,"rva":1065216,"va":5369774336}
// 编译实例 RVAs: 0x104100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::impl$1::default
// 记录形式: compiled_procedure; instances=1
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":558,"record_offset":564,"rva":1065216,"va":5369774336}
// 编译实例 RVAs: 0x104100
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
// 完整结构及原始类型记录: wall_attachment.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::wall_attachment::WallAttachmentAnchor
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::wall_attachment::WallAttachmentAnchor>", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "country_core::resources::walls::wall_attachment::WallAttachmentAnchor"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<country_core::resources::walls::wall_attachment::WallAttachmentAnchor>", "constructor": "core::fmt::impl$80::fmt", "arguments": ["country_core::resources::walls::wall_attachment::WallAttachmentAnchor"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::wall_attachment::WallAttachmentAnchor>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["country_core::resources::walls::wall_attachment::WallAttachmentAnchor"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::wall_attachment::WallAttachmentSide
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::wall_attachment::WallAttachmentSide>", "constructor": "serde_json::ser::impl$9::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "country_core::resources::walls::wall_attachment::WallAttachmentSide"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::wall_attachment::WallAttachmentSide>", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "country_core::resources::walls::wall_attachment::WallAttachmentSide"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<country_core::resources::walls::wall_attachment::WallAttachmentSide>", "constructor": "core::fmt::impl$80::fmt", "arguments": ["country_core::resources::walls::wall_attachment::WallAttachmentSide"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::wall_attachment::WallAttachmentSide>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["country_core::resources::walls::wall_attachment::WallAttachmentSide"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::wall_attachment::WallCornerAttachment
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::wall_attachment::WallCornerAttachment>", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "country_core::resources::walls::wall_attachment::WallCornerAttachment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_attachment.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_attachment.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 7; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)
// 候选参数类型: ["ref$<country_core::resources::clutter_packs::ClutterPackObject>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::OnCircleRotated>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::OnCircleRotated>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::OnCircleRotated>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,ref$<country_core::geometry::instanced_wall::InstancedWall> >,bevy_ecs::query::filter::Changed<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,bevy_ecs::lifecycle::RemovedComponents<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,bevy_ecs::query::filter::Changed<country_core::resources::roofs::roof_shape::Roof> > >,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,bevy_ecs::lifecycle::RemovedComponents<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,tuple$<bevy_ecs::query::filter::With<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,bevy_ecs::query::filter::Added<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref$<country_core::components::AutoClutterConvertableToDecoClutter> >,bevy_ecs::query::filter::Changed<utils::components::transform::Transform> >,bevy_ecs::lifecycle::RemovedComponents<country_core::components::AutoClutterConvertableToDecoClutter>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,ref$<country_core::geometry::instanced_wall::InstancedWall> >,bevy_ecs::query::filter::Changed<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >", "bevy_ecs::lifecycle::RemovedComponents<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,bevy_ecs::query::filter::Changed<country_core::resources::roofs::roof_shape::Roof> > >,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >", "bevy_ecs::lifecycle::RemovedComponents<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>", "bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,tuple$<bevy_ecs::query::filter::With<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,bevy_ecs::query::filter::Added<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >", "bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>", "bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref$<country_core::components::AutoClutterConvertableToDecoClutter> >,bevy_ecs::query::filter::Changed<utils::components::transform::Transform> >", "bevy_ecs::lifecycle::RemovedComponents<country_core::components::AutoClutterConvertableToDecoClutter>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,ref$<country_core::geometry::instanced_wall::InstancedWall> >,bevy_ecs::query::filter::Changed<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,bevy_ecs::query::filter::Changed<country_core::resources::roofs::roof_shape::Roof> > >,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,tuple$<bevy_ecs::query::filter::With<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,bevy_ecs::query::filter::Added<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref$<country_core::components::AutoClutterConvertableToDecoClutter> >,bevy_ecs::query::filter::Changed<utils::components::transform::Transform> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>

// ------------------------------------------------------------------
// 函数: country_core::resources::stairs::visual::impl$6::clone_behavior
// 信息 ID: 502af7b6d126d29715285af6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 230 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::drawable::NaniActiveInstance> @ 0x2db39a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::WoodenGate> @ 0x2dbafa -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::FollowMouse> @ 0x2dc94a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::systems::prefab::PrefabInstanceData> @ 0x2dd0aa -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::resources::critters::sheep::SheepAccessory> @ 0x2dd80a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::Ui2DFollowMouse> @ 0x2ddf6a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::Ui2dExtent> @ 0x2de6ba -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::AutoClutter> @ 0x2dee1a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::systems::look_at_camera::LookAtCamera> @ 0x2df57a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::SnappingToSegmentPreview> @ 0x2dfcca -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::SemiTransparent> @ 0x2e041a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> @ 0x2e0b6a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> @ 0x2e1a1a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier> @ 0x2e216a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::TestCube> @ 0x2e28ca -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::DashedLineLength> @ 0x2e301a -> 0x104100 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::WallCornerAttachment::from_rectangle_corner_id
// 信息 ID: 47599263402166537f0fad80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0x14d8bc1 -> 0xaf95e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::WallCornerAttachment::is_attached
// 信息 ID: cd8b4f7fbc4fa34d157128a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::derive_decorator_subtype::derive_decorator_subtype @ 0x8d9214 -> 0xaf95c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::derive_decorator_subtype::derive_decorator_subtype @ 0x8d90dc -> 0xaf95c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part @ 0x14d21c0 -> 0xaf95c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner @ 0x250b130 -> 0xaf95c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::WallCornerAttachment::to_rectangle_corner_id
// 信息 ID: 3fd2793b01c5ace6191c327d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x8cb75e -> 0xaf95d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x8dd130 -> 0xaf95d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x8e1cd2 -> 0xaf95d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::deserialize
// 信息 ID: b624cdc03e956f3677b9fa4d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Visitor>; module=1803, inline_record=156992; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::walls::decorator::_::impl$0::deserialize::__Visitor>; module=1803, inline_record=211552; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::walls::decorator::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=96044; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::walls::decorator::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=96436; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=225724; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=226692; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 54837eb3ee127502b15b94e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::variant_seed<core::marker::PhantomData<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Field> >; module=1803, inline_record=510152; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: f53271138203834b2481089e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Field> > @ 0x1ba7213 -> 0x17db2f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 8d5c781eb4424c79cef24035
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Field> >; module=1803, inline_record=386980; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::variant_seed<core::marker::PhantomData<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Field> >; module=1803, inline_record=510028; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 53ae2ae94e1a7518c9646fa0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Visitor>; module=1669, inline_record=35828; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 43d8cdf81f537bc5dd928db8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::wall_attachment::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1669, inline_record=5964; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_enum<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=321708; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 467f45a00158f525a830ed9f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=224640; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 933c72db61cc7025a737de9f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Visitor>; module=1803, inline_record=155992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 信息 ID: 92b81cc82ab443bdef7cc30f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::resources::walls::wall_attachment::_::impl$0::deserialize -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::visit_array<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Visitor> @ 0x1b98abc -> 0x17d86b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::serialize
// 信息 ID: 60f465c5900519f14ed009f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::wall_attachment::WallAttachmentSide>; module=144, inline_record=94260; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::wall_attachment::WallCornerAttachment>; module=144, inline_record=355080; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::wall_attachment::WallCornerAttachment>; module=1694, inline_record=89596; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::wall_attachment::WallAttachmentSide>; module=1694, inline_record=301540; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::wall_attachment::WallAttachmentAnchor>; module=1802, inline_record=86380; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::wall_attachment::WallAttachmentSide>; module=1802, inline_record=258804; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::wall_attachment::WallAttachmentSide>; module=1802, inline_record=261224; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: 4110bb5e4c6cb464616d4057
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::walls::wall_attachment::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::wall_attachment::WallAttachmentAnchor> @ 0x4642a2 -> 0x3ff430 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::wall_attachment::WallAttachmentAnchor> @ 0x18b13c2 -> 0x17d8ad0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: 726f7766ef94fe004616a020
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::walls::wall_attachment::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::wall_attachment::WallAttachmentAnchor> @ 0x1b8178b -> 0x17d8860 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::impl$15::fmt
// 信息 ID: b7a72b62db16af1daf5b11c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::impl$1::default
// 信息 ID: 2f3f4a5ef3075df3027c228c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 230 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::drawable::NaniActiveInstance> @ 0x2db39a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::WoodenGate> @ 0x2dbafa -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::FollowMouse> @ 0x2dc94a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::systems::prefab::PrefabInstanceData> @ 0x2dd0aa -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::resources::critters::sheep::SheepAccessory> @ 0x2dd80a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::Ui2DFollowMouse> @ 0x2ddf6a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::Ui2dExtent> @ 0x2de6ba -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::AutoClutter> @ 0x2dee1a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::systems::look_at_camera::LookAtCamera> @ 0x2df57a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::SnappingToSegmentPreview> @ 0x2dfcca -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::SemiTransparent> @ 0x2e041a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> @ 0x2e0b6a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> @ 0x2e1a1a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier> @ 0x2e216a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::TestCube> @ 0x2e28ca -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::DashedLineLength> @ 0x2e301a -> 0x104100 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::impl$20::eq
// 信息 ID: 0cfbbbd263254f3d9e2e95ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::decorator::impl$134::eq; module=439, inline_record=28348; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::impl$82::extend<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0> >; module=1658, inline_record=10616; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::impl$2::fmt
// 信息 ID: ceba3416c1bc960454e918e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$80::fmt<country_core::resources::walls::wall_attachment::WallAttachmentAnchor>; module=1610, inline_record=1076; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$80::fmt<country_core::resources::walls::wall_attachment::WallAttachmentAnchor>; module=1669, inline_record=33760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::impl$7::eq
// 信息 ID: 42d275d8dd50aefd7dca7282
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::decorator::impl$134::eq; module=439, inline_record=28276; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::convert_before_after_states_to_changes; module=607, inline_record=127064; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::impl$82::extend<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0> >; module=1658, inline_record=10544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::impl$8::fmt
// 信息 ID: a61231e7cde997e46e927e2d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$80::fmt<country_core::resources::walls::wall_attachment::WallAttachmentSide>; module=1610, inline_record=688; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$80::fmt<country_core::resources::walls::wall_attachment::WallAttachmentSide>; module=1669, inline_record=33552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::is_default
// 信息 ID: 1d3d5537b75132c6f4725d49
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 24 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::wall_attachment::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=126, inline_record=1520; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_attachment::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=126, inline_record=1608; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_attachment::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=126, inline_record=1692; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_attachment::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=126, inline_record=1748; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_attachment::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=126, inline_record=1840; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_attachment::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=126, inline_record=2472; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_attachment::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=126, inline_record=2600; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_attachment::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=1669, inline_record=6288; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
