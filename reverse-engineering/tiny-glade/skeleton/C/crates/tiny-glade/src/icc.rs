// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/icc.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 1b8d882b44a8b98d5c49af200ae609155fd46c977aa529947fdeac5575793cb7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: icc.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 61
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use winit; // namespace candidate
//   package identities: [{"name":"winit","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e","version":"0.29.15"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::icc::get_monitor_icc_profile
// 记录形式: compiled_procedure; instances=1
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":61,"record_offset":496,"rva":2020848,"va":5370729968}
// 编译实例 RVAs: 0x1ed5f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::ffi::c_str::CString>::_from_vec_unchecked
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14063680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370729968,"raw_hex":"e84098d600","size":5,"soft_fail":false,"target_va":5384794128,"va":5370730443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <alloc::ffi::c_str::CString>::from_vec_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14063035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370729968,"raw_hex":"e8bb95d600","size":5,"soft_fail":false,"target_va":5384793648,"va":5370730608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <alloc::ffi::c_str::CString>::into_string
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14062767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370729968,"raw_hex":"e8af94d600","size":5,"soft_fail":false,"target_va":5384793408,"va":5370730636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::wtf8::Wtf8>::to_string_lossy
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14062083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370729968,"raw_hex":"e80392d600","size":5,"soft_fail":false,"target_va":5384792224,"va":5370730136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t13928421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370729968,"raw_hex":"e8e587d400","size":5,"soft_fail":false,"target_va":5384659264,"va":5370730838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4948478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370729968,"raw_hex":"e8fe814b00","size":5,"soft_fail":false,"target_va":5375678688,"va":5370730205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc_zeroed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4948242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370729968,"raw_hex":"e812814b00","size":5,"soft_fail":false,"target_va":5375678736,"va":5370730489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=13
//     evidence={"assembly":"\tcallq\t4948449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370729968,"raw_hex":"e8e1814b00","size":5,"soft_fail":false,"target_va":5375678704,"va":5370730250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2008000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370729968,"raw_hex":"e8405ce1ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5370730187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1393201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370729968,"raw_hex":"e8cfbdeaff","size":5,"soft_fail":false,"target_va":5369338848,"va":5370732044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d66e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45585777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370729968,"raw_hex":"e87195b702","size":5,"soft_fail":false,"target_va":5416316480,"va":5370730698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d670c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45586167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370729968,"raw_hex":"e8f796b702","size":5,"soft_fail":false,"target_va":5416317120,"va":5370730948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d67eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45589527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370729968,"raw_hex":"e817a4b702","size":5,"soft_fail":false,"target_va":5416320688,"va":5370731156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1b1f0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46324233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370729968,"raw_hex":"e809dac202","size":5,"soft_fail":false,"target_va":5417054704,"va":5370730466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1b200
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46324036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370729968,"raw_hex":"e844d9c202","size":5,"soft_fail":false,"target_va":5417054720,"va":5370730679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1b210
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46324188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370729968,"raw_hex":"e8dcd9c202","size":5,"soft_fail":false,"target_va":5417054736,"va":5370730543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46326824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370729968,"raw_hex":"e828e4c202","size":5,"soft_fail":false,"target_va":5417057056,"va":5370730227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t45890619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370729968,"raw_hex":"e83b3cbc02","size":5,"soft_fail":false,"target_va":5416620794,"va":5370730170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t45890371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370729968,"raw_hex":"e8433bbc02","size":5,"soft_fail":false,"target_va":5416621632,"va":5370731256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::memchr::memchr_aligned
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t14100248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370729968,"raw_hex":"e81827d700","size":5,"soft_fail":false,"target_va":5384830608,"va":5370730355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t5590335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370729968,"raw_hex":"e83f4d5500","size":5,"soft_fail":false,"target_va":5376320352,"va":5370730012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::util::decode_wide
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t5589691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370729968,"raw_hex":"e8bb4a5500","size":5,"soft_fail":false,"target_va":5376319808,"va":5370730112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\ffi\c_str.rs
//     evidence={"module":61,"record_offset":3184,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":6244,"source_line":257,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":61,"record_offset":3860,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":9755,"source_line":1761,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":61,"record_offset":3584,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_direct] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":61,"record_offset":4664,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":61,"record_offset":3888,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":61,"record_offset":4692,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":61,"record_offset":5444,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":61,"record_offset":4636,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":61,"record_offset":5144,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":61,"record_offset":656,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":9750,"source_line":112,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":61,"record_offset":1740,"parent_record_offset":1716,"ancestor_record_offset":496,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":61,"record_offset":1764,"parent_record_offset":1740,"ancestor_record_offset":496,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":61,"record_offset":2104,"parent_record_offset":2080,"ancestor_record_offset":496,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":61,"record_offset":2128,"parent_record_offset":2104,"ancestor_record_offset":496,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":61,"record_offset":1788,"parent_record_offset":1764,"ancestor_record_offset":496,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":61,"record_offset":3800,"parent_record_offset":3776,"ancestor_record_offset":496,"inlinee":6459,"source_line":172,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":61,"record_offset":2152,"parent_record_offset":2128,"ancestor_record_offset":496,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":61,"record_offset":1716,"parent_record_offset":1556,"ancestor_record_offset":496,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":61,"record_offset":3728,"parent_record_offset":3700,"ancestor_record_offset":496,"inlinee":6458,"source_line":434,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":61,"record_offset":2080,"parent_record_offset":2056,"ancestor_record_offset":496,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$4::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":61,"record_offset":1224,"parent_record_offset":1188,"ancestor_record_offset":496,"inlinee":6483,"source_line":349,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":61,"record_offset":4472,"parent_record_offset":4448,"ancestor_record_offset":496,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$1::new::impl$0::spec_new_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\ffi\c_str.rs
//     evidence={"module":61,"record_offset":3220,"parent_record_offset":3184,"ancestor_record_offset":496,"inlinee":6279,"source_line":263,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\ffi\c_str.rs
//     evidence={"module":61,"record_offset":4420,"parent_record_offset":4396,"ancestor_record_offset":496,"inlinee":4646,"source_line":700,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":61,"record_offset":988,"parent_record_offset":964,"ancestor_record_offset":496,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":61,"record_offset":1484,"parent_record_offset":1452,"ancestor_record_offset":496,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":61,"record_offset":3640,"parent_record_offset":3612,"ancestor_record_offset":496,"inlinee":8417,"source_line":219,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":61,"record_offset":2388,"parent_record_offset":2360,"ancestor_record_offset":496,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":61,"record_offset":2056,"parent_record_offset":2032,"ancestor_record_offset":496,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":61,"record_offset":1036,"parent_record_offset":1012,"ancestor_record_offset":496,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":61,"record_offset":1012,"parent_record_offset":988,"ancestor_record_offset":496,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":61,"record_offset":1556,"parent_record_offset":1516,"ancestor_record_offset":496,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":61,"record_offset":1516,"parent_record_offset":1484,"ancestor_record_offset":496,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":61,"record_offset":3668,"parent_record_offset":3640,"ancestor_record_offset":496,"inlinee":8418,"source_line":588,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":61,"record_offset":2032,"parent_record_offset":2008,"ancestor_record_offset":496,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":61,"record_offset":1604,"parent_record_offset":1556,"ancestor_record_offset":496,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":61,"record_offset":1348,"parent_record_offset":1316,"ancestor_record_offset":496,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":61,"record_offset":1380,"parent_record_offset":1348,"ancestor_record_offset":496,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":61,"record_offset":1412,"parent_record_offset":1380,"ancestor_record_offset":496,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":61,"record_offset":1316,"parent_record_offset":1284,"ancestor_record_offset":496,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":61,"record_offset":1284,"parent_record_offset":1252,"ancestor_record_offset":496,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$113::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":61,"record_offset":1188,"parent_record_offset":1156,"ancestor_record_offset":496,"inlinee":9754,"source_line":3043,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":61,"record_offset":1156,"parent_record_offset":704,"ancestor_record_offset":496,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":61,"record_offset":1252,"parent_record_offset":1188,"ancestor_record_offset":496,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":61,"record_offset":964,"parent_record_offset":940,"ancestor_record_offset":496,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":61,"record_offset":940,"parent_record_offset":916,"ancestor_record_offset":496,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":61,"record_offset":1452,"parent_record_offset":1412,"ancestor_record_offset":496,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":61,"record_offset":916,"parent_record_offset":892,"ancestor_record_offset":496,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$3::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":61,"record_offset":3612,"parent_record_offset":3584,"ancestor_record_offset":496,"inlinee":9075,"source_line":49,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::Wtf8Buf::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":61,"record_offset":892,"parent_record_offset":868,"ancestor_record_offset":496,"inlinee":6907,"source_line":175,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":61,"record_offset":3968,"parent_record_offset":3928,"ancestor_record_offset":496,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":61,"record_offset":3928,"parent_record_offset":3888,"ancestor_record_offset":496,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":61,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":496,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":61,"record_offset":1676,"parent_record_offset":1652,"ancestor_record_offset":496,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":61,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":496,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":61,"record_offset":1844,"parent_record_offset":1412,"ancestor_record_offset":496,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":61,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":496,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":61,"record_offset":1932,"parent_record_offset":704,"ancestor_record_offset":496,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":61,"record_offset":780,"parent_record_offset":704,"ancestor_record_offset":496,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\memchr.rs
//     evidence={"module":61,"record_offset":3264,"parent_record_offset":3220,"ancestor_record_offset":496,"inlinee":6283,"source_line":25,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr_naive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\memchr.rs
//     evidence={"module":61,"record_offset":3304,"parent_record_offset":3264,"ancestor_record_offset":496,"inlinee":6284,"source_line":35,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":61,"record_offset":1100,"parent_record_offset":704,"ancestor_record_offset":496,"inlinee":6789,"source_line":966,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":61,"record_offset":844,"parent_record_offset":820,"ancestor_record_offset":496,"inlinee":6903,"source_line":656,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":61,"record_offset":820,"parent_record_offset":704,"ancestor_record_offset":496,"inlinee":6901,"source_line":674,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Buf::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":61,"record_offset":868,"parent_record_offset":844,"ancestor_record_offset":496,"inlinee":6905,"source_line":157,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":61,"record_offset":1124,"parent_record_offset":1100,"ancestor_record_offset":496,"inlinee":6790,"source_line":260,"archive":"_pdb/C/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":61,"record_offset":704,"parent_record_offset":656,"ancestor_record_offset":496,"inlinee":9752,"source_line":165,"archive":"_pdb/C/modules/00061.jsonl.gz"}
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
// 完整结构及原始类型记录: icc.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: icc.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: icc.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::system::query::Query<ref$<country_core::resources::window_lights::WindowLight>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::LightWindowCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>", "bevy_ecs::system::query::Query<ref$<country_core::resources::window_lights::WindowLight>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::LightWindowCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::window_lights::WindowLight>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::LightWindowCmd>

// ------------------------------------------------------------------
// 函数: tiny_glade::icc::get_monitor_icc_profile
// 信息 ID: 564a1e86f40d41b6a855ec59
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::winit_event_loop::run_event_loop @ 0x5213a2 -> 0x1ed5f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
