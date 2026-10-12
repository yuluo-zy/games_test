// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/terrain_stroke.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 80d6acaaf0e434fdb09c984a356111e841908bb10a573d691f781e9ae587ac62
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: terrain_stroke.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 558, 600, 614, 1766, 1803, 1804
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
// 函数符号: country_core::resources::terrain_stroke::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14EF3
// 首条位置: {"module":1803,"inlinee":85747}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1766,"record_offset":61408,"rva":28129952,"va":5396839072}
// 编译实例 RVAs: 0x1ad3aa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1766,"record_offset":61600,"parent_record_offset":61408,"ancestor_record_offset":61408,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1766,"record_offset":61624,"parent_record_offset":61600,"ancestor_record_offset":61408,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1766,"record_offset":61648,"parent_record_offset":61624,"ancestor_record_offset":61408,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1766,"record_offset":61676,"parent_record_offset":61648,"ancestor_record_offset":61408,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15711
// 首条位置: {"module":1803,"inlinee":87825}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":429172,"parent_record_offset":429148,"ancestor_record_offset":429148,"inlinee":37547,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":429448,"parent_record_offset":429424,"ancestor_record_offset":429148,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":429472,"parent_record_offset":429448,"ancestor_record_offset":429148,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":429496,"parent_record_offset":429472,"ancestor_record_offset":429148,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":429424,"parent_record_offset":429368,"ancestor_record_offset":429148,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":429396,"parent_record_offset":429368,"ancestor_record_offset":429148,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":429368,"parent_record_offset":429344,"ancestor_record_offset":429148,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":429344,"parent_record_offset":429320,"ancestor_record_offset":429148,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":429272,"parent_record_offset":429244,"ancestor_record_offset":429148,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":429244,"parent_record_offset":429220,"ancestor_record_offset":429148,"inlinee":10022,"source_line":1568,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$21::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":429196,"parent_record_offset":429172,"ancestor_record_offset":429148,"inlinee":21295,"source_line":1248,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$26::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":429220,"parent_record_offset":429196,"ancestor_record_offset":429148,"inlinee":21297,"source_line":1420,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x953B
// 首条位置: {"module":1766,"inlinee":38203}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15789
// 首条位置: {"module":1804,"inlinee":87945}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":257952,"parent_record_offset":257584,"ancestor_record_offset":257584,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1804,"record_offset":259280,"parent_record_offset":257584,"ancestor_record_offset":257584,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":257696,"parent_record_offset":257584,"ancestor_record_offset":257584,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":259252,"parent_record_offset":257584,"ancestor_record_offset":257584,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":259112,"parent_record_offset":257584,"ancestor_record_offset":257584,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":260036,"parent_record_offset":257584,"ancestor_record_offset":257584,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":257640,"parent_record_offset":257584,"ancestor_record_offset":257584,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":257732,"parent_record_offset":257584,"ancestor_record_offset":257584,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":260332,"parent_record_offset":260308,"ancestor_record_offset":257584,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":260356,"parent_record_offset":260332,"ancestor_record_offset":257584,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":260380,"parent_record_offset":260356,"ancestor_record_offset":257584,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":260308,"parent_record_offset":260220,"ancestor_record_offset":257584,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":259376,"parent_record_offset":259352,"ancestor_record_offset":257584,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":259352,"parent_record_offset":259328,"ancestor_record_offset":257584,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":259328,"parent_record_offset":259304,"ancestor_record_offset":257584,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":257848,"parent_record_offset":257824,"ancestor_record_offset":257584,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1804,"record_offset":259400,"parent_record_offset":259376,"ancestor_record_offset":257584,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":257824,"parent_record_offset":257788,"ancestor_record_offset":257584,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1804,"record_offset":259304,"parent_record_offset":259280,"ancestor_record_offset":257584,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":258492,"parent_record_offset":258440,"ancestor_record_offset":257584,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":1804,"record_offset":258440,"parent_record_offset":258416,"ancestor_record_offset":257584,"inlinee":35709,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":259184,"parent_record_offset":259160,"ancestor_record_offset":257584,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":259496,"parent_record_offset":259472,"ancestor_record_offset":257584,"inlinee":7793,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":259160,"parent_record_offset":259136,"ancestor_record_offset":257584,"inlinee":7805,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":259660,"parent_record_offset":259636,"ancestor_record_offset":257584,"inlinee":35640,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":259208,"parent_record_offset":259184,"ancestor_record_offset":257584,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":257664,"parent_record_offset":257640,"ancestor_record_offset":257584,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":257760,"parent_record_offset":257732,"ancestor_record_offset":257584,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":257908,"parent_record_offset":257884,"ancestor_record_offset":257584,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":257884,"parent_record_offset":257788,"ancestor_record_offset":257584,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":258156,"parent_record_offset":258132,"ancestor_record_offset":257584,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$70::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":258464,"parent_record_offset":258440,"ancestor_record_offset":257584,"inlinee":9914,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":258752,"parent_record_offset":258728,"ancestor_record_offset":257584,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":257788,"parent_record_offset":257760,"ancestor_record_offset":257584,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156AC
// 首条位置: {"module":1803,"inlinee":87724}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":152768,"parent_record_offset":152068,"ancestor_record_offset":152068,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":153056,"parent_record_offset":152068,"ancestor_record_offset":152068,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$6::next_element
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":152100,"parent_record_offset":152068,"ancestor_record_offset":152068,"inlinee":15022,"source_line":1798,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":152952,"parent_record_offset":152928,"ancestor_record_offset":152068,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":152976,"parent_record_offset":152952,"ancestor_record_offset":152068,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":153000,"parent_record_offset":152976,"ancestor_record_offset":152068,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":152928,"parent_record_offset":152840,"ancestor_record_offset":152068,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":152868,"parent_record_offset":152840,"ancestor_record_offset":152068,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":152840,"parent_record_offset":152816,"ancestor_record_offset":152068,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":152816,"parent_record_offset":152792,"ancestor_record_offset":152068,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1803,"record_offset":152184,"parent_record_offset":152156,"ancestor_record_offset":152068,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1803,"record_offset":152896,"parent_record_offset":152868,"ancestor_record_offset":152068,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":152792,"parent_record_offset":152768,"ancestor_record_offset":152068,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":152240,"parent_record_offset":152184,"ancestor_record_offset":152068,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":152268,"parent_record_offset":152184,"ancestor_record_offset":152068,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":152212,"parent_record_offset":152184,"ancestor_record_offset":152068,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":152292,"parent_record_offset":152268,"ancestor_record_offset":152068,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":152384,"parent_record_offset":152156,"ancestor_record_offset":152068,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":1803,"record_offset":153356,"parent_record_offset":153332,"ancestor_record_offset":152068,"inlinee":35709,"source_line":38,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":152128,"parent_record_offset":152100,"ancestor_record_offset":152068,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":152328,"parent_record_offset":152156,"ancestor_record_offset":152068,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":152352,"parent_record_offset":152328,"ancestor_record_offset":152068,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$70::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":153380,"parent_record_offset":153356,"ancestor_record_offset":152068,"inlinee":9914,"source_line":132,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":152696,"parent_record_offset":152672,"ancestor_record_offset":152068,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$10::next_element_seed
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":152156,"parent_record_offset":152128,"ancestor_record_offset":152068,"inlinee":21316,"source_line":636,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::terrain_stroke::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=1
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1766,"record_offset":43576,"rva":28123488,"va":5396832608}
// 编译实例 RVAs: 0x1ad2160
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$::end
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13605094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396832608,"raw_hex":"e81a6730ff","size":5,"soft_fail":false,"target_va":5383227792,"va":5396832881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ad40b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_stroke.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396832608,"raw_hex":"e8701e0000","size":5,"soft_fail":false,"target_va":5396840624,"va":5396832827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b84570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_stroke.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t730001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396832608,"raw_hex":"e891230b00","size":5,"soft_fail":false,"target_va":5397562736,"va":5396832730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b863f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_stroke.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t737845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396832608,"raw_hex":"e835420b00","size":5,"soft_fail":false,"target_va":5397570544,"va":5396832694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b86650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_stroke.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t738381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396832608,"raw_hex":"e84d440b00","size":5,"soft_fail":false,"target_va":5397571152,"va":5396832766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1766,"record_offset":43760,"parent_record_offset":43576,"ancestor_record_offset":43576,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::terrain_stroke::impl$2::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9871
// 首条位置: {"module":614,"inlinee":39025}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":614,"record_offset":68216,"parent_record_offset":68172,"ancestor_record_offset":68172,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":614,"record_offset":68652,"parent_record_offset":68628,"ancestor_record_offset":68172,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":614,"record_offset":68676,"parent_record_offset":68652,"ancestor_record_offset":68172,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":614,"record_offset":68700,"parent_record_offset":68676,"ancestor_record_offset":68172,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":614,"record_offset":68628,"parent_record_offset":68448,"ancestor_record_offset":68172,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":614,"record_offset":68388,"parent_record_offset":68360,"ancestor_record_offset":68172,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":614,"record_offset":68448,"parent_record_offset":68416,"ancestor_record_offset":68172,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":614,"record_offset":68416,"parent_record_offset":68388,"ancestor_record_offset":68172,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":614,"record_offset":68484,"parent_record_offset":68448,"ancestor_record_offset":68172,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":614,"record_offset":68316,"parent_record_offset":68248,"ancestor_record_offset":68172,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":614,"record_offset":68288,"parent_record_offset":68248,"ancestor_record_offset":68172,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":614,"record_offset":68360,"parent_record_offset":68316,"ancestor_record_offset":68172,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":614,"record_offset":68248,"parent_record_offset":68216,"ancestor_record_offset":68172,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":614,"record_offset":68508,"parent_record_offset":68484,"ancestor_record_offset":68172,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":614,"record_offset":68588,"parent_record_offset":68532,"ancestor_record_offset":68172,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":614,"record_offset":68532,"parent_record_offset":68508,"ancestor_record_offset":68172,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":614,"record_offset":68560,"parent_record_offset":68532,"ancestor_record_offset":68172,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":614,"record_offset":68756,"parent_record_offset":68316,"ancestor_record_offset":68172,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":614,"record_offset":68780,"parent_record_offset":68756,"ancestor_record_offset":68172,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::terrain_stroke::TerrainStroke::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8867
// 首条位置: {"module":558,"inlinee":34919}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":558,"record_offset":71604,"parent_record_offset":71572,"ancestor_record_offset":71572,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":558,"record_offset":71684,"parent_record_offset":71660,"ancestor_record_offset":71572,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":558,"record_offset":71708,"parent_record_offset":71684,"ancestor_record_offset":71572,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":558,"record_offset":71732,"parent_record_offset":71708,"ancestor_record_offset":71572,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":558,"record_offset":71660,"parent_record_offset":71632,"ancestor_record_offset":71572,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":558,"record_offset":71632,"parent_record_offset":71604,"ancestor_record_offset":71572,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00558.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::terrain_stroke::TerrainStroke::insert_or_morph_last_point
// 记录形式: compiled_procedure; instances=1
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":600,"record_offset":76232,"rva":12897936,"va":5381607056}
// 编译实例 RVAs: 0xc4ce90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f940
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: terrain_stroke.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34744963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381607056,"raw_hex":"e8832a1202","size":5,"soft_fail":false,"target_va":5416352064,"va":5381607096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":76416,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":77044,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":600,"record_offset":76620,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":600,"record_offset":77216,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":600,"record_offset":77128,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":76496,"parent_record_offset":76472,"ancestor_record_offset":76232,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":76544,"parent_record_offset":76520,"ancestor_record_offset":76232,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":76520,"parent_record_offset":76496,"ancestor_record_offset":76232,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":76472,"parent_record_offset":76440,"ancestor_record_offset":76232,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":76440,"parent_record_offset":76416,"ancestor_record_offset":76232,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":600,"record_offset":76768,"parent_record_offset":76744,"ancestor_record_offset":76232,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":600,"record_offset":76684,"parent_record_offset":76644,"ancestor_record_offset":76232,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":76584,"parent_record_offset":76440,"ancestor_record_offset":76232,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":600,"record_offset":77068,"parent_record_offset":77044,"ancestor_record_offset":76232,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":600,"record_offset":77092,"parent_record_offset":77068,"ancestor_record_offset":76232,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":600,"record_offset":76708,"parent_record_offset":76644,"ancestor_record_offset":76232,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":600,"record_offset":76644,"parent_record_offset":76620,"ancestor_record_offset":76232,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":600,"record_offset":76744,"parent_record_offset":76708,"ancestor_record_offset":76232,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00600.jsonl.gz"}
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
// 完整结构及原始类型记录: terrain_stroke.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::terrain_stroke::TerrainStroke
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 13
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>", "constructor": "tuple$", "arguments": ["country_core::resources::terrain_editing::TerrainStrokeId", "country_core::resources::terrain_stroke::TerrainStroke"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTable", "arguments": ["tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<hashbrown::raw::RawTable<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["hashbrown::raw::RawTable<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["country_core::resources::terrain_editing::TerrainStrokeId", "country_core::resources::terrain_stroke::TerrainStroke", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::HashMap<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::HashMap<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::resources::terrain_stroke::TerrainStroke>", "constructor": "core::option::Option", "arguments": ["country_core::resources::terrain_stroke::TerrainStroke"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::resources::terrain_stroke::TerrainStroke> >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::resources::terrain_stroke::TerrainStroke>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<country_core::resources::terrain_stroke::TerrainStroke> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<country_core::resources::terrain_stroke::TerrainStroke> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<hashbrown::raw::RawTable<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global> >", "constructor": "ref_mut$", "arguments": ["hashbrown::raw::RawTable<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<usize,ref_mut$<hashbrown::raw::RawTable<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global> > >", "constructor": "tuple$", "arguments": ["usize", "ref_mut$<hashbrown::raw::RawTable<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$16::clone_from_impl::closure_env$0<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global>", "constructor": "hashbrown::raw::impl$16::clone_from_impl::closure_env$0", "arguments": ["tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::scopeguard::ScopeGuard<tuple$<usize,ref_mut$<hashbrown::raw::RawTable<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global> > >,hashbrown::raw::impl$16::clone_from_impl::closure_env$0<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global> >", "constructor": "hashbrown::scopeguard::ScopeGuard", "arguments": ["tuple$<usize,ref_mut$<hashbrown::raw::RawTable<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global> > >", "hashbrown::raw::impl$16::clone_from_impl::closure_env$0<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<hashbrown::scopeguard::ScopeGuard<tuple$<usize,ref_mut$<hashbrown::raw::RawTable<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global> > >,hashbrown::raw::impl$16::clone_from_impl::closure_env$0<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["hashbrown::scopeguard::ScopeGuard<tuple$<usize,ref_mut$<hashbrown::raw::RawTable<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global> > >,hashbrown::raw::impl$16::clone_from_impl::closure_env$0<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$13::clone<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global>", "constructor": "hashbrown::raw::impl$13::clone", "arguments": ["tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::terrain_editing::TerrainStrokeId", "country_core::resources::terrain_stroke::TerrainStroke", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::clear<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTable::clear", "arguments": ["tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState>", "constructor": "serde_core::de::impls::impl$204::deserialize::MapVisitor", "arguments": ["country_core::resources::terrain_editing::TerrainStrokeId", "country_core::resources::terrain_stroke::TerrainStroke", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$4::deserialize_map<serde_core::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState> >", "constructor": "serde_json::value::de::impl$4::deserialize_map", "arguments": ["serde_core::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::terrain_editing::TerrainStrokeId", "country_core::resources::terrain_stroke::TerrainStroke", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<country_core::resources::terrain_stroke::TerrainStroke>", "constructor": "ref$", "arguments": ["country_core::resources::terrain_stroke::TerrainStroke"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<std::collections::hash::map::HashMap<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["std::collections::hash::map::HashMap<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::Serializer::collect_map<serde_json::value::ser::Serializer,ref$<country_core::resources::terrain_editing::TerrainStrokeId>,ref$<country_core::resources::terrain_stroke::TerrainStroke>,ref$<std::collections::hash::map::HashMap<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState,alloc::alloc::Global> > >", "constructor": "serde_core::ser::Serializer::collect_map", "arguments": ["serde_json::value::ser::Serializer", "ref$<country_core::resources::terrain_editing::TerrainStrokeId>", "ref$<country_core::resources::terrain_stroke::TerrainStroke>", "ref$<std::collections::hash::map::HashMap<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<std::collections::hash::map::HashMap<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["std::collections::hash::map::HashMap<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$3::deserialize_any<serde_core::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState> >", "constructor": "serde_json::value::de::impl$3::deserialize_any", "arguments": ["serde_core::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: terrain_stroke.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/terrain-editing/source-ownership.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/terrain_stroke.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: terrain_stroke.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::systems::paths::path_brush_preview_size::SetSizeBasedOnAppModePath> >,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::systems::paths::path_brush_preview_size::SetSizeBasedOnAppModePath> >", "bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::systems::paths::path_brush_preview_size::SetSizeBasedOnAppModePath>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::systems::paths::path_brush_preview_size::SetSizeBasedOnAppModePath> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>
// [文件级接口候选，具体函数归属 unknown] asset::prefab::PrefabName (*)(ref$<asset::prefab::PrefabName>)
// 候选参数类型: ["ref$<asset::prefab::PrefabName>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<bevy_input::mouse::MouseButton> (*)(ref$<enum2$<bevy_input::mouse::MouseButton> >)
// 候选参数类型: ["ref$<enum2$<bevy_input::mouse::MouseButton> >"]
// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<country_core::resources::walls::WallId>)
// 候选参数类型: ["ref$<country_core::resources::walls::WallId>"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<usize>,ref$<usize>)
// 候选参数类型: ["ref$<usize>", "ref$<usize>"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::decorator::DecoratorId (*)(ref$<country_core::resources::walls::decorator::DecoratorId>)
// 候选参数类型: ["ref$<country_core::resources::walls::decorator::DecoratorId>"]

// ------------------------------------------------------------------
// 函数: country_core::resources::terrain_stroke::TerrainStroke::insert_or_morph_last_point
// 信息 ID: 4e0f856be3701ef5eb470630
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::terrain_stroke::TerrainStroke::new
// 信息 ID: ff09113e111e9a6d39f67b79
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::terrain_editing::TerrainEditingData::new_stroke; module=558, inline_record=71572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::terrain_stroke::_::impl$0::deserialize
// 信息 ID: a94f081ff83b39d8387e840e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<serde_core::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState> >; module=1804, inline_record=43576; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: 05df91a55cb3bd40a669ec8e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::terrain_stroke::_::impl$0::deserialize::__Field> > @ 0x1ba94a3 -> 0x1ad3aa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: f5ef5a4ad9b22ede1ec5ec1d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::terrain_stroke::_::impl$0::deserialize::__Field> >; module=1803, inline_record=429148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 16108f41f8a009a94e7d3b89
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::terrain_stroke::_::impl$0::deserialize::__Visitor>; module=1766, inline_record=67392; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 5cf373f570a0a19c5f6251a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::terrain_stroke::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=257584; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: efca4eb0a4dff26154a877c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::terrain_stroke::_::impl$0::deserialize::__Visitor>; module=1803, inline_record=152068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::terrain_stroke::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: 1a69a9d6f72e7c65f5de63fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::terrain_stroke::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::ser::Serializer::collect_map<serde_json::value::ser::Serializer,ref$<country_core::resources::terrain_editing::TerrainStrokeId>,ref$<country_core::resources::terrain_stroke::TerrainStroke>,ref$<std::collections::hash::map::HashMap<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState,alloc::alloc::Global> > > @ 0x1b7cf3d -> 0x1ad2160 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::terrain_stroke::impl$2::clone
// 信息 ID: d78e3d4c5df716a584300f96
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::raw::impl$13::clone<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global>; module=614, inline_record=68172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
