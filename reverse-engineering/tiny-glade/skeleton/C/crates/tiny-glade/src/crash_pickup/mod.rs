// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/crash_pickup/mod.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 3e7def16ceb706ae21d122b99e566b87a6621fdf38931849745c4408f564a1b5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 30, 186, 234, 242
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use minidumper; // namespace candidate
//   package identities: [{"name":"minidumper","source":"git+https://github.com/PounceLight/crash-handling.git?rev=28eb7b10d7e36fbd7a3cdf713562b36e538a2a50#28eb7b10d7e36fbd7a3cdf713562b36e538a2a50","version":"0.8.2"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use uuid; // namespace candidate
//   package identities: [{"name":"uuid","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.15.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::impl$4::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":234,"record_offset":25768,"rva":6264432,"va":5374973552}
// 编译实例 RVAs: 0x5f9670
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9852509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374973552,"raw_hex":"e85d569600","size":5,"soft_fail":false,"target_va":5384826176,"va":5374973662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] <std::io::error::Error as core::fmt::Display>::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t9792621","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374973552,"raw_hex":"e96d6c9500","size":5,"soft_fail":false,"target_va":5384766224,"va":5374973598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] _$LT$crash_handler..error..Error$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t2035178","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374973552,"raw_hex":"e9ea0d1f00","size":5,"soft_fail":false,"target_va":5377008864,"va":5374973681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] _$LT$minidumper..errors..Error$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t2031588","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374973552,"raw_hex":"e9e4ff1e00","size":5,"soft_fail":false,"target_va":5377005200,"va":5374973607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":234,"record_offset":25892,"parent_record_offset":25768,"ancestor_record_offset":25768,"inlinee":4604,"source_line":2121,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::impl$6::from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D07
// 首条位置: {"module":186,"inlinee":15623}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::impl$8::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":242,"record_offset":53176,"rva":6581488,"va":5375290608}
// 编译实例 RVAs: 0x646cf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_tuple_field1_finish
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t9526968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375290608,"raw_hex":"e8b85e9100","size":5,"soft_fail":false,"target_va":5384817664,"va":5375290691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::impl$9::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4502
// 首条位置: {"module":242,"inlinee":17666}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":242,"record_offset":22468,"parent_record_offset":22440,"ancestor_record_offset":22440,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":242,"record_offset":22496,"parent_record_offset":22468,"ancestor_record_offset":22440,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":242,"record_offset":22520,"parent_record_offset":22496,"ancestor_record_offset":22440,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::ClientHandle::send_raw_message
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D86
// 首条位置: {"module":234,"inlinee":7558}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":234,"record_offset":3636,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":234,"record_offset":4212,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] minidumper::ipc::client::Client::send_message
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=28eb7b10d7e36fbd7a3cdf713562b36e538a2a50#28eb7b10d7e36fbd7a3cdf713562b36e538a2a50"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/28eb7b1/minidumper/src/ipc/client.rs
//     evidence={"module":234,"record_offset":3664,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":7475,"source_line":166,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":3848,"parent_record_offset":3824,"ancestor_record_offset":3608,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":3872,"parent_record_offset":3848,"ancestor_record_offset":3608,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":3896,"parent_record_offset":3872,"ancestor_record_offset":3608,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":3824,"parent_record_offset":3768,"ancestor_record_offset":3608,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":234,"record_offset":3796,"parent_record_offset":3768,"ancestor_record_offset":3608,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":234,"record_offset":3768,"parent_record_offset":3744,"ancestor_record_offset":3608,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":234,"record_offset":3744,"parent_record_offset":3720,"ancestor_record_offset":3608,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":234,"record_offset":3696,"parent_record_offset":3664,"ancestor_record_offset":3608,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::ClientHandle::send_message
// 记录形式: compiled_procedure; instances=1
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":234,"record_offset":2488,"rva":6256800,"va":5374965920}
// 编译实例 RVAs: 0x5f78a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t712718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374965920,"raw_hex":"e80ee00a00","size":5,"soft_fail":false,"target_va":5375678688,"va":5374965965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t712590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374965920,"raw_hex":"e88edf0a00","size":5,"soft_fail":false,"target_va":5375678704,"va":5374966109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6243763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374965920,"raw_hex":"e84dbaa0ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374965950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ae10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5622970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374965920,"raw_hex":"e84633aaff","size":5,"soft_fail":false,"target_va":5369343504,"va":5374966469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405a2a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-347879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374965920,"raw_hex":"e819b1faff","size":5,"soft_fail":false,"target_va":5374618144,"va":5374966018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f8ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374965920,"raw_hex":"e80f130000","size":5,"soft_fail":false,"target_va":5374971040,"va":5374966156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41654606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374965920,"raw_hex":"e84e997b02","size":5,"soft_fail":false,"target_va":5416620794,"va":5374966183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41655370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374965920,"raw_hex":"e84a9c7b02","size":5,"soft_fail":false,"target_va":5416621632,"va":5374966257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidumper::ipc::client::Client::send_message_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=28eb7b10d7e36fbd7a3cdf713562b36e538a2a50#28eb7b10d7e36fbd7a3cdf713562b36e538a2a50"}]
//     evidence={"assembly":"\tcallq\t2025415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374965920,"raw_hex":"e8c7e71e00","size":5,"soft_fail":false,"target_va":5376991504,"va":5374966084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":234,"record_offset":3568,"parent_record_offset":2488,"ancestor_record_offset":2488,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_vec
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":234,"record_offset":2632,"parent_record_offset":2488,"ancestor_record_offset":2488,"inlinee":8445,"source_line":2213,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::crash_pickup::ClientHandle::send_raw_message
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":234,"record_offset":3608,"parent_record_offset":2488,"ancestor_record_offset":2488,"inlinee":7558,"source_line":38,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":2860,"parent_record_offset":2836,"ancestor_record_offset":2488,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":2884,"parent_record_offset":2860,"ancestor_record_offset":2488,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":3204,"parent_record_offset":3180,"ancestor_record_offset":2488,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":3228,"parent_record_offset":3204,"ancestor_record_offset":2488,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":2908,"parent_record_offset":2884,"ancestor_record_offset":2488,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":3252,"parent_record_offset":3228,"ancestor_record_offset":2488,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":2836,"parent_record_offset":2808,"ancestor_record_offset":2488,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":3180,"parent_record_offset":3124,"ancestor_record_offset":2488,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":234,"record_offset":2748,"parent_record_offset":2720,"ancestor_record_offset":2488,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":234,"record_offset":3152,"parent_record_offset":3124,"ancestor_record_offset":2488,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":234,"record_offset":3124,"parent_record_offset":3100,"ancestor_record_offset":2488,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":234,"record_offset":2808,"parent_record_offset":2776,"ancestor_record_offset":2488,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":234,"record_offset":2776,"parent_record_offset":2748,"ancestor_record_offset":2488,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":234,"record_offset":3100,"parent_record_offset":3076,"ancestor_record_offset":2488,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":234,"record_offset":3636,"parent_record_offset":3608,"ancestor_record_offset":2488,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":234,"record_offset":2692,"parent_record_offset":2632,"ancestor_record_offset":2488,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":234,"record_offset":2720,"parent_record_offset":2692,"ancestor_record_offset":2488,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":234,"record_offset":3052,"parent_record_offset":2632,"ancestor_record_offset":2488,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":234,"record_offset":4212,"parent_record_offset":3608,"ancestor_record_offset":2488,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] minidumper::ipc::client::Client::send_message
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=28eb7b10d7e36fbd7a3cdf713562b36e538a2a50#28eb7b10d7e36fbd7a3cdf713562b36e538a2a50"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/28eb7b1/minidumper/src/ipc/client.rs
//     evidence={"module":234,"record_offset":3664,"parent_record_offset":3608,"ancestor_record_offset":2488,"inlinee":7475,"source_line":166,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":234,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":2488,"inlinee":8447,"source_line":28,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":234,"record_offset":3016,"parent_record_offset":2992,"ancestor_record_offset":2488,"inlinee":8092,"source_line":52,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":234,"record_offset":2968,"parent_record_offset":2632,"ancestor_record_offset":2488,"inlinee":8446,"source_line":2177,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::impl$1::default
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x437E
// 首条位置: {"module":234,"inlinee":17278}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":242,"record_offset":5880,"parent_record_offset":5816,"ancestor_record_offset":5816,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":242,"record_offset":5852,"parent_record_offset":5816,"ancestor_record_offset":5816,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":6128,"parent_record_offset":6104,"ancestor_record_offset":5816,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":6152,"parent_record_offset":6128,"ancestor_record_offset":5816,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":6176,"parent_record_offset":6152,"ancestor_record_offset":5816,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":6104,"parent_record_offset":6048,"ancestor_record_offset":5816,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":6076,"parent_record_offset":6048,"ancestor_record_offset":5816,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":6048,"parent_record_offset":6024,"ancestor_record_offset":5816,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":6024,"parent_record_offset":6000,"ancestor_record_offset":5816,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":242,"record_offset":5904,"parent_record_offset":5880,"ancestor_record_offset":5816,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::CrashReporter::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x43A8
// 首条位置: {"module":242,"inlinee":17320}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tiny_glade::crash_pickup::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":242,"record_offset":5816,"parent_record_offset":5788,"ancestor_record_offset":5788,"inlinee":17278,"source_line":69,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":6128,"parent_record_offset":6104,"ancestor_record_offset":5788,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":6152,"parent_record_offset":6128,"ancestor_record_offset":5788,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":6176,"parent_record_offset":6152,"ancestor_record_offset":5788,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":6104,"parent_record_offset":6048,"ancestor_record_offset":5788,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":6076,"parent_record_offset":6048,"ancestor_record_offset":5788,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":6048,"parent_record_offset":6024,"ancestor_record_offset":5788,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":6024,"parent_record_offset":6000,"ancestor_record_offset":5788,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":242,"record_offset":5880,"parent_record_offset":5816,"ancestor_record_offset":5788,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":242,"record_offset":5852,"parent_record_offset":5816,"ancestor_record_offset":5788,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::CrashReporter::run_server
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x43A6
// 首条位置: {"module":242,"inlinee":17318}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":242,"record_offset":13480,"parent_record_offset":13448,"ancestor_record_offset":13448,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":13656,"parent_record_offset":13632,"ancestor_record_offset":13448,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":13680,"parent_record_offset":13656,"ancestor_record_offset":13448,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":13704,"parent_record_offset":13680,"ancestor_record_offset":13448,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":13632,"parent_record_offset":13576,"ancestor_record_offset":13448,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":13604,"parent_record_offset":13576,"ancestor_record_offset":13448,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":13576,"parent_record_offset":13552,"ancestor_record_offset":13448,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":13552,"parent_record_offset":13528,"ancestor_record_offset":13448,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":242,"record_offset":13504,"parent_record_offset":13480,"ancestor_record_offset":13448,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::CrashReporter::spawn_server
// 记录形式: compiled_procedure; instances=1
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":234,"record_offset":4252,"rva":6257376,"va":5374966496}
// 编译实例 RVAs: 0x5f7ae0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9689493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e895d99300","size":5,"soft_fail":false,"target_va":5384656240,"va":5374966742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::process::Child>::id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9723982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e84e609400","size":5,"soft_fail":false,"target_va":5384691472,"va":5374967485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::process::Child>::kill
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9723766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e8765f9400","size":5,"soft_fail":false,"target_va":5384691488,"va":5374967717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::process::Child>::wait
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9723890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e8f25f9400","size":5,"soft_fail":false,"target_va":5384691648,"va":5374967753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::process::Command>::spawn
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9735797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e8758e9400","size":5,"soft_fail":false,"target_va":5384703120,"va":5374967318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::process::windows::Command>::arg
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t9721207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e877559400","size":5,"soft_fail":false,"target_va":5384688336,"va":5374967124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::process::windows::Command>::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9721494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e896569400","size":5,"soft_fail":false,"target_va":5384688544,"va":5374967045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t711487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e83fdb0a00","size":5,"soft_fail":false,"target_va":5375678688,"va":5374967196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=13
//     evidence={"assembly":"\tcallq\t711926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e8f6dc0a00","size":5,"soft_fail":false,"target_va":5375678704,"va":5374966773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6244996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e87cb5a0ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374967183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099440
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5632033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e8df0faaff","size":5,"soft_fail":false,"target_va":5369336896,"va":5374968924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5628893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e8231caaff","size":5,"soft_fail":false,"target_va":5369338848,"va":5374967736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099cc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5628264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e8981eaaff","size":5,"soft_fail":false,"target_va":5369339072,"va":5374967331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b0480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5537541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e8fb80abff","size":5,"soft_fail":false,"target_va":5369431168,"va":5374968704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404379b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1837605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e8dbf5e3ff","size":5,"soft_fail":false,"target_va":5373131184,"va":5374968784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ea38d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9092412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e83cbd8a00","size":5,"soft_fail":false,"target_va":5384059088,"va":5374966671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42089983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e8ff3d8202","size":5,"soft_fail":false,"target_va":5417057056,"va":5374967068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41652782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e82e927b02","size":5,"soft_fail":false,"target_va":5416620769,"va":5374967982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41652791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e837927b02","size":5,"soft_fail":false,"target_va":5416620794,"va":5374967998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<tiny_glade::crash_pickup::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e8ec0c0000","size":5,"soft_fail":false,"target_va":5374970288,"va":5374966975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41653670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e8a6957b02","size":5,"soft_fail":false,"target_va":5416621632,"va":5374967957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9863249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e851809600","size":5,"soft_fail":false,"target_va":5384830080,"va":5374966826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::current_exe
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9745119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e8dfb29400","size":5,"soft_fail":false,"target_va":5384712112,"va":5374966988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::temp_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9748884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e894c19400","size":5,"soft_fail":false,"target_va":5384715440,"va":5374966551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::crash_pickup::client::start
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-793869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e8f3e2f3ff","size":5,"soft_fail":false,"target_va":5374173664,"va":5374967528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] uuid::v4::_$LT$impl$u20$uuid..Uuid$GT$::new_v4
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9092344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374966496,"raw_hex":"e8f8bc8a00","size":5,"soft_fail":false,"target_va":5384058928,"va":5374966579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*%r14","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5374966496,"raw_hex":"41ffd6","size":3,"soft_fail":false,"va":5374967825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":234,"record_offset":4420,"parent_record_offset":4252,"ancestor_record_offset":4252,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":234,"record_offset":5984,"parent_record_offset":4252,"ancestor_record_offset":4252,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":234,"record_offset":5496,"parent_record_offset":4252,"ancestor_record_offset":4252,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":234,"record_offset":7036,"parent_record_offset":4252,"ancestor_record_offset":4252,"inlinee":7484,"source_line":1488,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":234,"record_offset":9132,"parent_record_offset":4252,"ancestor_record_offset":4252,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":234,"record_offset":6024,"parent_record_offset":4252,"ancestor_record_offset":4252,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":234,"record_offset":4892,"parent_record_offset":4252,"ancestor_record_offset":4252,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":234,"record_offset":5864,"parent_record_offset":4252,"ancestor_record_offset":4252,"inlinee":10684,"source_line":2452,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":8392,"parent_record_offset":8368,"ancestor_record_offset":4252,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":8416,"parent_record_offset":8392,"ancestor_record_offset":4252,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":4776,"parent_record_offset":4752,"ancestor_record_offset":4252,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":4800,"parent_record_offset":4776,"ancestor_record_offset":4252,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":8440,"parent_record_offset":8416,"ancestor_record_offset":4252,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":4824,"parent_record_offset":4800,"ancestor_record_offset":4252,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":8368,"parent_record_offset":8340,"ancestor_record_offset":4252,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":4752,"parent_record_offset":4696,"ancestor_record_offset":4252,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":234,"record_offset":9376,"parent_record_offset":9332,"ancestor_record_offset":4252,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":234,"record_offset":9332,"parent_record_offset":9296,"ancestor_record_offset":4252,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":234,"record_offset":8280,"parent_record_offset":8252,"ancestor_record_offset":4252,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":234,"record_offset":4724,"parent_record_offset":4696,"ancestor_record_offset":4252,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":234,"record_offset":4696,"parent_record_offset":4672,"ancestor_record_offset":4252,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":234,"record_offset":8340,"parent_record_offset":8308,"ancestor_record_offset":4252,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":234,"record_offset":8308,"parent_record_offset":8280,"ancestor_record_offset":4252,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":234,"record_offset":4672,"parent_record_offset":4648,"ancestor_record_offset":4252,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":234,"record_offset":8152,"parent_record_offset":8120,"ancestor_record_offset":4252,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":234,"record_offset":8184,"parent_record_offset":8152,"ancestor_record_offset":4252,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":234,"record_offset":8216,"parent_record_offset":8184,"ancestor_record_offset":4252,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":234,"record_offset":8120,"parent_record_offset":8088,"ancestor_record_offset":4252,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":234,"record_offset":8088,"parent_record_offset":8056,"ancestor_record_offset":4252,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":234,"record_offset":4508,"parent_record_offset":4456,"ancestor_record_offset":4252,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":234,"record_offset":8000,"parent_record_offset":7076,"ancestor_record_offset":4252,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":234,"record_offset":4456,"parent_record_offset":4420,"ancestor_record_offset":4252,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$43::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":234,"record_offset":8028,"parent_record_offset":8000,"ancestor_record_offset":4252,"inlinee":17275,"source_line":2940,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":234,"record_offset":8056,"parent_record_offset":8028,"ancestor_record_offset":4252,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":234,"record_offset":9296,"parent_record_offset":9208,"ancestor_record_offset":4252,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":234,"record_offset":10384,"parent_record_offset":10360,"ancestor_record_offset":4252,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":234,"record_offset":8252,"parent_record_offset":8216,"ancestor_record_offset":4252,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":234,"record_offset":4536,"parent_record_offset":4456,"ancestor_record_offset":4252,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":234,"record_offset":8496,"parent_record_offset":8216,"ancestor_record_offset":4252,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":234,"record_offset":8520,"parent_record_offset":8496,"ancestor_record_offset":4252,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=94
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":234,"record_offset":4600,"parent_record_offset":4456,"ancestor_record_offset":4252,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":234,"record_offset":4564,"parent_record_offset":4456,"ancestor_record_offset":4252,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":10420,"parent_record_offset":10384,"ancestor_record_offset":4252,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":10444,"parent_record_offset":10420,"ancestor_record_offset":4252,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":10476,"parent_record_offset":10384,"ancestor_record_offset":4252,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":234,"record_offset":5944,"parent_record_offset":5912,"ancestor_record_offset":4252,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":234,"record_offset":5888,"parent_record_offset":5864,"ancestor_record_offset":4252,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":234,"record_offset":9716,"parent_record_offset":9692,"ancestor_record_offset":4252,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::process::Command::arg
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\process.rs
//     evidence={"module":234,"record_offset":7916,"parent_record_offset":7076,"ancestor_record_offset":4252,"inlinee":8165,"source_line":727,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::process::Command::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\process.rs
//     evidence={"module":234,"record_offset":7144,"parent_record_offset":7076,"ancestor_record_offset":4252,"inlinee":8166,"source_line":664,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":234,"record_offset":5912,"parent_record_offset":5888,"ancestor_record_offset":4252,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::impl$2::spawn_server::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":234,"record_offset":7076,"parent_record_offset":7036,"ancestor_record_offset":4252,"inlinee":17273,"source_line":104,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::impl$2::spawn_server::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":234,"record_offset":9208,"parent_record_offset":9168,"ancestor_record_offset":4252,"inlinee":17276,"source_line":113,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::impl$2::spawn_server::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4379
// 首条位置: {"module":234,"inlinee":17273}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":234,"record_offset":8000,"parent_record_offset":7076,"ancestor_record_offset":7076,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] std::process::Command::arg
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\process.rs
//     evidence={"module":234,"record_offset":7916,"parent_record_offset":7076,"ancestor_record_offset":7076,"inlinee":8165,"source_line":727,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] std::process::Command::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\process.rs
//     evidence={"module":234,"record_offset":7144,"parent_record_offset":7076,"ancestor_record_offset":7076,"inlinee":8166,"source_line":664,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":8392,"parent_record_offset":8368,"ancestor_record_offset":7076,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":8416,"parent_record_offset":8392,"ancestor_record_offset":7076,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":7424,"parent_record_offset":7400,"ancestor_record_offset":7076,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":7448,"parent_record_offset":7424,"ancestor_record_offset":7076,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":8440,"parent_record_offset":8416,"ancestor_record_offset":7076,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":7472,"parent_record_offset":7448,"ancestor_record_offset":7076,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":8368,"parent_record_offset":8340,"ancestor_record_offset":7076,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":7400,"parent_record_offset":7344,"ancestor_record_offset":7076,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":234,"record_offset":8280,"parent_record_offset":8252,"ancestor_record_offset":7076,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":234,"record_offset":7372,"parent_record_offset":7344,"ancestor_record_offset":7076,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":234,"record_offset":7344,"parent_record_offset":7320,"ancestor_record_offset":7076,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":234,"record_offset":8340,"parent_record_offset":8308,"ancestor_record_offset":7076,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":234,"record_offset":8308,"parent_record_offset":8280,"ancestor_record_offset":7076,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":234,"record_offset":7320,"parent_record_offset":7296,"ancestor_record_offset":7076,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":234,"record_offset":8152,"parent_record_offset":8120,"ancestor_record_offset":7076,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":234,"record_offset":8184,"parent_record_offset":8152,"ancestor_record_offset":7076,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":234,"record_offset":8216,"parent_record_offset":8184,"ancestor_record_offset":7076,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":234,"record_offset":8120,"parent_record_offset":8088,"ancestor_record_offset":7076,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":234,"record_offset":8088,"parent_record_offset":8056,"ancestor_record_offset":7076,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$43::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":234,"record_offset":8028,"parent_record_offset":8000,"ancestor_record_offset":7076,"inlinee":17275,"source_line":2940,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":234,"record_offset":8056,"parent_record_offset":8028,"ancestor_record_offset":7076,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":234,"record_offset":8252,"parent_record_offset":8216,"ancestor_record_offset":7076,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":234,"record_offset":8496,"parent_record_offset":8216,"ancestor_record_offset":7076,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":234,"record_offset":8520,"parent_record_offset":8496,"ancestor_record_offset":7076,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":234,"record_offset":7176,"parent_record_offset":7144,"ancestor_record_offset":7076,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::impl$2::spawn_server::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x437C
// 首条位置: {"module":234,"inlinee":17276}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":234,"record_offset":9296,"parent_record_offset":9208,"ancestor_record_offset":9208,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":234,"record_offset":9564,"parent_record_offset":9208,"ancestor_record_offset":9208,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":9432,"parent_record_offset":9408,"ancestor_record_offset":9208,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":9456,"parent_record_offset":9432,"ancestor_record_offset":9208,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":9480,"parent_record_offset":9456,"ancestor_record_offset":9208,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":9408,"parent_record_offset":9376,"ancestor_record_offset":9208,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":234,"record_offset":9376,"parent_record_offset":9332,"ancestor_record_offset":9208,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":234,"record_offset":9332,"parent_record_offset":9296,"ancestor_record_offset":9208,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":234,"record_offset":10384,"parent_record_offset":10360,"ancestor_record_offset":9208,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":234,"record_offset":9528,"parent_record_offset":9332,"ancestor_record_offset":9208,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":10420,"parent_record_offset":10384,"ancestor_record_offset":9208,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":10444,"parent_record_offset":10420,"ancestor_record_offset":9208,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":10476,"parent_record_offset":10384,"ancestor_record_offset":9208,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":234,"record_offset":9716,"parent_record_offset":9692,"ancestor_record_offset":9208,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00234.jsonl.gz"}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::ClientHandle
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<tiny_glade::crash_pickup::ClientHandle>", "constructor": "core::option::Option", "arguments": ["tiny_glade::crash_pickup::ClientHandle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >", "constructor": "enum2$", "arguments": ["core::option::Option<tiny_glade::crash_pickup::ClientHandle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::impl$0::try_insert::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> > >", "constructor": "std::sync::once_lock::impl$0::try_insert::closure_env$0", "arguments": ["enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >,std::sync::once_lock::impl$0::try_insert::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> > > >", "constructor": "std::sync::once_lock::impl$0::get_or_init::closure_env$0", "arguments": ["enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >", "std::sync::once_lock::impl$0::try_insert::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >,std::sync::once_lock::impl$0::try_insert::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> > > >,never$>", "constructor": "std::sync::once_lock::impl$0::initialize::closure_env$0", "arguments": ["enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >", "std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >,std::sync::once_lock::impl$0::try_insert::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> > > >", "never$"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once::impl$2::call_once_force::closure$0<std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >,std::sync::once_lock::impl$0::try_insert::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> > > >,never$> >", "constructor": "std::sync::once::impl$2::call_once_force::closure$0", "arguments": ["std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >,std::sync::once_lock::impl$0::try_insert::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> > > >,never$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::OnceLock::initialize<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >,std::sync::once_lock::impl$0::try_insert::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> > > >,never$>", "constructor": "std::sync::once_lock::OnceLock::initialize", "arguments": ["enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >", "std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >,std::sync::once_lock::impl$0::try_insert::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> > > >", "never$"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >,std::sync::once_lock::impl$0::try_insert::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> > > >,never$> >", "constructor": "std::sync::once::impl$2::call_once_force::closure_env$0", "arguments": ["std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >,std::sync::once_lock::impl$0::try_insert::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> > > >,never$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >,std::sync::once_lock::impl$0::try_insert::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> > > >,never$> >,tuple$<ref$<std::sync::once::OnceState> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> >,std::sync::once_lock::impl$0::try_insert::closure_env$0<enum2$<core::option::Option<tiny_glade::crash_pickup::ClientHandle> > > >,never$> >", "tuple$<ref$<std::sync::once::OnceState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::crash_pickup::ClientHandle>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::crash_pickup::ClientHandle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::CrashHandlerHooks
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::crash_pickup::CrashHandlerHooks>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::crash_pickup::CrashHandlerHooks"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<tiny_glade::crash_pickup::CrashHandlerHooks,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["tiny_glade::crash_pickup::CrashHandlerHooks", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::CrashReporter
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::Error
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tiny_glade::crash_pickup::Error>", "constructor": "enum2$", "arguments": ["tiny_glade::crash_pickup::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<>,enum2$<tiny_glade::crash_pickup::Error> >", "constructor": "core::result::Result", "arguments": ["tuple$<>", "enum2$<tiny_glade::crash_pickup::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<>,enum2$<tiny_glade::crash_pickup::Error> > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<>,enum2$<tiny_glade::crash_pickup::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<tiny_glade::crash_pickup::Error> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tuple$<>,enum2$<tiny_glade::crash_pickup::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<tiny_glade::crash_pickup::Error> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<tiny_glade::crash_pickup::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ReanchorDecorators>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ReanchorDecorators>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ReanchorDecorators>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>", "bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::ClientHandle::send_message
// 信息 ID: be8da2fe4bca55854f17b631
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x42ffdb -> 0x5f78a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x430060 -> 0x5f78a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::crash_service::start::closure_env$2,tuple$<> > @ 0x5e8fba -> 0x5f78a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::ClientHandle::send_raw_message
// 信息 ID: 0bf9d107e312f52406f289e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::ClientHandle::send_message; module=234, inline_record=3608; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::CrashReporter::new
// 信息 ID: 60bf4c51147c4d9f66a09f71
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::main; module=242, inline_record=5788; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::CrashReporter::run_server
// 信息 ID: 4c9a7040af7e77e3390d85c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::main; module=242, inline_record=13448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::CrashReporter::spawn_server
// 信息 ID: 705d8e8b3b8db36bb9bb1b64
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x642a94 -> 0x5f7ae0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::impl$1::default
// 信息 ID: deb43868e6c960fd4b103ee1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::main; module=242, inline_record=5816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::impl$2::spawn_server::closure$0
// 信息 ID: 26f1fab86b3954e7fbd408b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::crash_pickup::impl$2::spawn_server", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::CrashReporter::spawn_server; module=234, inline_record=7076; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::impl$2::spawn_server::closure$1
// 信息 ID: ce0b7def9ea438ec27661805
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::crash_pickup::impl$2::spawn_server", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::CrashReporter::spawn_server; module=234, inline_record=9208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::impl$4::fmt
// 信息 ID: 89a17f37212516fffd9926c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::impl$6::from
// 信息 ID: d0f3dc194f7387a9d3bfb24f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::client::start; module=186, inline_record=2720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::impl$8::fmt
// 信息 ID: b22702fbcad039e268db94c9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::impl$9::clone
// 信息 ID: c90d0e984b35aa43bdffad9e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::main; module=242, inline_record=22440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
