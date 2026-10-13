// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/usdl/src/event.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 0152f932e774467c451e61e53f50762235397df43bd60f0ae92ee5b6e5d7a558
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: event.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 281, 283
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use usdl; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"2.11.0"},{"name":"usdl","source":"local_or_path","version":"0.1.0"}]
// use usdl_sys; // namespace candidate
//   package identities: [{"name":"usdl-sys","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: usdl::event::WindowEvent::from_ll
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4A73
// 首条位置: {"module":283,"inlinee":19059}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::event::set_event_enabled
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":283,"record_offset":260,"rva":7120336,"va":5375829456}
// 编译实例 RVAs: 0x6ca5d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] usdl_sys::event::uSDL_EventState
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375829456,"raw_hex":"e8a46c9300","size":5,"soft_fail":false,"target_va":5385491072,"va":5375829463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::event::impl$1::next
// 记录形式: compiled_procedure; instances=1
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":283,"record_offset":2460,"rva":7121392,"va":5375830512}
// 编译实例 RVAs: 0x6ca9f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: event.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830512,"raw_hex":"e8a0fbffff","size":5,"soft_fail":false,"target_va":5375829488,"va":5375830603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140957040
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: event.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2672183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830512,"raw_hex":"e837c62800","size":5,"soft_fail":false,"target_va":5378502720,"va":5375830532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::event::uSDL_PollEvent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9660431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830512,"raw_hex":"e80f689300","size":5,"soft_fail":false,"target_va":5385490976,"va":5375830540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl::event::poll_event
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/event.rs
//     evidence={"module":283,"record_offset":2572,"parent_record_offset":2460,"ancestor_record_offset":2460,"inlinee":19028,"source_line":158,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::event::poll_event
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4A54
// 首条位置: {"module":281,"inlinee":19028}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::event::wait_event_timeout
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4A55
// 首条位置: {"module":281,"inlinee":19029}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::event::from_ll
// 记录形式: compiled_procedure; instances=1
// 源码行: 178
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":283,"record_offset":368,"rva":7120368,"va":5375829488}
// 编译实例 RVAs: 0x6ca5f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_string_lossy
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8963565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375829488,"raw_hex":"e8edc58800","size":5,"soft_fail":false,"target_va":5384793392,"va":5375829822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-151519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375829488,"raw_hex":"e821b0fdff","size":5,"soft_fail":false,"target_va":5375678688,"va":5375830202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-151784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375829488,"raw_hex":"e818affdff","size":5,"soft_fail":false,"target_va":5375678704,"va":5375830483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: event.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7107996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375829488,"raw_hex":"e8648a93ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375830183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: event.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41226823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375829488,"raw_hex":"e847127502","size":5,"soft_fail":false,"target_va":5417057056,"va":5375830228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bf30
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: event.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41228285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375829488,"raw_hex":"e8fd177502","size":5,"soft_fail":false,"target_va":5417058096,"va":5375829806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t40790920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375829488,"raw_hex":"e8886b6e02","size":5,"soft_fail":false,"target_va":5416620794,"va":5375829869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::keyboard::Keycode::from_i32
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375829488,"raw_hex":"e807100000","size":5,"soft_fail":false,"target_va":5375833824,"va":5375829716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::keyboard::Scancode::from_i32
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375829488,"raw_hex":"e85a100000","size":5,"soft_fail":false,"target_va":5375833920,"va":5375829729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::event::mouse_button_from_raw
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375829488,"raw_hex":"e8036b9300","size":5,"soft_fail":false,"target_va":5385491168,"va":5375829976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::event::uSDL_free
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9660953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375829488,"raw_hex":"e8196a9300","size":5,"soft_fail":false,"target_va":5385491280,"va":5375830322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::borrow::Cow::into_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":283,"record_offset":676,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":5422,"source_line":331,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_direct] core::ffi::c_str::CStr::from_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ffi\c_str.rs
//     evidence={"module":283,"record_offset":580,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":8370,"source_line":253,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":283,"record_offset":2096,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_direct] usdl::event::WindowEvent::from_ll
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/event.rs
//     evidence={"module":283,"record_offset":488,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":19059,"source_line":30,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_direct] usdl::keyboard::Mod::from_bits_truncate
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":283,"record_offset":516,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":19060,"source_line":663,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":283,"record_offset":1124,"parent_record_offset":1100,"ancestor_record_offset":368,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":283,"record_offset":1148,"parent_record_offset":1124,"ancestor_record_offset":368,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":283,"record_offset":2272,"parent_record_offset":2248,"ancestor_record_offset":368,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":283,"record_offset":2296,"parent_record_offset":2272,"ancestor_record_offset":368,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":283,"record_offset":1172,"parent_record_offset":1148,"ancestor_record_offset":368,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":283,"record_offset":2320,"parent_record_offset":2296,"ancestor_record_offset":368,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":283,"record_offset":1100,"parent_record_offset":948,"ancestor_record_offset":368,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":283,"record_offset":2248,"parent_record_offset":2192,"ancestor_record_offset":368,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":283,"record_offset":884,"parent_record_offset":856,"ancestor_record_offset":368,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":283,"record_offset":2220,"parent_record_offset":2192,"ancestor_record_offset":368,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":283,"record_offset":2192,"parent_record_offset":2168,"ancestor_record_offset":368,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":283,"record_offset":948,"parent_record_offset":912,"ancestor_record_offset":368,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":283,"record_offset":912,"parent_record_offset":884,"ancestor_record_offset":368,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":283,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":368,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":283,"record_offset":988,"parent_record_offset":948,"ancestor_record_offset":368,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":283,"record_offset":768,"parent_record_offset":740,"ancestor_record_offset":368,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":283,"record_offset":796,"parent_record_offset":768,"ancestor_record_offset":368,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":283,"record_offset":824,"parent_record_offset":796,"ancestor_record_offset":368,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":283,"record_offset":740,"parent_record_offset":712,"ancestor_record_offset":368,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":283,"record_offset":712,"parent_record_offset":676,"ancestor_record_offset":368,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":283,"record_offset":856,"parent_record_offset":824,"ancestor_record_offset":368,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":283,"record_offset":1012,"parent_record_offset":988,"ancestor_record_offset":368,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ffi\c_str.rs
//     evidence={"module":283,"record_offset":608,"parent_record_offset":580,"ancestor_record_offset":368,"inlinee":8372,"source_line":737,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":283,"record_offset":636,"parent_record_offset":608,"ancestor_record_offset":368,"inlinee":8374,"source_line":2434,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":283,"record_offset":1060,"parent_record_offset":1036,"ancestor_record_offset":368,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":283,"record_offset":1036,"parent_record_offset":1012,"ancestor_record_offset":368,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":283,"record_offset":1228,"parent_record_offset":824,"ancestor_record_offset":368,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":283,"record_offset":1252,"parent_record_offset":1228,"ancestor_record_offset":368,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":283,"record_offset":2120,"parent_record_offset":2096,"ancestor_record_offset":368,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00283.jsonl.gz"}
//   [pdb_inline_transitive] usdl::keyboard::_::InternalBitFlags::from_bits_truncate
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":283,"record_offset":544,"parent_record_offset":516,"ancestor_record_offset":368,"inlinee":19061,"source_line":663,"archive":"_pdb/C/modules/00283.jsonl.gz"}
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
// 完整结构及原始类型记录: event.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] usdl::event::WindowEvent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: event.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: event.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: usdl::event::WindowEvent::from_ll
// 信息 ID: b0d087d37fb566da10ea7368
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] usdl::event::from_ll; module=283, inline_record=488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::event::from_ll
// 信息 ID: 3718232f7ea29f85523d223c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::sdl::EventPump::wait_event_timeout @ 0x6ca41e -> 0x6ca5f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl::event::impl$1::next @ 0x6caa4b -> 0x6ca5f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::event::impl$1::next
// 信息 ID: e5c86a1a5bc3385c10a59bb3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_event_loop::process_event_batch<tiny_glade::sdl_event_loop::run_event_loop::closure_env$0> @ 0x52e257 -> 0x6ca9f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::sdl_event_loop::process_event_batch<tiny_glade::sdl_event_loop::run_event_loop::closure_env$0> @ 0x52e386 -> 0x6ca9f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_iced::gui_runner_sdl::run_gui<tg_gpu_selector::gpu_selection::Controls> @ 0x52fc06 -> 0x6ca9f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::event::poll_event
// 信息 ID: cefee53414128ade9d09782d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] usdl::event::impl$1::next; module=283, inline_record=2572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::event::set_event_enabled
// 信息 ID: 2aeac82ba1a6cc4a21243d91
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::init_sdl @ 0x3fa8df -> 0x6ca5d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::event::wait_event_timeout
// 信息 ID: 554cdace7aeda9e3589929ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] usdl::sdl::EventPump::wait_event_timeout; module=281, inline_record=2272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
