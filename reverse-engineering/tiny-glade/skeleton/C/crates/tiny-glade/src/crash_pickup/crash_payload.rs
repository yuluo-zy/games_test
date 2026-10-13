// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/crash_pickup/crash_payload.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: bb478adc4f8c17b03b8f130d89bea2ff563a107ef5c9573ead60bd19744c8672
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: crash_payload.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 6, 30, 31, 91, 137, 166, 189, 214, 216, 235
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use backtrace; // namespace candidate
//   package identities: [{"name":"backtrace","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.3.71"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"},{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"},{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$0::default
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":235,"record_offset":32352,"rva":6275824,"va":5374984944}
// 编译实例 RVAs: 0x5fc2f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t693654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374984944,"raw_hex":"e896950a00","size":5,"soft_fail":false,"target_va":5375678688,"va":5374985029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=7
//     target_aliases=1699 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6262827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374984944,"raw_hex":"e8d56fa0ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374985014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007eec0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5756948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374984944,"raw_hex":"e8ec27a8ff","size":5,"soft_fail":false,"target_va":5369228992,"va":5374985935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140103f10
//     owner=unresolved_namespace; count=6
//     target_aliases=3 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5212164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374984944,"raw_hex":"e8fc77b0ff","size":5,"soft_fail":false,"target_va":5369773840,"va":5374985999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42071449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374984944,"raw_hex":"e899f58102","size":5,"soft_fail":false,"target_va":5417057056,"va":5374985602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t41635005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374984944,"raw_hex":"e8bd4c7b02","size":5,"soft_fail":false,"target_va":5416620794,"va":5374985784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4834257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374984944,"raw_hex":"e82f3cb6ff","size":5,"soft_fail":false,"target_va":5370151808,"va":5374986060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$48::from
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":235,"record_offset":32552,"parent_record_offset":32352,"ancestor_record_offset":32352,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$27::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":235,"record_offset":32496,"parent_record_offset":32352,"ancestor_record_offset":32352,"inlinee":7206,"source_line":4236,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":235,"record_offset":32868,"parent_record_offset":32844,"ancestor_record_offset":32352,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":235,"record_offset":32892,"parent_record_offset":32868,"ancestor_record_offset":32352,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":235,"record_offset":32916,"parent_record_offset":32892,"ancestor_record_offset":32352,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":235,"record_offset":32844,"parent_record_offset":32816,"ancestor_record_offset":32352,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":235,"record_offset":32756,"parent_record_offset":32728,"ancestor_record_offset":32352,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":235,"record_offset":32816,"parent_record_offset":32784,"ancestor_record_offset":32352,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":235,"record_offset":32784,"parent_record_offset":32756,"ancestor_record_offset":32352,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":235,"record_offset":32636,"parent_record_offset":32608,"ancestor_record_offset":32352,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":235,"record_offset":32664,"parent_record_offset":32636,"ancestor_record_offset":32352,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":235,"record_offset":32692,"parent_record_offset":32664,"ancestor_record_offset":32352,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":235,"record_offset":32608,"parent_record_offset":32580,"ancestor_record_offset":32352,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":235,"record_offset":32580,"parent_record_offset":32552,"ancestor_record_offset":32352,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":235,"record_offset":32520,"parent_record_offset":32496,"ancestor_record_offset":32352,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":235,"record_offset":32728,"parent_record_offset":32692,"ancestor_record_offset":32352,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":235,"record_offset":32972,"parent_record_offset":32692,"ancestor_record_offset":32352,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":235,"record_offset":32996,"parent_record_offset":32972,"ancestor_record_offset":32352,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=19
// 源码行: 6, 26, 39, 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E4C
// 首条位置: {"module":31,"inlinee":7756}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":109300,"parent_record_offset":109276,"ancestor_record_offset":109276,"inlinee":7758,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":31,"record_offset":109324,"parent_record_offset":109300,"ancestor_record_offset":109276,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":109348,"parent_record_offset":109324,"ancestor_record_offset":109276,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 6, 17, 39, 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E1A
// 首条位置: {"module":31,"inlinee":7706}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":31,"record_offset":88968,"parent_record_offset":88932,"ancestor_record_offset":88932,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":31,"record_offset":88996,"parent_record_offset":88968,"ancestor_record_offset":88932,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":89024,"parent_record_offset":88996,"ancestor_record_offset":88932,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":89056,"parent_record_offset":89024,"ancestor_record_offset":88932,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 6, 17, 26, 39, 45, 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E18
// 首条位置: {"module":31,"inlinee":7704}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":86888,"parent_record_offset":86868,"ancestor_record_offset":86868,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::impl$7::deserialize_identifier
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":117980,"parent_record_offset":117948,"ancestor_record_offset":117948,"inlinee":7716,"source_line":1903,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":31,"record_offset":87004,"parent_record_offset":86912,"ancestor_record_offset":86868,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":31,"record_offset":88996,"parent_record_offset":88968,"ancestor_record_offset":88744,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":89024,"parent_record_offset":88996,"ancestor_record_offset":88744,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":89056,"parent_record_offset":89024,"ancestor_record_offset":88744,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":31,"record_offset":88968,"parent_record_offset":88932,"ancestor_record_offset":88744,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":86948,"parent_record_offset":86912,"ancestor_record_offset":86868,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::fix_position
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":118332,"parent_record_offset":118012,"ancestor_record_offset":117948,"inlinee":7591,"source_line":441,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":118080,"parent_record_offset":118012,"ancestor_record_offset":117948,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":118120,"parent_record_offset":118080,"ancestor_record_offset":117948,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":86912,"parent_record_offset":86888,"ancestor_record_offset":86868,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$7::deserialize_str
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":118012,"parent_record_offset":117980,"ancestor_record_offset":117948,"inlinee":7717,"source_line":1524,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":31,"record_offset":86972,"parent_record_offset":86948,"ancestor_record_offset":86868,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":31,"record_offset":118148,"parent_record_offset":118120,"ancestor_record_offset":117948,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":31,"record_offset":88932,"parent_record_offset":88800,"ancestor_record_offset":88744,"inlinee":7706,"source_line":6,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 6, 26, 39, 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x40EB
// 首条位置: {"module":214,"inlinee":16619}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 6, 26, 39, 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E87
// 首条位置: {"module":31,"inlinee":7815}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":31,"record_offset":177124,"parent_record_offset":176336,"ancestor_record_offset":176336,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":177608,"parent_record_offset":176336,"ancestor_record_offset":176336,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":31,"record_offset":177576,"parent_record_offset":176336,"ancestor_record_offset":176336,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":31,"record_offset":178616,"parent_record_offset":176336,"ancestor_record_offset":176336,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":180200,"parent_record_offset":176336,"ancestor_record_offset":176336,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":176412,"parent_record_offset":176336,"ancestor_record_offset":176336,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":177152,"parent_record_offset":176336,"ancestor_record_offset":176336,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":179284,"parent_record_offset":179260,"ancestor_record_offset":176336,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":179308,"parent_record_offset":179284,"ancestor_record_offset":176336,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":179332,"parent_record_offset":179308,"ancestor_record_offset":176336,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":179260,"parent_record_offset":179204,"ancestor_record_offset":176336,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":31,"record_offset":179232,"parent_record_offset":179204,"ancestor_record_offset":176336,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":31,"record_offset":179204,"parent_record_offset":179180,"ancestor_record_offset":176336,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":31,"record_offset":179180,"parent_record_offset":179156,"ancestor_record_offset":176336,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":31,"record_offset":176692,"parent_record_offset":176584,"ancestor_record_offset":176336,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":31,"record_offset":250248,"parent_record_offset":250216,"ancestor_record_offset":245356,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] backtrace::capture::_::impl$0::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"backtrace","version":"0.3.71","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.71/src/capture.rs
//     evidence={"module":31,"record_offset":264888,"parent_record_offset":264864,"ancestor_record_offset":263816,"inlinee":7672,"source_line":25,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":31,"record_offset":176780,"parent_record_offset":176756,"ancestor_record_offset":176336,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":31,"record_offset":250416,"parent_record_offset":250388,"ancestor_record_offset":245356,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=137
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":177632,"parent_record_offset":177608,"ancestor_record_offset":176336,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":176804,"parent_record_offset":176780,"ancestor_record_offset":176336,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":176832,"parent_record_offset":176804,"ancestor_record_offset":176336,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":31,"record_offset":176756,"parent_record_offset":176720,"ancestor_record_offset":176336,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":31,"record_offset":177536,"parent_record_offset":177512,"ancestor_record_offset":176336,"inlinee":7798,"source_line":122,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":31,"record_offset":178688,"parent_record_offset":178664,"ancestor_record_offset":176336,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":251844,"parent_record_offset":251816,"ancestor_record_offset":245356,"inlinee":7799,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":251016,"parent_record_offset":250992,"ancestor_record_offset":245356,"inlinee":7805,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_string
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":178804,"parent_record_offset":178780,"ancestor_record_offset":176336,"inlinee":7748,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":178664,"parent_record_offset":178640,"ancestor_record_offset":176336,"inlinee":7758,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":178712,"parent_record_offset":178688,"ancestor_record_offset":176336,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":176500,"parent_record_offset":176448,"ancestor_record_offset":176336,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":31,"record_offset":246200,"parent_record_offset":246176,"ancestor_record_offset":245356,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$29::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":31,"record_offset":178068,"parent_record_offset":178044,"ancestor_record_offset":176336,"inlinee":7146,"source_line":1870,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$43::deserialize
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":31,"record_offset":251816,"parent_record_offset":251792,"ancestor_record_offset":245356,"inlinee":7862,"source_line":2987,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":31,"record_offset":265392,"parent_record_offset":265368,"ancestor_record_offset":263816,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=36
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":176636,"parent_record_offset":176584,"ancestor_record_offset":176336,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=git_dependency_definition; count=16
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":177236,"parent_record_offset":177192,"ancestor_record_offset":176336,"inlinee":7597,"source_line":1061,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=16
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":177280,"parent_record_offset":177236,"ancestor_record_offset":176336,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=16
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":177320,"parent_record_offset":177280,"ancestor_record_offset":176336,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":176448,"parent_record_offset":176412,"ancestor_record_offset":176336,"inlinee":7724,"source_line":1986,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=git_dependency_definition; count=16
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":177192,"parent_record_offset":177152,"ancestor_record_offset":176336,"inlinee":7596,"source_line":2027,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":176584,"parent_record_offset":176556,"ancestor_record_offset":176336,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":176556,"parent_record_offset":176528,"ancestor_record_offset":176336,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=36
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":31,"record_offset":176660,"parent_record_offset":176636,"ancestor_record_offset":176336,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=16
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":31,"record_offset":177348,"parent_record_offset":177320,"ancestor_record_offset":176336,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":31,"record_offset":176720,"parent_record_offset":176584,"ancestor_record_offset":176336,"inlinee":7706,"source_line":6,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":31,"record_offset":176528,"parent_record_offset":176500,"ancestor_record_offset":176336,"inlinee":7704,"source_line":6,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 6, 26, 39, 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E88
// 首条位置: {"module":31,"inlinee":7816}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":181884,"parent_record_offset":181472,"ancestor_record_offset":181472,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":31,"record_offset":261044,"parent_record_offset":258196,"ancestor_record_offset":258196,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":181532,"parent_record_offset":181472,"ancestor_record_offset":181472,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":182168,"parent_record_offset":182144,"ancestor_record_offset":181472,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":182192,"parent_record_offset":182168,"ancestor_record_offset":181472,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":182216,"parent_record_offset":182192,"ancestor_record_offset":181472,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":182144,"parent_record_offset":182080,"ancestor_record_offset":181472,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":31,"record_offset":182112,"parent_record_offset":182080,"ancestor_record_offset":181472,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":31,"record_offset":182080,"parent_record_offset":182052,"ancestor_record_offset":181472,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":31,"record_offset":182052,"parent_record_offset":182024,"ancestor_record_offset":181472,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":31,"record_offset":260192,"parent_record_offset":260164,"ancestor_record_offset":258196,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] backtrace::capture::_::impl$0::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"backtrace","version":"0.3.71","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.71/src/capture.rs
//     evidence={"module":31,"record_offset":266576,"parent_record_offset":266552,"ancestor_record_offset":266420,"inlinee":7672,"source_line":25,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":31,"record_offset":260360,"parent_record_offset":260332,"ancestor_record_offset":258196,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":181912,"parent_record_offset":181884,"ancestor_record_offset":181472,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":31,"record_offset":181660,"parent_record_offset":181636,"ancestor_record_offset":181472,"inlinee":7798,"source_line":122,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":181636,"parent_record_offset":181564,"ancestor_record_offset":181472,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":31,"record_offset":258412,"parent_record_offset":258388,"ancestor_record_offset":258196,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$29::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":31,"record_offset":181844,"parent_record_offset":181820,"ancestor_record_offset":181472,"inlinee":7146,"source_line":1870,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":31,"record_offset":266736,"parent_record_offset":266712,"ancestor_record_offset":266420,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=git_dependency_definition; count=13
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":181564,"parent_record_offset":181532,"ancestor_record_offset":181472,"inlinee":7609,"source_line":1933,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=6
// 源码行: 6, 17, 26, 39, 45, 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":214,"record_offset":248,"rva":5906480,"va":5374615600}
// 编译实例 RVAs: 0x5a2030, 0x5a2150, 0x5a23a0, 0x5a24c0, 0x5a25e0, 0x5a2a20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f9710
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2788432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374615888,"raw_hex":"e8b073d5ff","size":5,"soft_fail":false,"target_va":5371827984,"va":5374616411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14045ad90
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1341085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374616480,"raw_hex":"e86389ebff","size":5,"soft_fail":false,"target_va":5373275536,"va":5374616616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14045e170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1329827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374618144,"raw_hex":"e85db5ebff","size":5,"soft_fail":false,"target_va":5373288816,"va":5374618638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14045f200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1325767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374618144,"raw_hex":"e839c5ebff","size":5,"soft_fail":false,"target_va":5373293056,"va":5374618818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14045fce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1319901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374615600,"raw_hex":"e823dcebff","size":5,"soft_fail":false,"target_va":5373295840,"va":5374615736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140460a00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1317359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374616480,"raw_hex":"e811e6ebff","size":5,"soft_fail":false,"target_va":5373299200,"va":5374616554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404625f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1309327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374615600,"raw_hex":"e87105ecff","size":5,"soft_fail":false,"target_va":5373306352,"va":5374615674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140464620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1302255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374616768,"raw_hex":"e81121ecff","size":5,"soft_fail":false,"target_va":5373314592,"va":5374616842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140465080
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1301165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374618144,"raw_hex":"e85325ecff","size":5,"soft_fail":false,"target_va":5373317248,"va":5374618408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404658b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1299529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374618144,"raw_hex":"e8b72becff","size":5,"soft_fail":false,"target_va":5373319344,"va":5374618868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140465aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1298523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374618144,"raw_hex":"e8a52fecff","size":5,"soft_fail":false,"target_va":5373319840,"va":5374618358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140465ca0
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1295674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374615888,"raw_hex":"e8c63aecff","size":5,"soft_fail":false,"target_va":5373320352,"va":5374616021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404671e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1290175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374615888,"raw_hex":"e84150ecff","size":5,"soft_fail":false,"target_va":5373325792,"va":5374615962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140469160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1282248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374615888,"raw_hex":"e8386fecff","size":5,"soft_fail":false,"target_va":5373333856,"va":5374616099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14046b660
//     owner=unresolved_namespace; count=8
//     target_aliases=3 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1273774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374617056,"raw_hex":"e85290ecff","size":5,"soft_fail":false,"target_va":5373343328,"va":5374617097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405a2150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374618144,"raw_hex":"e8bdf2ffff","size":5,"soft_fail":false,"target_va":5374615888,"va":5374619278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=25
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41737207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374615600,"raw_hex":"e8f7db7c02","size":5,"soft_fail":false,"target_va":5416353040,"va":5374615828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42002843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374618144,"raw_hex":"e89be98002","size":5,"soft_fail":false,"target_va":5416622208,"va":5374619360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::error::Error::io
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t41940869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374618144,"raw_hex":"e885f77f02","size":5,"soft_fail":false,"target_va":5416560208,"va":5374619334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142da1cb0
//     owner=unresolved_namespace; count=4
//     target_aliases=9 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t41942033","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374615600,"raw_hex":"e911fc7f02","size":5,"soft_fail":false,"target_va":5416557744,"va":5374615706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] serde_json::error::Error::io
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tjmp\t41942913","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374617056,"raw_hex":"e981ff7f02","size":5,"soft_fail":false,"target_va":5416560208,"va":5374617290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$10::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":16432,"parent_record_offset":9328,"ancestor_record_offset":9328,"inlinee":7412,"source_line":752,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$10::serialize_field
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":11204,"parent_record_offset":9328,"ancestor_record_offset":9328,"inlinee":7411,"source_line":738,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_newtype_variant
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":5856,"parent_record_offset":5492,"ancestor_record_offset":5492,"inlinee":7399,"source_line":233,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":532,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct_variant
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":9616,"parent_record_offset":9328,"ancestor_record_offset":9328,"inlinee":7409,"source_line":383,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_unit_variant
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":5772,"parent_record_offset":5492,"ancestor_record_offset":5492,"inlinee":7408,"source_line":214,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":1140,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4577,"source_line":718,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=17
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":1076,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":214,"record_offset":928,"parent_record_offset":896,"ancestor_record_offset":248,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":214,"record_offset":796,"parent_record_offset":768,"ancestor_record_offset":248,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":214,"record_offset":856,"parent_record_offset":824,"ancestor_record_offset":248,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":214,"record_offset":976,"parent_record_offset":952,"ancestor_record_offset":248,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":214,"record_offset":952,"parent_record_offset":928,"ancestor_record_offset":248,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":214,"record_offset":824,"parent_record_offset":796,"ancestor_record_offset":248,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":214,"record_offset":728,"parent_record_offset":700,"ancestor_record_offset":248,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":214,"record_offset":896,"parent_record_offset":728,"ancestor_record_offset":248,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":214,"record_offset":672,"parent_record_offset":644,"ancestor_record_offset":248,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":214,"record_offset":1016,"parent_record_offset":728,"ancestor_record_offset":248,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":214,"record_offset":768,"parent_record_offset":728,"ancestor_record_offset":248,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":214,"record_offset":700,"parent_record_offset":672,"ancestor_record_offset":248,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":214,"record_offset":17012,"parent_record_offset":16988,"ancestor_record_offset":9328,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":214,"record_offset":5820,"parent_record_offset":5796,"ancestor_record_offset":5492,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$1::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":214,"record_offset":6928,"parent_record_offset":5856,"ancestor_record_offset":5492,"inlinee":12739,"source_line":54,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=git_dependency_definition; count=14
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":588,"parent_record_offset":560,"ancestor_record_offset":248,"inlinee":4570,"source_line":1865,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object_value
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":6448,"parent_record_offset":5856,"ancestor_record_offset":5492,"inlinee":7401,"source_line":1910,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":1200,"parent_record_offset":1172,"ancestor_record_offset":248,"inlinee":4580,"source_line":1875,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":560,"parent_record_offset":532,"ancestor_record_offset":248,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":5796,"parent_record_offset":5772,"ancestor_record_offset":5492,"inlinee":7400,"source_line":190,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":1172,"parent_record_offset":1140,"ancestor_record_offset":248,"inlinee":4579,"source_line":669,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":214,"record_offset":11228,"parent_record_offset":11204,"ancestor_record_offset":9328,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":214,"record_offset":644,"parent_record_offset":616,"ancestor_record_offset":248,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":214,"record_offset":616,"parent_record_offset":588,"ancestor_record_offset":248,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 17, 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x40DB
// 首条位置: {"module":214,"inlinee":16603}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":214,"record_offset":8180,"parent_record_offset":8064,"ancestor_record_offset":8064,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":214,"record_offset":8336,"parent_record_offset":8268,"ancestor_record_offset":8268,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":214,"record_offset":8092,"parent_record_offset":8064,"ancestor_record_offset":8064,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":214,"record_offset":8120,"parent_record_offset":8064,"ancestor_record_offset":8064,"inlinee":6481,"source_line":2180,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":214,"record_offset":8412,"parent_record_offset":8388,"ancestor_record_offset":8268,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":214,"record_offset":8436,"parent_record_offset":8412,"ancestor_record_offset":8268,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$14::newtype_variant_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":214,"record_offset":8388,"parent_record_offset":8364,"ancestor_record_offset":8268,"inlinee":7890,"source_line":2078,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$17::newtype_variant_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":214,"record_offset":8144,"parent_record_offset":8120,"ancestor_record_offset":8064,"inlinee":7895,"source_line":2130,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":214,"record_offset":19160,"parent_record_offset":19136,"ancestor_record_offset":19024,"inlinee":7756,"source_line":26,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 17, 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":214,"record_offset":7500,"rva":5908304,"va":5374617424}
// 编译实例 RVAs: 0x5a2750, 0x5a31a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde_core..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t41942500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374617424,"raw_hex":"e8e4fd7f02","size":5,"soft_fail":false,"target_va":5416560512,"va":5374618007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1060743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374617424,"raw_hex":"e8872f1000","size":5,"soft_fail":false,"target_va":5375678704,"va":5374617956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4463933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374620064,"raw_hex":"e8c3e2bbff","size":5,"soft_fail":false,"target_va":5370156336,"va":5374620264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4460559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374617424,"raw_hex":"e8f1efbbff","size":5,"soft_fail":false,"target_va":5370157072,"va":5374617626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401627a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4456623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374617424,"raw_hex":"e851ffbbff","size":5,"soft_fail":false,"target_va":5370161056,"va":5374617674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401628c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4459050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374620064,"raw_hex":"e8d6f5bbff","size":5,"soft_fail":false,"target_va":5370161344,"va":5374620389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140163180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4456787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374620064,"raw_hex":"e8adfebbff","size":5,"soft_fail":false,"target_va":5370163584,"va":5374620366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140164040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4452954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374620064,"raw_hex":"e8a60dbcff","size":5,"soft_fail":false,"target_va":5370167360,"va":5374620309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140164ee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4449313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374620064,"raw_hex":"e8df1bbcff","size":5,"soft_fail":false,"target_va":5370171104,"va":5374620412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140165d30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4445448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374620064,"raw_hex":"e8f82abcff","size":5,"soft_fail":false,"target_va":5370174768,"va":5374620211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140165e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4442519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374617424,"raw_hex":"e86936bcff","size":5,"soft_fail":false,"target_va":5370175040,"va":5374617554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401687e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4432028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374617424,"raw_hex":"e8645fbcff","size":5,"soft_fail":false,"target_va":5370185696,"va":5374617719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140176aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4376887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374620064,"raw_hex":"e8c936bdff","size":5,"soft_fail":false,"target_va":5370243744,"va":5374620626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68840
//     owner=unresolved_namespace; count=2
//     target_aliases=11 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41705594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374617424,"raw_hex":"e87a607c02","size":5,"soft_fail":false,"target_va":5416323136,"va":5374617537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68ef0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41706937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374617424,"raw_hex":"e8b9657c02","size":5,"soft_fail":false,"target_va":5416324848,"va":5374617906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42436596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374620064,"raw_hex":"e8f4878702","size":5,"soft_fail":false,"target_va":5417057056,"va":5374620455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374620064,"raw_hex":"e83c170000","size":5,"soft_fail":false,"target_va":5374626512,"va":5374620559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::de::impl$7::deserialize_enum
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":214,"record_offset":7720,"parent_record_offset":7500,"ancestor_record_offset":7500,"inlinee":14882,"source_line":1871,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":214,"record_offset":8868,"parent_record_offset":8844,"ancestor_record_offset":7500,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":214,"record_offset":8892,"parent_record_offset":8868,"ancestor_record_offset":7500,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":214,"record_offset":8920,"parent_record_offset":8892,"ancestor_record_offset":7500,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":214,"record_offset":8844,"parent_record_offset":8820,"ancestor_record_offset":7500,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":214,"record_offset":8820,"parent_record_offset":8796,"ancestor_record_offset":7500,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":214,"record_offset":8796,"parent_record_offset":8772,"ancestor_record_offset":7500,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":214,"record_offset":8700,"parent_record_offset":7720,"ancestor_record_offset":7500,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":214,"record_offset":8180,"parent_record_offset":8064,"ancestor_record_offset":7500,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":214,"record_offset":8336,"parent_record_offset":8268,"ancestor_record_offset":7500,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":214,"record_offset":8092,"parent_record_offset":8064,"ancestor_record_offset":7500,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":214,"record_offset":8120,"parent_record_offset":8064,"ancestor_record_offset":7500,"inlinee":6481,"source_line":2180,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":214,"record_offset":8412,"parent_record_offset":8388,"ancestor_record_offset":7500,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":214,"record_offset":8436,"parent_record_offset":8412,"ancestor_record_offset":7500,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":214,"record_offset":8004,"parent_record_offset":7884,"ancestor_record_offset":7500,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":214,"record_offset":7884,"parent_record_offset":7720,"ancestor_record_offset":7500,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":214,"record_offset":7936,"parent_record_offset":7884,"ancestor_record_offset":7500,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$14::newtype_variant_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":214,"record_offset":8388,"parent_record_offset":8364,"ancestor_record_offset":7500,"inlinee":7890,"source_line":2078,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$17::newtype_variant_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":214,"record_offset":8144,"parent_record_offset":8120,"ancestor_record_offset":7500,"inlinee":7895,"source_line":2130,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":214,"record_offset":8028,"parent_record_offset":8004,"ancestor_record_offset":7500,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":214,"record_offset":7964,"parent_record_offset":7936,"ancestor_record_offset":7500,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":214,"record_offset":19160,"parent_record_offset":19136,"ancestor_record_offset":18276,"inlinee":7756,"source_line":26,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":214,"record_offset":8064,"parent_record_offset":7720,"ancestor_record_offset":7500,"inlinee":16603,"source_line":68,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$5::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":235,"record_offset":32184,"rva":6275728,"va":5374984848}
// 编译实例 RVAs: 0x5fc290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9841238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374984848,"raw_hex":"e8562a9600","size":5,"soft_fail":false,"target_va":5384826176,"va":5374984933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t9839193","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374984848,"raw_hex":"e959229600","size":5,"soft_fail":false,"target_va":5384824080,"va":5374984882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":235,"record_offset":32320,"parent_record_offset":32184,"ancestor_record_offset":32184,"inlinee":4604,"source_line":2121,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$6::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D45
// 首条位置: {"module":189,"inlinee":15685}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=2
// 源码行: 26, 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":214,"record_offset":30444,"rva":5915872,"va":5374624992}
// 编译实例 RVAs: 0x5a44e0, 0x5a4650
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] serde_core::de::Error::unknown_variant
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t41728375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374625360,"raw_hex":"e877b97c02","size":5,"soft_fail":false,"target_va":5416353984,"va":5374625604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":214,"record_offset":30636,"parent_record_offset":30444,"ancestor_record_offset":30444,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":214,"record_offset":30664,"parent_record_offset":30636,"ancestor_record_offset":30444,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":214,"record_offset":30692,"parent_record_offset":30664,"ancestor_record_offset":30444,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":214,"record_offset":30720,"parent_record_offset":30692,"ancestor_record_offset":30444,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00214.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$7::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D43
// 首条位置: {"module":189,"inlinee":15683}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$5::clone
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":189,"record_offset":3944,"parent_record_offset":3772,"ancestor_record_offset":3772,"inlinee":15676,"source_line":1893,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":4508,"parent_record_offset":4484,"ancestor_record_offset":3772,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":4532,"parent_record_offset":4508,"ancestor_record_offset":3772,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":7696,"parent_record_offset":7672,"ancestor_record_offset":3772,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":7720,"parent_record_offset":7696,"ancestor_record_offset":3772,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":4556,"parent_record_offset":4532,"ancestor_record_offset":3772,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":7744,"parent_record_offset":7720,"ancestor_record_offset":3772,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":4484,"parent_record_offset":4332,"ancestor_record_offset":3772,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":7672,"parent_record_offset":7616,"ancestor_record_offset":3772,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":4268,"parent_record_offset":4240,"ancestor_record_offset":3772,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":7644,"parent_record_offset":7616,"ancestor_record_offset":3772,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":7616,"parent_record_offset":7592,"ancestor_record_offset":3772,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":4332,"parent_record_offset":4296,"ancestor_record_offset":3772,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":4296,"parent_record_offset":4268,"ancestor_record_offset":3772,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":7592,"parent_record_offset":7568,"ancestor_record_offset":3772,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":4372,"parent_record_offset":4332,"ancestor_record_offset":3772,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":189,"record_offset":6648,"parent_record_offset":6572,"ancestor_record_offset":3772,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":189,"record_offset":4208,"parent_record_offset":4144,"ancestor_record_offset":3772,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":189,"record_offset":4180,"parent_record_offset":4144,"ancestor_record_offset":3772,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":189,"record_offset":4240,"parent_record_offset":4208,"ancestor_record_offset":3772,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":189,"record_offset":4116,"parent_record_offset":4084,"ancestor_record_offset":3772,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":189,"record_offset":4144,"parent_record_offset":4116,"ancestor_record_offset":3772,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":189,"record_offset":4084,"parent_record_offset":4052,"ancestor_record_offset":3772,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":189,"record_offset":4396,"parent_record_offset":4372,"ancestor_record_offset":3772,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":189,"record_offset":4444,"parent_record_offset":4420,"ancestor_record_offset":3772,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":189,"record_offset":7172,"parent_record_offset":7128,"ancestor_record_offset":3772,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":189,"record_offset":7128,"parent_record_offset":6648,"ancestor_record_offset":3772,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":189,"record_offset":7264,"parent_record_offset":6648,"ancestor_record_offset":3772,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":189,"record_offset":4420,"parent_record_offset":4396,"ancestor_record_offset":3772,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":189,"record_offset":6276,"parent_record_offset":5500,"ancestor_record_offset":3772,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":189,"record_offset":4612,"parent_record_offset":4208,"ancestor_record_offset":3772,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":189,"record_offset":4636,"parent_record_offset":4612,"ancestor_record_offset":3772,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":189,"record_offset":7544,"parent_record_offset":6464,"ancestor_record_offset":3772,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":189,"record_offset":7224,"parent_record_offset":7200,"ancestor_record_offset":3772,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":189,"record_offset":7200,"parent_record_offset":7172,"ancestor_record_offset":3772,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":189,"record_offset":7372,"parent_record_offset":7336,"ancestor_record_offset":3772,"inlinee":8473,"source_line":24,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":189,"record_offset":7336,"parent_record_offset":7308,"ancestor_record_offset":3772,"inlinee":8471,"source_line":27,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":189,"record_offset":7416,"parent_record_offset":7336,"ancestor_record_offset":3772,"inlinee":8475,"source_line":11,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":189,"record_offset":7508,"parent_record_offset":7480,"ancestor_record_offset":3772,"inlinee":8479,"source_line":122,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$30::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":189,"record_offset":6464,"parent_record_offset":6412,"ancestor_record_offset":3772,"inlinee":8466,"source_line":176,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":189,"record_offset":6412,"parent_record_offset":6336,"ancestor_record_offset":3772,"inlinee":8464,"source_line":278,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::import::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":189,"record_offset":7480,"parent_record_offset":7452,"ancestor_record_offset":3772,"inlinee":8477,"source_line":17,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":189,"record_offset":4020,"parent_record_offset":3988,"ancestor_record_offset":3772,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":189,"record_offset":3988,"parent_record_offset":3944,"ancestor_record_offset":3772,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":189,"record_offset":4052,"parent_record_offset":4020,"ancestor_record_offset":3772,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$6::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":189,"record_offset":5464,"parent_record_offset":5392,"ancestor_record_offset":3772,"inlinee":15685,"source_line":17,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$8::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":189,"record_offset":5500,"parent_record_offset":5392,"ancestor_record_offset":3772,"inlinee":15688,"source_line":39,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$9::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":189,"record_offset":6336,"parent_record_offset":6276,"ancestor_record_offset":3772,"inlinee":15690,"source_line":45,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$8::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D48
// 首条位置: {"module":189,"inlinee":15688}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":189,"record_offset":6276,"parent_record_offset":5500,"ancestor_record_offset":5500,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":189,"record_offset":9900,"parent_record_offset":5500,"ancestor_record_offset":5500,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":189,"record_offset":5576,"parent_record_offset":5500,"ancestor_record_offset":5500,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":6088,"parent_record_offset":6064,"ancestor_record_offset":5500,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":6112,"parent_record_offset":6088,"ancestor_record_offset":5500,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":7696,"parent_record_offset":7672,"ancestor_record_offset":5500,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":7720,"parent_record_offset":7696,"ancestor_record_offset":5500,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":6136,"parent_record_offset":6112,"ancestor_record_offset":5500,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":7744,"parent_record_offset":7720,"ancestor_record_offset":5500,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":6064,"parent_record_offset":5908,"ancestor_record_offset":5500,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":7672,"parent_record_offset":7616,"ancestor_record_offset":5500,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":5844,"parent_record_offset":5816,"ancestor_record_offset":5500,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":7644,"parent_record_offset":7616,"ancestor_record_offset":5500,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":7616,"parent_record_offset":7592,"ancestor_record_offset":5500,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":5908,"parent_record_offset":5872,"ancestor_record_offset":5500,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":5872,"parent_record_offset":5844,"ancestor_record_offset":5500,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":7592,"parent_record_offset":7568,"ancestor_record_offset":5500,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":5952,"parent_record_offset":5908,"ancestor_record_offset":5500,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":189,"record_offset":6648,"parent_record_offset":6572,"ancestor_record_offset":5500,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":189,"record_offset":5780,"parent_record_offset":5716,"ancestor_record_offset":5500,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":189,"record_offset":5752,"parent_record_offset":5716,"ancestor_record_offset":5500,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":189,"record_offset":5816,"parent_record_offset":5780,"ancestor_record_offset":5500,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":189,"record_offset":5688,"parent_record_offset":5660,"ancestor_record_offset":5500,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":189,"record_offset":5716,"parent_record_offset":5688,"ancestor_record_offset":5500,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":189,"record_offset":5660,"parent_record_offset":5632,"ancestor_record_offset":5500,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":189,"record_offset":5976,"parent_record_offset":5952,"ancestor_record_offset":5500,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":189,"record_offset":6024,"parent_record_offset":6000,"ancestor_record_offset":5500,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":189,"record_offset":7172,"parent_record_offset":7128,"ancestor_record_offset":5500,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":189,"record_offset":7128,"parent_record_offset":6648,"ancestor_record_offset":5500,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":189,"record_offset":7264,"parent_record_offset":6648,"ancestor_record_offset":5500,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":189,"record_offset":6000,"parent_record_offset":5976,"ancestor_record_offset":5500,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":189,"record_offset":7308,"parent_record_offset":6464,"ancestor_record_offset":5500,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":189,"record_offset":6192,"parent_record_offset":5780,"ancestor_record_offset":5500,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":189,"record_offset":6216,"parent_record_offset":6192,"ancestor_record_offset":5500,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":189,"record_offset":7544,"parent_record_offset":6464,"ancestor_record_offset":5500,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":189,"record_offset":7224,"parent_record_offset":7200,"ancestor_record_offset":5500,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":189,"record_offset":7200,"parent_record_offset":7172,"ancestor_record_offset":5500,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":189,"record_offset":7372,"parent_record_offset":7336,"ancestor_record_offset":5500,"inlinee":8473,"source_line":24,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":189,"record_offset":7336,"parent_record_offset":7308,"ancestor_record_offset":5500,"inlinee":8471,"source_line":27,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":189,"record_offset":7416,"parent_record_offset":7336,"ancestor_record_offset":5500,"inlinee":8475,"source_line":11,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":189,"record_offset":7508,"parent_record_offset":7480,"ancestor_record_offset":5500,"inlinee":8479,"source_line":122,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$30::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":189,"record_offset":6464,"parent_record_offset":6412,"ancestor_record_offset":5500,"inlinee":8466,"source_line":176,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":189,"record_offset":6412,"parent_record_offset":6336,"ancestor_record_offset":5500,"inlinee":8464,"source_line":278,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::import::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":189,"record_offset":7480,"parent_record_offset":7452,"ancestor_record_offset":5500,"inlinee":8477,"source_line":17,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":189,"record_offset":5604,"parent_record_offset":5576,"ancestor_record_offset":5500,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":189,"record_offset":8144,"parent_record_offset":6336,"ancestor_record_offset":5500,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":189,"record_offset":5632,"parent_record_offset":5604,"ancestor_record_offset":5500,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$9::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":189,"record_offset":6336,"parent_record_offset":6276,"ancestor_record_offset":5500,"inlinee":15690,"source_line":45,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$9::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D4A
// 首条位置: {"module":189,"inlinee":15690}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":189,"record_offset":9496,"parent_record_offset":6336,"ancestor_record_offset":6336,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":189,"record_offset":6412,"parent_record_offset":6336,"ancestor_record_offset":6336,"inlinee":8464,"source_line":278,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":189,"record_offset":8144,"parent_record_offset":6336,"ancestor_record_offset":6336,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":7024,"parent_record_offset":7000,"ancestor_record_offset":6336,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":7048,"parent_record_offset":7024,"ancestor_record_offset":6336,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":7696,"parent_record_offset":7672,"ancestor_record_offset":6336,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":7720,"parent_record_offset":7696,"ancestor_record_offset":6336,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":7072,"parent_record_offset":7048,"ancestor_record_offset":6336,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":7744,"parent_record_offset":7720,"ancestor_record_offset":6336,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":7000,"parent_record_offset":6836,"ancestor_record_offset":6336,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":7672,"parent_record_offset":7616,"ancestor_record_offset":6336,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":6752,"parent_record_offset":6712,"ancestor_record_offset":6336,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":7644,"parent_record_offset":7616,"ancestor_record_offset":6336,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":7616,"parent_record_offset":7592,"ancestor_record_offset":6336,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":6836,"parent_record_offset":6792,"ancestor_record_offset":6336,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":6792,"parent_record_offset":6752,"ancestor_record_offset":6336,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":7592,"parent_record_offset":7568,"ancestor_record_offset":6336,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":6888,"parent_record_offset":6836,"ancestor_record_offset":6336,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":189,"record_offset":6648,"parent_record_offset":6572,"ancestor_record_offset":6336,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":189,"record_offset":8348,"parent_record_offset":8284,"ancestor_record_offset":6336,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":189,"record_offset":6620,"parent_record_offset":6572,"ancestor_record_offset":6336,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":189,"record_offset":6712,"parent_record_offset":6648,"ancestor_record_offset":6336,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":189,"record_offset":6532,"parent_record_offset":6464,"ancestor_record_offset":6336,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":189,"record_offset":6572,"parent_record_offset":6532,"ancestor_record_offset":6336,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":189,"record_offset":8228,"parent_record_offset":8200,"ancestor_record_offset":6336,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":189,"record_offset":6912,"parent_record_offset":6888,"ancestor_record_offset":6336,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":189,"record_offset":6960,"parent_record_offset":6936,"ancestor_record_offset":6336,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":189,"record_offset":7172,"parent_record_offset":7128,"ancestor_record_offset":6336,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":189,"record_offset":7128,"parent_record_offset":6648,"ancestor_record_offset":6336,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":189,"record_offset":7264,"parent_record_offset":6648,"ancestor_record_offset":6336,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":189,"record_offset":6936,"parent_record_offset":6912,"ancestor_record_offset":6336,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":189,"record_offset":7308,"parent_record_offset":6464,"ancestor_record_offset":6336,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":189,"record_offset":8760,"parent_record_offset":8348,"ancestor_record_offset":6336,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":189,"record_offset":8784,"parent_record_offset":8760,"ancestor_record_offset":6336,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":189,"record_offset":7544,"parent_record_offset":6464,"ancestor_record_offset":6336,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":189,"record_offset":7224,"parent_record_offset":7200,"ancestor_record_offset":6336,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":189,"record_offset":7200,"parent_record_offset":7172,"ancestor_record_offset":6336,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":189,"record_offset":7372,"parent_record_offset":7336,"ancestor_record_offset":6336,"inlinee":8473,"source_line":24,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":189,"record_offset":7336,"parent_record_offset":7308,"ancestor_record_offset":6336,"inlinee":8471,"source_line":27,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":189,"record_offset":7416,"parent_record_offset":7336,"ancestor_record_offset":6336,"inlinee":8475,"source_line":11,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":189,"record_offset":7508,"parent_record_offset":7480,"ancestor_record_offset":6336,"inlinee":8479,"source_line":122,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$30::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":189,"record_offset":6464,"parent_record_offset":6412,"ancestor_record_offset":6336,"inlinee":8466,"source_line":176,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::import::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":189,"record_offset":7480,"parent_record_offset":7452,"ancestor_record_offset":6336,"inlinee":8477,"source_line":17,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":189,"record_offset":8172,"parent_record_offset":8144,"ancestor_record_offset":6336,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":189,"record_offset":8200,"parent_record_offset":8172,"ancestor_record_offset":6336,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x40EC
// 首条位置: {"module":214,"inlinee":16620}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E4A
// 首条位置: {"module":31,"inlinee":7754}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":31,"record_offset":107972,"parent_record_offset":107940,"ancestor_record_offset":107940,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":31,"record_offset":107996,"parent_record_offset":107972,"ancestor_record_offset":107940,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":108020,"parent_record_offset":107996,"ancestor_record_offset":107940,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":108048,"parent_record_offset":108020,"ancestor_record_offset":107940,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$10::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E40
// 首条位置: {"module":31,"inlinee":7744}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":102148,"parent_record_offset":102124,"ancestor_record_offset":102124,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":31,"record_offset":102276,"parent_record_offset":102172,"ancestor_record_offset":102124,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":31,"record_offset":102356,"parent_record_offset":102332,"ancestor_record_offset":102124,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":102380,"parent_record_offset":102356,"ancestor_record_offset":102124,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":102408,"parent_record_offset":102380,"ancestor_record_offset":102124,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":31,"record_offset":102332,"parent_record_offset":102304,"ancestor_record_offset":102124,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":102220,"parent_record_offset":102172,"ancestor_record_offset":102124,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":102172,"parent_record_offset":102148,"ancestor_record_offset":102124,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":31,"record_offset":102244,"parent_record_offset":102220,"ancestor_record_offset":102124,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":31,"record_offset":102304,"parent_record_offset":102172,"ancestor_record_offset":102124,"inlinee":7746,"source_line":68,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$11::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x40E9
// 首条位置: {"module":214,"inlinee":16617}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$11::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E3E
// 首条位置: {"module":31,"inlinee":7742}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":31,"record_offset":102588,"parent_record_offset":101956,"ancestor_record_offset":101956,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":103652,"parent_record_offset":101956,"ancestor_record_offset":101956,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":31,"record_offset":103004,"parent_record_offset":101956,"ancestor_record_offset":101956,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":31,"record_offset":103512,"parent_record_offset":101956,"ancestor_record_offset":101956,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":104252,"parent_record_offset":101956,"ancestor_record_offset":101956,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":102020,"parent_record_offset":101956,"ancestor_record_offset":101956,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":102616,"parent_record_offset":101956,"ancestor_record_offset":101956,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":103796,"parent_record_offset":103772,"ancestor_record_offset":101956,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":103820,"parent_record_offset":103796,"ancestor_record_offset":101956,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":103844,"parent_record_offset":103820,"ancestor_record_offset":101956,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":103772,"parent_record_offset":103748,"ancestor_record_offset":101956,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":31,"record_offset":104092,"parent_record_offset":104064,"ancestor_record_offset":101956,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":31,"record_offset":103748,"parent_record_offset":103724,"ancestor_record_offset":101956,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":31,"record_offset":103724,"parent_record_offset":103700,"ancestor_record_offset":101956,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":31,"record_offset":102276,"parent_record_offset":102172,"ancestor_record_offset":101956,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":31,"record_offset":102356,"parent_record_offset":102332,"ancestor_record_offset":101956,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":103676,"parent_record_offset":103652,"ancestor_record_offset":101956,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":102380,"parent_record_offset":102356,"ancestor_record_offset":101956,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":102408,"parent_record_offset":102380,"ancestor_record_offset":101956,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":31,"record_offset":102332,"parent_record_offset":102304,"ancestor_record_offset":101956,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":31,"record_offset":103584,"parent_record_offset":103560,"ancestor_record_offset":101956,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":103560,"parent_record_offset":103536,"ancestor_record_offset":101956,"inlinee":7748,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":103608,"parent_record_offset":103584,"ancestor_record_offset":101956,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":102100,"parent_record_offset":102052,"ancestor_record_offset":101956,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":31,"record_offset":103440,"parent_record_offset":103416,"ancestor_record_offset":101956,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":102220,"parent_record_offset":102172,"ancestor_record_offset":101956,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":102692,"parent_record_offset":102652,"ancestor_record_offset":101956,"inlinee":7597,"source_line":1061,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":102736,"parent_record_offset":102692,"ancestor_record_offset":101956,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":102776,"parent_record_offset":102736,"ancestor_record_offset":101956,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":102052,"parent_record_offset":102020,"ancestor_record_offset":101956,"inlinee":7724,"source_line":1986,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":102652,"parent_record_offset":102616,"ancestor_record_offset":101956,"inlinee":7596,"source_line":2027,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":102172,"parent_record_offset":102148,"ancestor_record_offset":101956,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":102148,"parent_record_offset":102124,"ancestor_record_offset":101956,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":31,"record_offset":102244,"parent_record_offset":102220,"ancestor_record_offset":101956,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":31,"record_offset":102804,"parent_record_offset":102776,"ancestor_record_offset":101956,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$10::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":31,"record_offset":102124,"parent_record_offset":102100,"ancestor_record_offset":101956,"inlinee":7744,"source_line":68,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":31,"record_offset":102304,"parent_record_offset":102172,"ancestor_record_offset":101956,"inlinee":7746,"source_line":68,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$11::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E43
// 首条位置: {"module":31,"inlinee":7747}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":105264,"parent_record_offset":104936,"ancestor_record_offset":104936,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":104984,"parent_record_offset":104936,"ancestor_record_offset":104936,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":105440,"parent_record_offset":105416,"ancestor_record_offset":104936,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":105464,"parent_record_offset":105440,"ancestor_record_offset":104936,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":105488,"parent_record_offset":105464,"ancestor_record_offset":104936,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":105416,"parent_record_offset":105360,"ancestor_record_offset":104936,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":31,"record_offset":105388,"parent_record_offset":105360,"ancestor_record_offset":104936,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":31,"record_offset":105360,"parent_record_offset":105336,"ancestor_record_offset":104936,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":31,"record_offset":105336,"parent_record_offset":105312,"ancestor_record_offset":104936,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":105288,"parent_record_offset":105264,"ancestor_record_offset":104936,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":105080,"parent_record_offset":105012,"ancestor_record_offset":104936,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":31,"record_offset":105104,"parent_record_offset":105080,"ancestor_record_offset":104936,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":105012,"parent_record_offset":104984,"ancestor_record_offset":104936,"inlinee":7609,"source_line":1933,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E48
// 首条位置: {"module":31,"inlinee":7752}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":107780,"parent_record_offset":107752,"ancestor_record_offset":107752,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":31,"record_offset":107912,"parent_record_offset":107808,"ancestor_record_offset":107752,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":31,"record_offset":107996,"parent_record_offset":107972,"ancestor_record_offset":107752,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":108020,"parent_record_offset":107996,"ancestor_record_offset":107752,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":108048,"parent_record_offset":108020,"ancestor_record_offset":107752,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":31,"record_offset":107972,"parent_record_offset":107940,"ancestor_record_offset":107752,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":107856,"parent_record_offset":107808,"ancestor_record_offset":107752,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":107808,"parent_record_offset":107780,"ancestor_record_offset":107752,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":31,"record_offset":107880,"parent_record_offset":107856,"ancestor_record_offset":107752,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":31,"record_offset":107940,"parent_record_offset":107808,"ancestor_record_offset":107752,"inlinee":7754,"source_line":68,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x40EA
// 首条位置: {"module":214,"inlinee":16618}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E46
// 首条位置: {"module":31,"inlinee":7750}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":31,"record_offset":108228,"parent_record_offset":107572,"ancestor_record_offset":107572,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":109164,"parent_record_offset":107572,"ancestor_record_offset":107572,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":31,"record_offset":109132,"parent_record_offset":107572,"ancestor_record_offset":107572,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":31,"record_offset":109252,"parent_record_offset":107572,"ancestor_record_offset":107572,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":109532,"parent_record_offset":107572,"ancestor_record_offset":107572,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":107636,"parent_record_offset":107572,"ancestor_record_offset":107572,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":108256,"parent_record_offset":107572,"ancestor_record_offset":107572,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":31,"record_offset":107912,"parent_record_offset":107808,"ancestor_record_offset":107572,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":31,"record_offset":107996,"parent_record_offset":107972,"ancestor_record_offset":107572,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":108020,"parent_record_offset":107996,"ancestor_record_offset":107572,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":108048,"parent_record_offset":108020,"ancestor_record_offset":107572,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":31,"record_offset":107972,"parent_record_offset":107940,"ancestor_record_offset":107572,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":31,"record_offset":109324,"parent_record_offset":109300,"ancestor_record_offset":107572,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":109300,"parent_record_offset":109276,"ancestor_record_offset":107572,"inlinee":7758,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":109348,"parent_record_offset":109324,"ancestor_record_offset":107572,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":107724,"parent_record_offset":107672,"ancestor_record_offset":107572,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":107856,"parent_record_offset":107808,"ancestor_record_offset":107572,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":108340,"parent_record_offset":108296,"ancestor_record_offset":107572,"inlinee":7597,"source_line":1061,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":108384,"parent_record_offset":108340,"ancestor_record_offset":107572,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":108424,"parent_record_offset":108384,"ancestor_record_offset":107572,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":107672,"parent_record_offset":107636,"ancestor_record_offset":107572,"inlinee":7724,"source_line":1986,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":108296,"parent_record_offset":108256,"ancestor_record_offset":107572,"inlinee":7596,"source_line":2027,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":107808,"parent_record_offset":107780,"ancestor_record_offset":107572,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":107780,"parent_record_offset":107752,"ancestor_record_offset":107572,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":31,"record_offset":107880,"parent_record_offset":107856,"ancestor_record_offset":107572,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":31,"record_offset":108452,"parent_record_offset":108424,"ancestor_record_offset":107572,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":31,"record_offset":108640,"parent_record_offset":108616,"ancestor_record_offset":107572,"inlinee":7756,"source_line":39,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":31,"record_offset":107940,"parent_record_offset":107808,"ancestor_record_offset":107572,"inlinee":7754,"source_line":68,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":31,"record_offset":107752,"parent_record_offset":107724,"ancestor_record_offset":107572,"inlinee":7752,"source_line":68,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E4D
// 首条位置: {"module":31,"inlinee":7757}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":109692,"parent_record_offset":109648,"ancestor_record_offset":109648,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":109812,"parent_record_offset":109724,"ancestor_record_offset":109648,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":109724,"parent_record_offset":109692,"ancestor_record_offset":109648,"inlinee":7609,"source_line":1933,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":31,"record_offset":109836,"parent_record_offset":109812,"ancestor_record_offset":109648,"inlinee":7756,"source_line":39,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E33
// 首条位置: {"module":31,"inlinee":7731}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":31,"record_offset":98488,"parent_record_offset":98456,"ancestor_record_offset":98456,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":31,"record_offset":98512,"parent_record_offset":98488,"ancestor_record_offset":98456,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":98536,"parent_record_offset":98512,"ancestor_record_offset":98456,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":98560,"parent_record_offset":98536,"ancestor_record_offset":98456,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$4::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E31
// 首条位置: {"module":31,"inlinee":7729}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":98376,"parent_record_offset":98348,"ancestor_record_offset":98348,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":31,"record_offset":98776,"parent_record_offset":98404,"ancestor_record_offset":98348,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":31,"record_offset":98512,"parent_record_offset":98488,"ancestor_record_offset":98348,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":98536,"parent_record_offset":98512,"ancestor_record_offset":98348,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":98560,"parent_record_offset":98536,"ancestor_record_offset":98348,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":31,"record_offset":98488,"parent_record_offset":98456,"ancestor_record_offset":98348,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":98720,"parent_record_offset":98404,"ancestor_record_offset":98348,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":98404,"parent_record_offset":98376,"ancestor_record_offset":98348,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":31,"record_offset":98744,"parent_record_offset":98720,"ancestor_record_offset":98348,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":31,"record_offset":98456,"parent_record_offset":98404,"ancestor_record_offset":98348,"inlinee":7731,"source_line":68,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$5::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x40ED
// 首条位置: {"module":214,"inlinee":16621}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$5::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E29
// 首条位置: {"module":31,"inlinee":7721}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":31,"record_offset":98828,"parent_record_offset":98160,"ancestor_record_offset":98160,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":31,"record_offset":99632,"parent_record_offset":98160,"ancestor_record_offset":98160,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":31,"record_offset":99696,"parent_record_offset":98160,"ancestor_record_offset":98160,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":99836,"parent_record_offset":98160,"ancestor_record_offset":98160,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":98220,"parent_record_offset":98160,"ancestor_record_offset":98160,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":98856,"parent_record_offset":98160,"ancestor_record_offset":98160,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":31,"record_offset":98776,"parent_record_offset":98404,"ancestor_record_offset":98160,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":31,"record_offset":98512,"parent_record_offset":98488,"ancestor_record_offset":98160,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":98536,"parent_record_offset":98512,"ancestor_record_offset":98160,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":98560,"parent_record_offset":98536,"ancestor_record_offset":98160,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":31,"record_offset":98488,"parent_record_offset":98456,"ancestor_record_offset":98160,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":31,"record_offset":99768,"parent_record_offset":99744,"ancestor_record_offset":98160,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_bool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":99744,"parent_record_offset":99720,"ancestor_record_offset":98160,"inlinee":7736,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":99792,"parent_record_offset":99768,"ancestor_record_offset":98160,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":98320,"parent_record_offset":98256,"ancestor_record_offset":98160,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":31,"record_offset":99204,"parent_record_offset":99180,"ancestor_record_offset":98160,"inlinee":7727,"source_line":71,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":98720,"parent_record_offset":98404,"ancestor_record_offset":98160,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":98916,"parent_record_offset":98884,"ancestor_record_offset":98160,"inlinee":7597,"source_line":1061,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":98948,"parent_record_offset":98916,"ancestor_record_offset":98160,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":98988,"parent_record_offset":98948,"ancestor_record_offset":98160,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":98256,"parent_record_offset":98220,"ancestor_record_offset":98160,"inlinee":7724,"source_line":1986,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":98884,"parent_record_offset":98856,"ancestor_record_offset":98160,"inlinee":7596,"source_line":2027,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":98404,"parent_record_offset":98376,"ancestor_record_offset":98160,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":98376,"parent_record_offset":98348,"ancestor_record_offset":98160,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":31,"record_offset":98744,"parent_record_offset":98720,"ancestor_record_offset":98160,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":31,"record_offset":99016,"parent_record_offset":98988,"ancestor_record_offset":98160,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":31,"record_offset":98456,"parent_record_offset":98404,"ancestor_record_offset":98160,"inlinee":7731,"source_line":68,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$4::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":31,"record_offset":98348,"parent_record_offset":98320,"ancestor_record_offset":98160,"inlinee":7729,"source_line":68,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$5::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E2D
// 首条位置: {"module":31,"inlinee":7725}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":100220,"parent_record_offset":100184,"ancestor_record_offset":100184,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":100292,"parent_record_offset":100244,"ancestor_record_offset":100184,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":31,"record_offset":100316,"parent_record_offset":100292,"ancestor_record_offset":100184,"inlinee":7727,"source_line":71,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":100244,"parent_record_offset":100220,"ancestor_record_offset":100184,"inlinee":7609,"source_line":1933,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E54
// 首条位置: {"module":31,"inlinee":7764}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":31,"record_offset":112144,"parent_record_offset":112116,"ancestor_record_offset":112116,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":31,"record_offset":112168,"parent_record_offset":112144,"ancestor_record_offset":112116,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":112192,"parent_record_offset":112168,"ancestor_record_offset":112116,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":112220,"parent_record_offset":112192,"ancestor_record_offset":112116,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$7::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E52
// 首条位置: {"module":31,"inlinee":7762}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":111960,"parent_record_offset":111936,"ancestor_record_offset":111936,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":31,"record_offset":112088,"parent_record_offset":111984,"ancestor_record_offset":111936,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":31,"record_offset":112168,"parent_record_offset":112144,"ancestor_record_offset":111936,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":112192,"parent_record_offset":112168,"ancestor_record_offset":111936,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":112220,"parent_record_offset":112192,"ancestor_record_offset":111936,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":31,"record_offset":112144,"parent_record_offset":112116,"ancestor_record_offset":111936,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":112032,"parent_record_offset":111984,"ancestor_record_offset":111936,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":111984,"parent_record_offset":111960,"ancestor_record_offset":111936,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":31,"record_offset":112056,"parent_record_offset":112032,"ancestor_record_offset":111936,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":31,"record_offset":112116,"parent_record_offset":111984,"ancestor_record_offset":111936,"inlinee":7764,"source_line":68,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$8::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x40E8
// 首条位置: {"module":214,"inlinee":16616}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$8::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E50
// 首条位置: {"module":31,"inlinee":7760}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":31,"record_offset":112400,"parent_record_offset":111776,"ancestor_record_offset":111776,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":113368,"parent_record_offset":111776,"ancestor_record_offset":111776,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":31,"record_offset":113304,"parent_record_offset":111776,"ancestor_record_offset":111776,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":31,"record_offset":114180,"parent_record_offset":111776,"ancestor_record_offset":111776,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":114744,"parent_record_offset":111776,"ancestor_record_offset":111776,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":111832,"parent_record_offset":111776,"ancestor_record_offset":111776,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":112428,"parent_record_offset":111776,"ancestor_record_offset":111776,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":113568,"parent_record_offset":113544,"ancestor_record_offset":111776,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":113592,"parent_record_offset":113568,"ancestor_record_offset":111776,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":113616,"parent_record_offset":113592,"ancestor_record_offset":111776,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":113544,"parent_record_offset":113480,"ancestor_record_offset":111776,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":31,"record_offset":113512,"parent_record_offset":113480,"ancestor_record_offset":111776,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":31,"record_offset":113480,"parent_record_offset":113452,"ancestor_record_offset":111776,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":31,"record_offset":113452,"parent_record_offset":113424,"ancestor_record_offset":111776,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":31,"record_offset":112088,"parent_record_offset":111984,"ancestor_record_offset":111776,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":31,"record_offset":112168,"parent_record_offset":112144,"ancestor_record_offset":111776,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":113396,"parent_record_offset":113368,"ancestor_record_offset":111776,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":112192,"parent_record_offset":112168,"ancestor_record_offset":111776,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":112220,"parent_record_offset":112192,"ancestor_record_offset":111776,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":31,"record_offset":112144,"parent_record_offset":112116,"ancestor_record_offset":111776,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":31,"record_offset":114252,"parent_record_offset":114228,"ancestor_record_offset":111776,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":114228,"parent_record_offset":114204,"ancestor_record_offset":111776,"inlinee":7748,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":114276,"parent_record_offset":114252,"ancestor_record_offset":111776,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":111912,"parent_record_offset":111864,"ancestor_record_offset":111776,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":31,"record_offset":112812,"parent_record_offset":112788,"ancestor_record_offset":111776,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":112032,"parent_record_offset":111984,"ancestor_record_offset":111776,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":112512,"parent_record_offset":112468,"ancestor_record_offset":111776,"inlinee":7597,"source_line":1061,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":112556,"parent_record_offset":112512,"ancestor_record_offset":111776,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":112596,"parent_record_offset":112556,"ancestor_record_offset":111776,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":111864,"parent_record_offset":111832,"ancestor_record_offset":111776,"inlinee":7724,"source_line":1986,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":112468,"parent_record_offset":112428,"ancestor_record_offset":111776,"inlinee":7596,"source_line":2027,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":111984,"parent_record_offset":111960,"ancestor_record_offset":111776,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":31,"record_offset":111960,"parent_record_offset":111936,"ancestor_record_offset":111776,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":31,"record_offset":112056,"parent_record_offset":112032,"ancestor_record_offset":111776,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":31,"record_offset":112624,"parent_record_offset":112596,"ancestor_record_offset":111776,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":31,"record_offset":112116,"parent_record_offset":111984,"ancestor_record_offset":111776,"inlinee":7764,"source_line":68,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$7::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":31,"record_offset":111936,"parent_record_offset":111912,"ancestor_record_offset":111776,"inlinee":7762,"source_line":68,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$8::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E55
// 首条位置: {"module":31,"inlinee":7765}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":116032,"parent_record_offset":115680,"ancestor_record_offset":115680,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":115724,"parent_record_offset":115680,"ancestor_record_offset":115680,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":116208,"parent_record_offset":116184,"ancestor_record_offset":115680,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":116232,"parent_record_offset":116208,"ancestor_record_offset":115680,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":116256,"parent_record_offset":116232,"ancestor_record_offset":115680,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":116184,"parent_record_offset":116128,"ancestor_record_offset":115680,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":31,"record_offset":116156,"parent_record_offset":116128,"ancestor_record_offset":115680,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":31,"record_offset":116128,"parent_record_offset":116104,"ancestor_record_offset":115680,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":31,"record_offset":116104,"parent_record_offset":116080,"ancestor_record_offset":115680,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":116056,"parent_record_offset":116032,"ancestor_record_offset":115680,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":31,"record_offset":115820,"parent_record_offset":115752,"ancestor_record_offset":115680,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":31,"record_offset":115844,"parent_record_offset":115820,"ancestor_record_offset":115680,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":31,"record_offset":115752,"parent_record_offset":115724,"ancestor_record_offset":115680,"inlinee":7609,"source_line":1933,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E42
// 首条位置: {"module":31,"inlinee":7746}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":31,"record_offset":102332,"parent_record_offset":102304,"ancestor_record_offset":102304,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":31,"record_offset":102356,"parent_record_offset":102332,"ancestor_record_offset":102304,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":102380,"parent_record_offset":102356,"ancestor_record_offset":102304,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":31,"record_offset":102408,"parent_record_offset":102380,"ancestor_record_offset":102304,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::CrashPickupMessage::set_context
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x35B7
// 首条位置: {"module":137,"inlinee":13751}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":137,"record_offset":191492,"parent_record_offset":191444,"ancestor_record_offset":191444,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":137,"record_offset":191812,"parent_record_offset":191784,"ancestor_record_offset":191444,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":137,"record_offset":191840,"parent_record_offset":191812,"ancestor_record_offset":191444,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":137,"record_offset":191868,"parent_record_offset":191840,"ancestor_record_offset":191444,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":137,"record_offset":191784,"parent_record_offset":191752,"ancestor_record_offset":191444,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":137,"record_offset":191688,"parent_record_offset":191656,"ancestor_record_offset":191444,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":137,"record_offset":191752,"parent_record_offset":191720,"ancestor_record_offset":191444,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":137,"record_offset":191720,"parent_record_offset":191688,"ancestor_record_offset":191444,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":137,"record_offset":191556,"parent_record_offset":191524,"ancestor_record_offset":191444,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":137,"record_offset":191588,"parent_record_offset":191556,"ancestor_record_offset":191444,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":137,"record_offset":191620,"parent_record_offset":191588,"ancestor_record_offset":191444,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":137,"record_offset":191524,"parent_record_offset":191492,"ancestor_record_offset":191444,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":137,"record_offset":191656,"parent_record_offset":191620,"ancestor_record_offset":191444,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":137,"record_offset":191928,"parent_record_offset":191620,"ancestor_record_offset":191444,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":137,"record_offset":191956,"parent_record_offset":191928,"ancestor_record_offset":191444,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00137.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$10::default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D61
// 首条位置: {"module":30,"inlinee":7521}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$9::default
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":28328,"parent_record_offset":28292,"ancestor_record_offset":28292,"inlinee":7523,"source_line":2297,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$2::drop
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 119
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14A4
// 首条位置: {"module":6,"inlinee":5284}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":6,"record_offset":1366288,"parent_record_offset":1366232,"ancestor_record_offset":1366232,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00006.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":6,"record_offset":1366348,"parent_record_offset":1366232,"ancestor_record_offset":1366232,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00006.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":6,"record_offset":1367144,"parent_record_offset":1366232,"ancestor_record_offset":1366232,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00006.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":6,"record_offset":1366372,"parent_record_offset":1366232,"ancestor_record_offset":1366232,"inlinee":5286,"source_line":2698,"archive":"_pdb/C/modules/00006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":6,"record_offset":1366652,"parent_record_offset":1366628,"ancestor_record_offset":1366232,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":6,"record_offset":1366676,"parent_record_offset":1366652,"ancestor_record_offset":1366232,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":6,"record_offset":1366700,"parent_record_offset":1366676,"ancestor_record_offset":1366232,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":6,"record_offset":1366628,"parent_record_offset":1366572,"ancestor_record_offset":1366232,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":6,"record_offset":1366600,"parent_record_offset":1366572,"ancestor_record_offset":1366232,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":6,"record_offset":1366572,"parent_record_offset":1366548,"ancestor_record_offset":1366232,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":6,"record_offset":1366548,"parent_record_offset":1366524,"ancestor_record_offset":1366232,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00006.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":6,"record_offset":1366312,"parent_record_offset":1366288,"ancestor_record_offset":1366232,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00006.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":6,"record_offset":1366404,"parent_record_offset":1366372,"ancestor_record_offset":1366232,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00006.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":6,"record_offset":1367300,"parent_record_offset":1367276,"ancestor_record_offset":1366232,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00006.jsonl.gz"}
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
// 完整结构及原始类型记录: crash_payload.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::crash_payload::BacktracePayload
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::crash_pickup::crash_payload::BacktracePayload>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::crash_pickup::crash_payload::BacktracePayload"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tiny_glade::crash_pickup::crash_payload::BacktracePayload,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["tiny_glade::crash_pickup::crash_payload::BacktracePayload", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::BacktracePayload,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<tiny_glade::crash_pickup::crash_payload::BacktracePayload,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::BacktracePayload,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::BacktracePayload,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,tiny_glade::crash_pickup::crash_payload::BacktracePayload>", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "tiny_glade::crash_pickup::crash_payload::BacktracePayload"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::crash_payload::CrashMiscInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "tiny_glade::crash_pickup::crash_payload::CrashMiscInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo> >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "tiny_glade::crash_pickup::crash_payload::CrashMiscInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::crash_payload::CrashPayload
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tiny_glade::crash_pickup::crash_payload::CrashPayload>", "constructor": "enum2$", "arguments": ["tiny_glade::crash_pickup::crash_payload::CrashPayload"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPayload> >", "constructor": "core::option::Option", "arguments": ["enum2$<tiny_glade::crash_pickup::crash_payload::CrashPayload>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPayload> > >", "constructor": "enum2$", "arguments": ["core::option::Option<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPayload> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPayload> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPayload> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::crash_payload::CrashPickupMessage
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage>", "constructor": "enum2$", "arguments": ["tiny_glade::crash_pickup::crash_payload::CrashPickupMessage"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::from_slice<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage> >", "constructor": "serde_json::de::from_slice", "arguments": ["enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage>,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage>", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage>,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage>,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage>,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage>,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::crash_payload::CrashRecovered
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::crash_pickup::crash_payload::CrashRecovered>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::crash_pickup::crash_payload::CrashRecovered"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tiny_glade::crash_pickup::crash_payload::CrashRecovered,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["tiny_glade::crash_pickup::crash_payload::CrashRecovered", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::CrashRecovered,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<tiny_glade::crash_pickup::crash_payload::CrashRecovered,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::CrashRecovered,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::CrashRecovered,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,tiny_glade::crash_pickup::crash_payload::CrashRecovered>", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "tiny_glade::crash_pickup::crash_payload::CrashRecovered"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::crash_payload::RecoveredSave
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> >", "constructor": "core::result::Result", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::crash_pickup::crash_payload::RecoveredSave>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$43::deserialize<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >", "constructor": "serde_core::de::impls::impl$43::deserialize", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>", "ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::impls::impl$22::serialize<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >", "constructor": "serde_core::ser::impls::impl$22::serialize", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>", "ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::crash_payload::RecoveredSession
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::crash_pickup::crash_payload::RecoveredSession>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSession"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSession,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSession", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSession,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSession,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSession,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSession,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<tiny_glade::crash_pickup::crash_payload::RecoveredSession>", "constructor": "core::option::Option", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSession"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<tiny_glade::crash_pickup::crash_payload::RecoveredSession> >", "constructor": "enum2$", "arguments": ["core::option::Option<tiny_glade::crash_pickup::crash_payload::RecoveredSession>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<core::option::Option<tiny_glade::crash_pickup::crash_payload::RecoveredSession> > >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "enum2$<core::option::Option<tiny_glade::crash_pickup::crash_payload::RecoveredSession> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$16::deserialize<tiny_glade::crash_pickup::crash_payload::RecoveredSession,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >", "constructor": "serde_core::de::impls::impl$16::deserialize", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSession", "ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::crash_payload::ResolvedCrash
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::crash_pickup::crash_payload::ResolvedCrash>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::crash_pickup::crash_payload::ResolvedCrash"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<tiny_glade::crash_pickup::crash_payload::ResolvedCrash>", "constructor": "core::option::Option", "arguments": ["tiny_glade::crash_pickup::crash_payload::ResolvedCrash"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<tiny_glade::crash_pickup::crash_payload::ResolvedCrash> >", "constructor": "enum2$", "arguments": ["core::option::Option<tiny_glade::crash_pickup::crash_payload::ResolvedCrash>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<tiny_glade::crash_pickup::crash_payload::ResolvedCrash> > >", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "enum2$<core::option::Option<tiny_glade::crash_pickup::crash_payload::ResolvedCrash> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<tiny_glade::crash_pickup::crash_payload::ResolvedCrash> > >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<tiny_glade::crash_pickup::crash_payload::ResolvedCrash> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::crash_payload::SaveRecoveryError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>", "constructor": "enum2$", "arguments": ["tiny_glade::crash_pickup::crash_payload::SaveRecoveryError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> >", "constructor": "core::result::Result", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<std::path::PathBuf,enum2$<core::option::Option<country_core::resources::session::ActiveSession> > >,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> >", "constructor": "core::result::Result", "arguments": ["tuple$<std::path::PathBuf,enum2$<core::option::Option<country_core::resources::session::ActiveSession> > >", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<std::path::PathBuf,enum2$<core::option::Option<country_core::resources::session::ActiveSession> > >,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<std::path::PathBuf,enum2$<core::option::Option<country_core::resources::session::ActiveSession> > >,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<std::path::PathBuf,enum2$<core::option::Option<country_core::resources::session::ActiveSession> > >,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tuple$<std::path::PathBuf,enum2$<core::option::Option<country_core::resources::session::ActiveSession> > >,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$43::deserialize<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >", "constructor": "serde_core::de::impls::impl$43::deserialize", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>", "ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::impls::impl$22::serialize<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >", "constructor": "serde_core::ser::impls::impl$22::serialize", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>", "ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: crash_payload.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: crash_payload.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 68; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjWallHeightModeCache>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjWallHeightModeCache>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjWallHeightModeCache>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::RecalculateStoneFoundationSurfaceCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::DeleteStoneFoundationSurfaceCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>", "bevy_ecs::system::commands::Commands", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::RecalculateStoneFoundationSurfaceCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::DeleteStoneFoundationSurfaceCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::RecalculateStoneFoundationSurfaceCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::DeleteStoneFoundationSurfaceCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::FollowMouse>,enum2$<core::option::Option<ref$<country_core::components::HoverAboveTerrain> > >,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::FollowMouse>,enum2$<core::option::Option<ref$<country_core::components::HoverAboveTerrain> > >,ref_mut$<utils::components::transform::Transform> >,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::FollowMouse>,enum2$<core::option::Option<ref$<country_core::components::HoverAboveTerrain> > >,ref_mut$<utils::components::transform::Transform> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wooden_platform_stairs::RecalculateWoodenGateStepsCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::gates::Gates>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wooden_platform_stairs::RecalculateWoodenGateStepsCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::gates::Gates>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wooden_platform_stairs::RecalculateWoodenGateStepsCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::gates::Gates>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::brush_preview::BrushPreview>,enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > > >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::brush_preview::BrushPreview>,enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > > >,tuple$<> >", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::brush_preview::OverwriteBrushPreview>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>", "bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>", "tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::brush_preview::BrushPreview>,enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > > >,tuple$<> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::brush_preview::OverwriteBrushPreview>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::CrashPickupMessage::set_context
// 信息 ID: 7f7c0b10ede9635ff0a0c46b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=137, inline_record=191444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize
// 信息 ID: 77f0de4bdf306199f3475405
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=31, inline_record=108640; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=31, inline_record=109092; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=31, inline_record=109276; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=31, inline_record=109416; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=31, inline_record=109836; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=31, inline_record=110000; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=214, inline_record=19160; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impls::impl$16::deserialize<tiny_glade::crash_pickup::crash_payload::RecoveredSession,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=216, inline_record=4612; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: 04b4cd77208abaf0b4bc0303
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__FieldVisitor>; module=214, inline_record=34020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 098b24c87f164482bcdd8261
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$13::variant_seed<serde_json::read::SliceRead,core::marker::PhantomData<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Field> >; module=31, inline_record=88932; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$16::variant_seed<serde_json::read::SliceRead,core::marker::PhantomData<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Field> >; module=31, inline_record=119056; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=31, inline_record=176720; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=31, inline_record=224064; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=31, inline_record=264188; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: 85d729d09d87f8aa1e375d28
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor> @ 0x176d0f -> 0x5a44e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::impl$13::variant_seed<serde_json::read::SliceRead,core::marker::PhantomData<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Field> > @ 0x1615d7 -> 0x5a4650 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::impl$16::variant_seed<serde_json::read::SliceRead,core::marker::PhantomData<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Field> > @ 0x165def -> 0x5a4650 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 95192919f3ce439e1b050ee0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$13::variant_seed<serde_json::read::SliceRead,core::marker::PhantomData<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Field> >; module=31, inline_record=86868; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$13::variant_seed<serde_json::read::SliceRead,core::marker::PhantomData<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Field> >; module=31, inline_record=88744; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$16::variant_seed<serde_json::read::SliceRead,core::marker::PhantomData<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Field> >; module=31, inline_record=117948; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$16::variant_seed<serde_json::read::SliceRead,core::marker::PhantomData<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Field> >; module=31, inline_record=118664; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=31, inline_record=176528; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=31, inline_record=223884; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=31, inline_record=245592; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=31, inline_record=263996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: cfa1734731f71463112d205e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=214, inline_record=33800; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=214, inline_record=34236; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=214, inline_record=34452; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=214, inline_record=34904; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: fb77503ac090f411ddc7caf0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=214, inline_record=8064; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=214, inline_record=8268; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=214, inline_record=18816; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=214, inline_record=19024; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 信息 ID: e1d7b4a3daf783991db01f9e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=31, inline_record=107940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$10::deserialize
// 信息 ID: c390b78904d57759cc21526d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=31, inline_record=102124; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$11::expecting
// 信息 ID: 0f12c3b4a6ee4e81cdbc9d20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=214, inline_record=33172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$11::visit_map
// 信息 ID: deb606bebf0406ca394d1858
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=31, inline_record=101956; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$11::visit_seq
// 信息 ID: 1a5e3f14a780c91a8730e699
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=31, inline_record=104936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 信息 ID: be28a2aa1c80999900c9853a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=31, inline_record=107752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$2::expecting
// 信息 ID: 3e716ac71d997ea5c17e26e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=214, inline_record=33584; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 信息 ID: 3f1d3bc71214b357993aef4c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=31, inline_record=107572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_seq
// 信息 ID: d323eb7e36281cedd2310de0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=31, inline_record=109648; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str
// 信息 ID: e73e12ab65a21570473d3daf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=31, inline_record=98456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$4::deserialize
// 信息 ID: ac8fb579be14c662ed8779c9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=31, inline_record=98348; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$5::expecting
// 信息 ID: 6c2e69e352271b6e48f120dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=214, inline_record=34688; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$5::visit_map
// 信息 ID: 09b997df13b2c3e3fe7b66ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=31, inline_record=98160; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$5::visit_seq
// 信息 ID: d3a3ee03d0dc4e0751ed5d64
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=31, inline_record=100184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str
// 信息 ID: 3c36358374c3815ff332992a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=31, inline_record=112116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$7::deserialize
// 信息 ID: 41b1809e4fb704d0eccdddb2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=31, inline_record=111936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$8::expecting
// 信息 ID: 4799c8a2c53ced24bc26f75d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=214, inline_record=32936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$8::visit_map
// 信息 ID: ff70a197895a398ae09ea986
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=31, inline_record=111776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$8::visit_seq
// 信息 ID: 02b1f42c8f81062d106826c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=31, inline_record=115680; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str
// 信息 ID: 4cef2f8d3f24367b4523ddbb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=31, inline_record=102304; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: a7fff2b5cc942941d1f65feb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=31, inline_record=176336; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=31, inline_record=223720; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=31, inline_record=245356; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=31, inline_record=263816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: c23c9d4e1b27bc53a920529e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=31, inline_record=181472; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=31, inline_record=226312; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=31, inline_record=258196; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=31, inline_record=266420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >
// 信息 ID: af67700004dc012cb06840c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::Deserializer -> ["serde_json::read::SliceRead"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::de::Deserializer<serde_json::read::SliceRead>"]; 字段布局 = unknown。
// [类型关系] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize -> ["ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::de::impls::impl$43::deserialize<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0x5f9a45 -> 0x5a2750 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::from_slice<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage> > @ 0x15c827 -> 0x5a31a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: 68701bb9ac3fd44f60f3756d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] tiny_glade::crash_pickup::crash_payload::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::ser::impls::impl$22::serialize<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x5fa22e -> 0x5a2030 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,tiny_glade::crash_pickup::crash_payload::CrashRecovered> @ 0x465102 -> 0x5a2150 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,tiny_glade::crash_pickup::crash_payload::BacktracePayload> @ 0x465b22 -> 0x5a23a0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<core::option::Option<tiny_glade::crash_pickup::crash_payload::RecoveredSession> > > @ 0x45fd8c -> 0x5a24c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::impls::impl$22::serialize<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x5fa1c0 -> 0x5a25e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::crash_pickup::ClientHandle::send_message @ 0x5f7902 -> 0x5a2a20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$0::default
// 信息 ID: ebb9eb83be2d792976621312
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::crash_pickup::server::impl$1::on_message @ 0x158d09 -> 0x5fc2f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::crash_pickup::server::impl$1::on_minidump_created @ 0x15a588 -> 0x5fc2f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$10::default
// 信息 ID: 108a243a9ec63e909d456617
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::impl$1::on_message; module=30, inline_record=28292; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::impl$1::on_message; module=30, inline_record=32208; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::impl$1::on_minidump_created; module=30, inline_record=39316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$2::drop
// 信息 ID: 6abe6695b4624c5882567dd7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ptr::drop_in_place<tiny_glade::crash_pickup::crash_payload::ResolvedCrash>; module=6, inline_record=1366232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$5::fmt
// 信息 ID: a690c24734792dd7d046cb92
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$6::clone
// 信息 ID: 77089fba47f062980a3ee5f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=189, inline_record=5464; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$7::clone
// 信息 ID: fbda624010f223a2e7b69e8b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=189, inline_record=3772; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$8::clone
// 信息 ID: 11b7d17c58d809de0ec5a235
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=189, inline_record=5500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$9::clone
// 信息 ID: f7d0d0dfe7b7927104139584
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=189, inline_record=6336; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
