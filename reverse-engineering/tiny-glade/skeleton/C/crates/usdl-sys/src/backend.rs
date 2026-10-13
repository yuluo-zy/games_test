// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/usdl-sys/src/backend.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 4a0b0df18d6e1133d32e1ec961b175f75653681a297aeab541753e0147ccbba6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: backend.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 16, 198, 288, 1005, 1006, 1007, 1008, 1010, 1012, 1021
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use libloading; // namespace candidate
//   package identities: [{"name":"libloading","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.7.3"},{"name":"libloading","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.3"}]
// use std; // namespace candidate
//   package identities: []
// use usdl_sys; // namespace candidate
//   package identities: [{"name":"usdl-sys","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::DynamicBackend::load
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":288,"record_offset":1440,"rva":7125376,"va":5375834496}
// 编译实例 RVAs: 0x6cb980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$libloading..os..windows..Library$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8218883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375834496,"raw_hex":"e803697d00","size":5,"soft_fail":false,"target_va":5384056736,"va":5375837848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002580
//     owner=unresolved_namespace; count=43
//     target_aliases=29 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9661312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375834496,"raw_hex":"e8806b9300","size":5,"soft_fail":false,"target_va":5385495936,"va":5375834619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::load_with_flags
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9660136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375834496,"raw_hex":"e8e8669300","size":5,"soft_fail":false,"target_va":5385494688,"va":5375834547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":288,"record_offset":3372,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":288,"record_offset":3148,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=43
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":288,"record_offset":1656,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=38
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":288,"record_offset":3176,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":288,"record_offset":1580,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":18181,"source_line":83,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":288,"record_offset":3396,"parent_record_offset":3372,"ancestor_record_offset":1440,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] libloading::os::windows::Library::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/os/windows/mod.rs
//     evidence={"module":288,"record_offset":1608,"parent_record_offset":1580,"ancestor_record_offset":1440,"inlinee":18183,"source_line":69,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::impl$2::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":288,"record_offset":1124,"rva":7125056,"va":5375834176}
// 编译实例 RVAs: 0x6cb840
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t8991838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375834176,"raw_hex":"e85e348900","size":5,"soft_fail":false,"target_va":5384826176,"va":5375834333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t8989828","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375834176,"raw_hex":"e9842c8900","size":5,"soft_fail":false,"target_va":5384824080,"va":5375834247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":288,"record_offset":1240,"parent_record_offset":1124,"ancestor_record_offset":1124,"inlinee":4604,"source_line":2121,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::impl$13::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":16,"record_offset":56948,"rva":1072512,"va":5369781632}
// 编译实例 RVAs: 0x105d80, 0x562aa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_struct_field1_finish
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t15039296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369781632,"raw_hex":"e8407be500","size":5,"soft_fail":false,"target_va":5384821184,"va":5369781883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_struct_field2_finish
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t15039543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369781632,"raw_hex":"e8377ce500","size":5,"soft_fail":false,"target_va":5384821360,"va":5369781812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t15042370","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5369781632,"raw_hex":"e94287e500","size":5,"soft_fail":false,"target_va":5384824080,"va":5369781705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_library<ref$<ref$<str$> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":31900,"rva":16787344,"va":5385496464}
// 编译实例 RVAs: 0x1002790
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::wtf8::Wtf8>::to_owned
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-703298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385496464,"raw_hex":"e8be44f5ff","size":5,"soft_fail":false,"target_va":5384793264,"va":5385496557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$libloading..os..windows..Library$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-1440186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385496464,"raw_hex":"e84606eaff","size":5,"soft_fail":false,"target_va":5384056736,"va":5385496917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-9818038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385496464,"raw_hex":"e84a306aff","size":5,"soft_fail":false,"target_va":5375678704,"va":5385496737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31560336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385496464,"raw_hex":"e89092e101","size":5,"soft_fail":false,"target_va":5417057056,"va":5385496715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::once_lock::OnceLock$LT$T$GT$::initialize
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31130005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385496464,"raw_hex":"e89501db01","size":5,"soft_fail":false,"target_va":5416627175,"va":5385497165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::backend::DynamicBackend::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9662105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385496464,"raw_hex":"e867916cff","size":5,"soft_fail":false,"target_va":5375834496,"va":5385496596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1021,"record_offset":32400,"parent_record_offset":31900,"ancestor_record_offset":31900,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1021,"record_offset":33916,"parent_record_offset":31900,"ancestor_record_offset":31900,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":32484,"parent_record_offset":31900,"ancestor_record_offset":31900,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":33716,"parent_record_offset":31900,"ancestor_record_offset":31900,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":33888,"parent_record_offset":31900,"ancestor_record_offset":31900,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1021,"record_offset":32428,"parent_record_offset":31900,"ancestor_record_offset":31900,"inlinee":5109,"source_line":986,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1021,"record_offset":32064,"parent_record_offset":31900,"ancestor_record_offset":31900,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] std::sync::once_lock::OnceLock::set
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1021,"record_offset":33272,"parent_record_offset":31900,"ancestor_record_offset":31900,"inlinee":15282,"source_line":238,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1021,"record_offset":32772,"parent_record_offset":32748,"ancestor_record_offset":31900,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1021,"record_offset":32796,"parent_record_offset":32772,"ancestor_record_offset":31900,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1021,"record_offset":32820,"parent_record_offset":32796,"ancestor_record_offset":31900,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1021,"record_offset":32748,"parent_record_offset":32692,"ancestor_record_offset":31900,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1021,"record_offset":32720,"parent_record_offset":32692,"ancestor_record_offset":31900,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1021,"record_offset":32692,"parent_record_offset":32668,"ancestor_record_offset":31900,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1021,"record_offset":32668,"parent_record_offset":32644,"ancestor_record_offset":31900,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1021,"record_offset":32524,"parent_record_offset":32484,"ancestor_record_offset":31900,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1021,"record_offset":32160,"parent_record_offset":32136,"ancestor_record_offset":31900,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1021,"record_offset":32184,"parent_record_offset":32160,"ancestor_record_offset":31900,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1021,"record_offset":32112,"parent_record_offset":32088,"ancestor_record_offset":31900,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1021,"record_offset":33420,"parent_record_offset":33388,"ancestor_record_offset":31900,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1021,"record_offset":33360,"parent_record_offset":33312,"ancestor_record_offset":31900,"inlinee":14249,"source_line":317,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get_or_try_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1021,"record_offset":33388,"parent_record_offset":33360,"ancestor_record_offset":31900,"inlinee":14250,"source_line":398,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1021,"record_offset":32088,"parent_record_offset":32064,"ancestor_record_offset":31900,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::try_insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1021,"record_offset":33312,"parent_record_offset":33272,"ancestor_record_offset":31900,"inlinee":15283,"source_line":278,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1021,"record_offset":32452,"parent_record_offset":32428,"ancestor_record_offset":31900,"inlinee":5111,"source_line":265,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1021,"record_offset":32136,"parent_record_offset":32112,"ancestor_record_offset":31900,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_library::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":32892,"parent_record_offset":32484,"ancestor_record_offset":31900,"inlinee":64848,"source_line":162,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_library::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":33772,"parent_record_offset":33744,"ancestor_record_offset":31900,"inlinee":64847,"source_line":171,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_library::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 162
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFD50
// 首条位置: {"module":1021,"inlinee":64848}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1021,"record_offset":32924,"parent_record_offset":32892,"ancestor_record_offset":32892,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1021,"record_offset":33148,"parent_record_offset":33124,"ancestor_record_offset":32892,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1021,"record_offset":33172,"parent_record_offset":33148,"ancestor_record_offset":32892,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1021,"record_offset":33196,"parent_record_offset":33172,"ancestor_record_offset":32892,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1021,"record_offset":33124,"parent_record_offset":33068,"ancestor_record_offset":32892,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1021,"record_offset":33096,"parent_record_offset":33068,"ancestor_record_offset":32892,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1021,"record_offset":33068,"parent_record_offset":33044,"ancestor_record_offset":32892,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1021,"record_offset":33044,"parent_record_offset":33020,"ancestor_record_offset":32892,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1021,"record_offset":32948,"parent_record_offset":32924,"ancestor_record_offset":32892,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_library::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 171
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFD4F
// 首条位置: {"module":1021,"inlinee":64847}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1021,"record_offset":33796,"parent_record_offset":33772,"ancestor_record_offset":33772,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1021,"record_offset":33820,"parent_record_offset":33796,"ancestor_record_offset":33772,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_default_library
// 记录形式: compiled_procedure; instances=1
// 源码行: 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":288,"record_offset":4356,"rva":7129808,"va":5375838928}
// 编译实例 RVAs: 0x6ccad0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::raw_vec::RawVec<std::ffi::os_str::OsString>>::grow_one
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t40778909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375838928,"raw_hex":"e89d3c6e02","size":5,"soft_fail":false,"target_va":5416618000,"va":5375839086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::wtf8::Wtf8>::to_owned
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8954192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375838928,"raw_hex":"e850a18800","size":5,"soft_fail":false,"target_va":5384793264,"va":5375839067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-160471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375838928,"raw_hex":"e8298dfdff","size":5,"soft_fail":false,"target_va":5375678704,"va":5375839170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140089010
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6569221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375838928,"raw_hex":"e8fbc29bff","size":5,"soft_fail":false,"target_va":5369270288,"va":5375839504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9657325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375838928,"raw_hex":"e8ed5b9300","size":5,"soft_fail":false,"target_va":5385496464,"va":5375839134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<libloading::error::Error> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375838928,"raw_hex":"e888ecffff","size":5,"soft_fail":false,"target_va":5375834064,"va":5375839043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":288,"record_offset":4496,"parent_record_offset":4356,"ancestor_record_offset":4356,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":288,"record_offset":5144,"parent_record_offset":4356,"ancestor_record_offset":4356,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":288,"record_offset":4976,"parent_record_offset":4356,"ancestor_record_offset":4356,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":288,"record_offset":4524,"parent_record_offset":4356,"ancestor_record_offset":4356,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":288,"record_offset":4920,"parent_record_offset":4356,"ancestor_record_offset":4356,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":288,"record_offset":4796,"parent_record_offset":4772,"ancestor_record_offset":4356,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":288,"record_offset":4820,"parent_record_offset":4796,"ancestor_record_offset":4356,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":288,"record_offset":4844,"parent_record_offset":4820,"ancestor_record_offset":4356,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":288,"record_offset":4772,"parent_record_offset":4716,"ancestor_record_offset":4356,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":288,"record_offset":5248,"parent_record_offset":5216,"ancestor_record_offset":4356,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":288,"record_offset":4744,"parent_record_offset":4716,"ancestor_record_offset":4356,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":288,"record_offset":4716,"parent_record_offset":4692,"ancestor_record_offset":4356,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":288,"record_offset":5296,"parent_record_offset":5272,"ancestor_record_offset":4356,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":288,"record_offset":5272,"parent_record_offset":5248,"ancestor_record_offset":4356,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":288,"record_offset":4692,"parent_record_offset":4668,"ancestor_record_offset":4356,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":288,"record_offset":5216,"parent_record_offset":5172,"ancestor_record_offset":4356,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":288,"record_offset":5172,"parent_record_offset":5144,"ancestor_record_offset":4356,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":288,"record_offset":5740,"parent_record_offset":5712,"ancestor_record_offset":4356,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":288,"record_offset":5048,"parent_record_offset":5024,"ancestor_record_offset":4356,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":288,"record_offset":6280,"parent_record_offset":6252,"ancestor_record_offset":4356,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":288,"record_offset":4548,"parent_record_offset":4524,"ancestor_record_offset":4356,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":288,"record_offset":4944,"parent_record_offset":4920,"ancestor_record_offset":4356,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":288,"record_offset":5336,"parent_record_offset":5172,"ancestor_record_offset":4356,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":288,"record_offset":5076,"parent_record_offset":5024,"ancestor_record_offset":4356,"inlinee":5109,"source_line":986,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$4::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":288,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":4356,"inlinee":5103,"source_line":623,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$4::from::impl$0::spec_to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":288,"record_offset":5024,"parent_record_offset":5000,"ancestor_record_offset":4356,"inlinee":5105,"source_line":630,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":288,"record_offset":5100,"parent_record_offset":5076,"ancestor_record_offset":4356,"inlinee":5111,"source_line":265,"archive":"_pdb/C/modules/00288.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::active_api
// 记录形式: inline_instance, inlinee_declaration; instances=47
// 源码行: 198
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4B0C
// 首条位置: {"module":1005,"inlinee":19212}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=82
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1005,"record_offset":408,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=82
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1005,"record_offset":488,"parent_record_offset":468,"ancestor_record_offset":372,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=82
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1005,"record_offset":508,"parent_record_offset":488,"ancestor_record_offset":372,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=82
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1005,"record_offset":448,"parent_record_offset":428,"ancestor_record_offset":372,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=82
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1005,"record_offset":428,"parent_record_offset":408,"ancestor_record_offset":372,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=82
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1005,"record_offset":468,"parent_record_offset":448,"ancestor_record_offset":372,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<i32 (*)()>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":30848,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":31044,"parent_record_offset":30848,"ancestor_record_offset":30848,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":30980,"parent_record_offset":30848,"ancestor_record_offset":30848,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":31380,"parent_record_offset":31352,"ancestor_record_offset":30848,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":31352,"parent_record_offset":31324,"ancestor_record_offset":30848,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":31712,"parent_record_offset":31688,"ancestor_record_offset":30848,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":31324,"parent_record_offset":31296,"ancestor_record_offset":30848,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":31688,"parent_record_offset":31664,"ancestor_record_offset":30848,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":31528,"parent_record_offset":31500,"ancestor_record_offset":30848,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":31136,"parent_record_offset":31076,"ancestor_record_offset":30848,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":31008,"parent_record_offset":30980,"ancestor_record_offset":30848,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":31640,"parent_record_offset":31616,"ancestor_record_offset":30848,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":31664,"parent_record_offset":31640,"ancestor_record_offset":30848,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":31296,"parent_record_offset":31256,"ancestor_record_offset":30848,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":31500,"parent_record_offset":31448,"ancestor_record_offset":30848,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":31208,"parent_record_offset":31076,"ancestor_record_offset":30848,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":31424,"parent_record_offset":31256,"ancestor_record_offset":30848,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":31232,"parent_record_offset":31208,"ancestor_record_offset":30848,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":31256,"parent_record_offset":31232,"ancestor_record_offset":30848,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":31448,"parent_record_offset":31424,"ancestor_record_offset":30848,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":31784,"parent_record_offset":31448,"ancestor_record_offset":30848,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":31756,"parent_record_offset":31448,"ancestor_record_offset":30848,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":31560,"parent_record_offset":31448,"ancestor_record_offset":30848,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":31076,"parent_record_offset":31044,"ancestor_record_offset":30848,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<i32 (*)(i32)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":22216,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":22412,"parent_record_offset":22216,"ancestor_record_offset":22216,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":22348,"parent_record_offset":22216,"ancestor_record_offset":22216,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":22748,"parent_record_offset":22720,"ancestor_record_offset":22216,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":22720,"parent_record_offset":22692,"ancestor_record_offset":22216,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":23080,"parent_record_offset":23056,"ancestor_record_offset":22216,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":22692,"parent_record_offset":22664,"ancestor_record_offset":22216,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":23056,"parent_record_offset":23032,"ancestor_record_offset":22216,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":22896,"parent_record_offset":22868,"ancestor_record_offset":22216,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":22504,"parent_record_offset":22444,"ancestor_record_offset":22216,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":22376,"parent_record_offset":22348,"ancestor_record_offset":22216,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":23008,"parent_record_offset":22984,"ancestor_record_offset":22216,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":23032,"parent_record_offset":23008,"ancestor_record_offset":22216,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":22664,"parent_record_offset":22624,"ancestor_record_offset":22216,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":22868,"parent_record_offset":22816,"ancestor_record_offset":22216,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":22576,"parent_record_offset":22444,"ancestor_record_offset":22216,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":22792,"parent_record_offset":22624,"ancestor_record_offset":22216,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":22600,"parent_record_offset":22576,"ancestor_record_offset":22216,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":22624,"parent_record_offset":22600,"ancestor_record_offset":22216,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":22816,"parent_record_offset":22792,"ancestor_record_offset":22216,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":23152,"parent_record_offset":22816,"ancestor_record_offset":22216,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":23124,"parent_record_offset":22816,"ancestor_record_offset":22216,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":22928,"parent_record_offset":22816,"ancestor_record_offset":22216,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":22444,"parent_record_offset":22412,"ancestor_record_offset":22216,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<i32 (*)(i32,i32,ptr_mut$<usdl_sys::video::uSDL_DisplayMode>)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":3656,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":3900,"parent_record_offset":3656,"ancestor_record_offset":3656,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":3836,"parent_record_offset":3656,"ancestor_record_offset":3656,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":4236,"parent_record_offset":4208,"ancestor_record_offset":3656,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":4208,"parent_record_offset":4180,"ancestor_record_offset":3656,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":4568,"parent_record_offset":4544,"ancestor_record_offset":3656,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":4180,"parent_record_offset":4152,"ancestor_record_offset":3656,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":4544,"parent_record_offset":4520,"ancestor_record_offset":3656,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":4384,"parent_record_offset":4356,"ancestor_record_offset":3656,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":3992,"parent_record_offset":3932,"ancestor_record_offset":3656,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":3864,"parent_record_offset":3836,"ancestor_record_offset":3656,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":4496,"parent_record_offset":4472,"ancestor_record_offset":3656,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":4520,"parent_record_offset":4496,"ancestor_record_offset":3656,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":4152,"parent_record_offset":4112,"ancestor_record_offset":3656,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":4356,"parent_record_offset":4304,"ancestor_record_offset":3656,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":4064,"parent_record_offset":3932,"ancestor_record_offset":3656,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":4280,"parent_record_offset":4112,"ancestor_record_offset":3656,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":4088,"parent_record_offset":4064,"ancestor_record_offset":3656,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":4112,"parent_record_offset":4088,"ancestor_record_offset":3656,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":4304,"parent_record_offset":4280,"ancestor_record_offset":3656,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":4640,"parent_record_offset":4304,"ancestor_record_offset":3656,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":4612,"parent_record_offset":4304,"ancestor_record_offset":3656,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":4416,"parent_record_offset":4304,"ancestor_record_offset":3656,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":3932,"parent_record_offset":3900,"ancestor_record_offset":3656,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<i32 (*)(i32,ptr_mut$<usdl_sys::rect::uSDL_Rect>)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":20064,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":20296,"parent_record_offset":20064,"ancestor_record_offset":20064,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":20232,"parent_record_offset":20064,"ancestor_record_offset":20064,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":20632,"parent_record_offset":20604,"ancestor_record_offset":20064,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":20604,"parent_record_offset":20576,"ancestor_record_offset":20064,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":20964,"parent_record_offset":20940,"ancestor_record_offset":20064,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":20576,"parent_record_offset":20548,"ancestor_record_offset":20064,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":20940,"parent_record_offset":20916,"ancestor_record_offset":20064,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":20780,"parent_record_offset":20752,"ancestor_record_offset":20064,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":20388,"parent_record_offset":20328,"ancestor_record_offset":20064,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":20260,"parent_record_offset":20232,"ancestor_record_offset":20064,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":20892,"parent_record_offset":20868,"ancestor_record_offset":20064,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":20916,"parent_record_offset":20892,"ancestor_record_offset":20064,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":20548,"parent_record_offset":20508,"ancestor_record_offset":20064,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":20752,"parent_record_offset":20700,"ancestor_record_offset":20064,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":20460,"parent_record_offset":20328,"ancestor_record_offset":20064,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":20676,"parent_record_offset":20508,"ancestor_record_offset":20064,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":20484,"parent_record_offset":20460,"ancestor_record_offset":20064,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":20508,"parent_record_offset":20484,"ancestor_record_offset":20064,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":20700,"parent_record_offset":20676,"ancestor_record_offset":20064,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":21036,"parent_record_offset":20700,"ancestor_record_offset":20064,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":21008,"parent_record_offset":20700,"ancestor_record_offset":20064,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":20812,"parent_record_offset":20700,"ancestor_record_offset":20064,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":20328,"parent_record_offset":20296,"ancestor_record_offset":20064,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<i32 (*)(i32,ptr_mut$<usdl_sys::video::uSDL_DisplayMode>)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":10248,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":10488,"parent_record_offset":10248,"ancestor_record_offset":10248,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":10424,"parent_record_offset":10248,"ancestor_record_offset":10248,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":10824,"parent_record_offset":10796,"ancestor_record_offset":10248,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":10796,"parent_record_offset":10768,"ancestor_record_offset":10248,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":11156,"parent_record_offset":11132,"ancestor_record_offset":10248,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":10768,"parent_record_offset":10740,"ancestor_record_offset":10248,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":11132,"parent_record_offset":11108,"ancestor_record_offset":10248,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":10972,"parent_record_offset":10944,"ancestor_record_offset":10248,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":10580,"parent_record_offset":10520,"ancestor_record_offset":10248,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":10452,"parent_record_offset":10424,"ancestor_record_offset":10248,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":11084,"parent_record_offset":11060,"ancestor_record_offset":10248,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":11108,"parent_record_offset":11084,"ancestor_record_offset":10248,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":10740,"parent_record_offset":10700,"ancestor_record_offset":10248,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":10944,"parent_record_offset":10892,"ancestor_record_offset":10248,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":10652,"parent_record_offset":10520,"ancestor_record_offset":10248,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":10868,"parent_record_offset":10700,"ancestor_record_offset":10248,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":10676,"parent_record_offset":10652,"ancestor_record_offset":10248,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":10700,"parent_record_offset":10676,"ancestor_record_offset":10248,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":10892,"parent_record_offset":10868,"ancestor_record_offset":10248,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":11228,"parent_record_offset":10892,"ancestor_record_offset":10248,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":11200,"parent_record_offset":10892,"ancestor_record_offset":10248,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":11004,"parent_record_offset":10892,"ancestor_record_offset":10248,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":10520,"parent_record_offset":10488,"ancestor_record_offset":10248,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<i32 (*)(ptr_mut$<usdl_sys::event::uSDL_Event>)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":1480,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":1712,"parent_record_offset":1480,"ancestor_record_offset":1480,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":1648,"parent_record_offset":1480,"ancestor_record_offset":1480,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":2048,"parent_record_offset":2020,"ancestor_record_offset":1480,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":2020,"parent_record_offset":1992,"ancestor_record_offset":1480,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":2380,"parent_record_offset":2356,"ancestor_record_offset":1480,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":1992,"parent_record_offset":1964,"ancestor_record_offset":1480,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":2356,"parent_record_offset":2332,"ancestor_record_offset":1480,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":2196,"parent_record_offset":2168,"ancestor_record_offset":1480,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":1804,"parent_record_offset":1744,"ancestor_record_offset":1480,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":1676,"parent_record_offset":1648,"ancestor_record_offset":1480,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":2308,"parent_record_offset":2284,"ancestor_record_offset":1480,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":2332,"parent_record_offset":2308,"ancestor_record_offset":1480,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":1964,"parent_record_offset":1924,"ancestor_record_offset":1480,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":2168,"parent_record_offset":2116,"ancestor_record_offset":1480,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":1876,"parent_record_offset":1744,"ancestor_record_offset":1480,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":2092,"parent_record_offset":1924,"ancestor_record_offset":1480,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":1900,"parent_record_offset":1876,"ancestor_record_offset":1480,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":1924,"parent_record_offset":1900,"ancestor_record_offset":1480,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":1480,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":2452,"parent_record_offset":2116,"ancestor_record_offset":1480,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":2424,"parent_record_offset":2116,"ancestor_record_offset":1480,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":2228,"parent_record_offset":2116,"ancestor_record_offset":1480,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":1744,"parent_record_offset":1712,"ancestor_record_offset":1480,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<i32 (*)(ptr_mut$<usdl_sys::event::uSDL_Event>,i32)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":23268,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":23504,"parent_record_offset":23268,"ancestor_record_offset":23268,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":23440,"parent_record_offset":23268,"ancestor_record_offset":23268,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":23840,"parent_record_offset":23812,"ancestor_record_offset":23268,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":23812,"parent_record_offset":23784,"ancestor_record_offset":23268,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":24172,"parent_record_offset":24148,"ancestor_record_offset":23268,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":23784,"parent_record_offset":23756,"ancestor_record_offset":23268,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":24148,"parent_record_offset":24124,"ancestor_record_offset":23268,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":23988,"parent_record_offset":23960,"ancestor_record_offset":23268,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":23596,"parent_record_offset":23536,"ancestor_record_offset":23268,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":23468,"parent_record_offset":23440,"ancestor_record_offset":23268,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":24100,"parent_record_offset":24076,"ancestor_record_offset":23268,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":24124,"parent_record_offset":24100,"ancestor_record_offset":23268,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":23756,"parent_record_offset":23716,"ancestor_record_offset":23268,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":23960,"parent_record_offset":23908,"ancestor_record_offset":23268,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":23668,"parent_record_offset":23536,"ancestor_record_offset":23268,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":23884,"parent_record_offset":23716,"ancestor_record_offset":23268,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":23692,"parent_record_offset":23668,"ancestor_record_offset":23268,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":23716,"parent_record_offset":23692,"ancestor_record_offset":23268,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":23908,"parent_record_offset":23884,"ancestor_record_offset":23268,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":24244,"parent_record_offset":23908,"ancestor_record_offset":23268,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":24216,"parent_record_offset":23908,"ancestor_record_offset":23268,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":24020,"parent_record_offset":23908,"ancestor_record_offset":23268,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":23536,"parent_record_offset":23504,"ancestor_record_offset":23268,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":9160,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":9392,"parent_record_offset":9160,"ancestor_record_offset":9160,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":9328,"parent_record_offset":9160,"ancestor_record_offset":9160,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":9728,"parent_record_offset":9700,"ancestor_record_offset":9160,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":9700,"parent_record_offset":9672,"ancestor_record_offset":9160,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":10060,"parent_record_offset":10036,"ancestor_record_offset":9160,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":9672,"parent_record_offset":9644,"ancestor_record_offset":9160,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":10036,"parent_record_offset":10012,"ancestor_record_offset":9160,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":9876,"parent_record_offset":9848,"ancestor_record_offset":9160,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":9484,"parent_record_offset":9424,"ancestor_record_offset":9160,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":9356,"parent_record_offset":9328,"ancestor_record_offset":9160,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":9988,"parent_record_offset":9964,"ancestor_record_offset":9160,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":10012,"parent_record_offset":9988,"ancestor_record_offset":9160,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":9644,"parent_record_offset":9604,"ancestor_record_offset":9160,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":9848,"parent_record_offset":9796,"ancestor_record_offset":9160,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":9556,"parent_record_offset":9424,"ancestor_record_offset":9160,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":9772,"parent_record_offset":9604,"ancestor_record_offset":9160,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":9580,"parent_record_offset":9556,"ancestor_record_offset":9160,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":9604,"parent_record_offset":9580,"ancestor_record_offset":9160,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":9796,"parent_record_offset":9772,"ancestor_record_offset":9160,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":10132,"parent_record_offset":9796,"ancestor_record_offset":9160,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":10104,"parent_record_offset":9796,"ancestor_record_offset":9160,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":9908,"parent_record_offset":9796,"ancestor_record_offset":9160,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":9424,"parent_record_offset":9392,"ancestor_record_offset":9160,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,ptr_const$<usdl_sys::video::uSDL_DisplayMode>)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":16720,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":17000,"parent_record_offset":16720,"ancestor_record_offset":16720,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":16936,"parent_record_offset":16720,"ancestor_record_offset":16720,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":17336,"parent_record_offset":17308,"ancestor_record_offset":16720,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":17308,"parent_record_offset":17280,"ancestor_record_offset":16720,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":17668,"parent_record_offset":17644,"ancestor_record_offset":16720,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":17280,"parent_record_offset":17252,"ancestor_record_offset":16720,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":17644,"parent_record_offset":17620,"ancestor_record_offset":16720,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":17484,"parent_record_offset":17456,"ancestor_record_offset":16720,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":17092,"parent_record_offset":17032,"ancestor_record_offset":16720,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":16964,"parent_record_offset":16936,"ancestor_record_offset":16720,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":17596,"parent_record_offset":17572,"ancestor_record_offset":16720,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":17620,"parent_record_offset":17596,"ancestor_record_offset":16720,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":17252,"parent_record_offset":17212,"ancestor_record_offset":16720,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":17456,"parent_record_offset":17404,"ancestor_record_offset":16720,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":17164,"parent_record_offset":17032,"ancestor_record_offset":16720,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":17380,"parent_record_offset":17212,"ancestor_record_offset":16720,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":17188,"parent_record_offset":17164,"ancestor_record_offset":16720,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":17212,"parent_record_offset":17188,"ancestor_record_offset":16720,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":17404,"parent_record_offset":17380,"ancestor_record_offset":16720,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":17740,"parent_record_offset":17404,"ancestor_record_offset":16720,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":17712,"parent_record_offset":17404,"ancestor_record_offset":16720,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":17516,"parent_record_offset":17404,"ancestor_record_offset":16720,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":17032,"parent_record_offset":17000,"ancestor_record_offset":16720,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,ptr_mut$<usdl_sys::syswm::uSDL_SysWMinfo>)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":4756,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":5032,"parent_record_offset":4756,"ancestor_record_offset":4756,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":4968,"parent_record_offset":4756,"ancestor_record_offset":4756,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":5368,"parent_record_offset":5340,"ancestor_record_offset":4756,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":5340,"parent_record_offset":5312,"ancestor_record_offset":4756,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":5700,"parent_record_offset":5676,"ancestor_record_offset":4756,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":5312,"parent_record_offset":5284,"ancestor_record_offset":4756,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":5676,"parent_record_offset":5652,"ancestor_record_offset":4756,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":5516,"parent_record_offset":5488,"ancestor_record_offset":4756,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":5124,"parent_record_offset":5064,"ancestor_record_offset":4756,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":4996,"parent_record_offset":4968,"ancestor_record_offset":4756,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":5628,"parent_record_offset":5604,"ancestor_record_offset":4756,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":5652,"parent_record_offset":5628,"ancestor_record_offset":4756,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":5284,"parent_record_offset":5244,"ancestor_record_offset":4756,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":5488,"parent_record_offset":5436,"ancestor_record_offset":4756,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":5196,"parent_record_offset":5064,"ancestor_record_offset":4756,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":5412,"parent_record_offset":5244,"ancestor_record_offset":4756,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":5220,"parent_record_offset":5196,"ancestor_record_offset":4756,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":5244,"parent_record_offset":5220,"ancestor_record_offset":4756,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":5436,"parent_record_offset":5412,"ancestor_record_offset":4756,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":5772,"parent_record_offset":5436,"ancestor_record_offset":4756,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":5744,"parent_record_offset":5436,"ancestor_record_offset":4756,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":5548,"parent_record_offset":5436,"ancestor_record_offset":4756,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":5064,"parent_record_offset":5032,"ancestor_record_offset":4756,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,usdl_sys::video::uSDL_FullscreenType)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":6940,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":7208,"parent_record_offset":6940,"ancestor_record_offset":6940,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":7144,"parent_record_offset":6940,"ancestor_record_offset":6940,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":7544,"parent_record_offset":7516,"ancestor_record_offset":6940,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":7516,"parent_record_offset":7488,"ancestor_record_offset":6940,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":7876,"parent_record_offset":7852,"ancestor_record_offset":6940,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":7488,"parent_record_offset":7460,"ancestor_record_offset":6940,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":7852,"parent_record_offset":7828,"ancestor_record_offset":6940,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":7692,"parent_record_offset":7664,"ancestor_record_offset":6940,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":7300,"parent_record_offset":7240,"ancestor_record_offset":6940,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":7172,"parent_record_offset":7144,"ancestor_record_offset":6940,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":7804,"parent_record_offset":7780,"ancestor_record_offset":6940,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":7828,"parent_record_offset":7804,"ancestor_record_offset":6940,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":7460,"parent_record_offset":7420,"ancestor_record_offset":6940,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":7664,"parent_record_offset":7612,"ancestor_record_offset":6940,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":7372,"parent_record_offset":7240,"ancestor_record_offset":6940,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":7588,"parent_record_offset":7420,"ancestor_record_offset":6940,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":7396,"parent_record_offset":7372,"ancestor_record_offset":6940,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":7420,"parent_record_offset":7396,"ancestor_record_offset":6940,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":7612,"parent_record_offset":7588,"ancestor_record_offset":6940,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":7948,"parent_record_offset":7612,"ancestor_record_offset":6940,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":7920,"parent_record_offset":7612,"ancestor_record_offset":6940,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":7724,"parent_record_offset":7612,"ancestor_record_offset":6940,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":7240,"parent_record_offset":7208,"ancestor_record_offset":6940,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<i32 (*)(u32)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":5888,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":6084,"parent_record_offset":5888,"ancestor_record_offset":5888,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":6020,"parent_record_offset":5888,"ancestor_record_offset":5888,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":6420,"parent_record_offset":6392,"ancestor_record_offset":5888,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":6392,"parent_record_offset":6364,"ancestor_record_offset":5888,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":6752,"parent_record_offset":6728,"ancestor_record_offset":5888,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":6364,"parent_record_offset":6336,"ancestor_record_offset":5888,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":6728,"parent_record_offset":6704,"ancestor_record_offset":5888,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":6568,"parent_record_offset":6540,"ancestor_record_offset":5888,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":6176,"parent_record_offset":6116,"ancestor_record_offset":5888,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":6048,"parent_record_offset":6020,"ancestor_record_offset":5888,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":6680,"parent_record_offset":6656,"ancestor_record_offset":5888,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":6704,"parent_record_offset":6680,"ancestor_record_offset":5888,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":6336,"parent_record_offset":6296,"ancestor_record_offset":5888,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":6540,"parent_record_offset":6488,"ancestor_record_offset":5888,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":6248,"parent_record_offset":6116,"ancestor_record_offset":5888,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":6464,"parent_record_offset":6296,"ancestor_record_offset":5888,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":6272,"parent_record_offset":6248,"ancestor_record_offset":5888,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":6296,"parent_record_offset":6272,"ancestor_record_offset":5888,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":6488,"parent_record_offset":6464,"ancestor_record_offset":5888,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":6824,"parent_record_offset":6488,"ancestor_record_offset":5888,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":6796,"parent_record_offset":6488,"ancestor_record_offset":5888,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":6600,"parent_record_offset":6488,"ancestor_record_offset":5888,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":6116,"parent_record_offset":6084,"ancestor_record_offset":5888,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<ptr_const$<i8> (*)()>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":13488,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":13692,"parent_record_offset":13488,"ancestor_record_offset":13488,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":13628,"parent_record_offset":13488,"ancestor_record_offset":13488,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":14028,"parent_record_offset":14000,"ancestor_record_offset":13488,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":14000,"parent_record_offset":13972,"ancestor_record_offset":13488,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":14360,"parent_record_offset":14336,"ancestor_record_offset":13488,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":13972,"parent_record_offset":13944,"ancestor_record_offset":13488,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":14336,"parent_record_offset":14312,"ancestor_record_offset":13488,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":14176,"parent_record_offset":14148,"ancestor_record_offset":13488,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":13784,"parent_record_offset":13724,"ancestor_record_offset":13488,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":13656,"parent_record_offset":13628,"ancestor_record_offset":13488,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":14288,"parent_record_offset":14264,"ancestor_record_offset":13488,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":14312,"parent_record_offset":14288,"ancestor_record_offset":13488,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":13944,"parent_record_offset":13904,"ancestor_record_offset":13488,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":14148,"parent_record_offset":14096,"ancestor_record_offset":13488,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":13856,"parent_record_offset":13724,"ancestor_record_offset":13488,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":14072,"parent_record_offset":13904,"ancestor_record_offset":13488,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":13880,"parent_record_offset":13856,"ancestor_record_offset":13488,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":13904,"parent_record_offset":13880,"ancestor_record_offset":13488,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":14096,"parent_record_offset":14072,"ancestor_record_offset":13488,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":14432,"parent_record_offset":14096,"ancestor_record_offset":13488,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":14404,"parent_record_offset":14096,"ancestor_record_offset":13488,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":14208,"parent_record_offset":14096,"ancestor_record_offset":13488,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":13724,"parent_record_offset":13692,"ancestor_record_offset":13488,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<ptr_const$<i8> (*)(i32)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":21152,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":21360,"parent_record_offset":21152,"ancestor_record_offset":21152,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":21296,"parent_record_offset":21152,"ancestor_record_offset":21152,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":21696,"parent_record_offset":21668,"ancestor_record_offset":21152,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":21668,"parent_record_offset":21640,"ancestor_record_offset":21152,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":22028,"parent_record_offset":22004,"ancestor_record_offset":21152,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":21640,"parent_record_offset":21612,"ancestor_record_offset":21152,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":22004,"parent_record_offset":21980,"ancestor_record_offset":21152,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":21844,"parent_record_offset":21816,"ancestor_record_offset":21152,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":21452,"parent_record_offset":21392,"ancestor_record_offset":21152,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":21324,"parent_record_offset":21296,"ancestor_record_offset":21152,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":21956,"parent_record_offset":21932,"ancestor_record_offset":21152,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":21980,"parent_record_offset":21956,"ancestor_record_offset":21152,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":21612,"parent_record_offset":21572,"ancestor_record_offset":21152,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":21816,"parent_record_offset":21764,"ancestor_record_offset":21152,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":21524,"parent_record_offset":21392,"ancestor_record_offset":21152,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":21740,"parent_record_offset":21572,"ancestor_record_offset":21152,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":21548,"parent_record_offset":21524,"ancestor_record_offset":21152,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":21572,"parent_record_offset":21548,"ancestor_record_offset":21152,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":21764,"parent_record_offset":21740,"ancestor_record_offset":21152,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":22100,"parent_record_offset":21764,"ancestor_record_offset":21152,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":22072,"parent_record_offset":21764,"ancestor_record_offset":21152,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":21876,"parent_record_offset":21764,"ancestor_record_offset":21152,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":21392,"parent_record_offset":21360,"ancestor_record_offset":21152,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<ptr_mut$<usdl_sys::mouse::uSDL_Cursor> (*)(usdl_sys::mouse::uSDL_SystemCursor)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":18944,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":19208,"parent_record_offset":18944,"ancestor_record_offset":18944,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":19144,"parent_record_offset":18944,"ancestor_record_offset":18944,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":19544,"parent_record_offset":19516,"ancestor_record_offset":18944,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":19516,"parent_record_offset":19488,"ancestor_record_offset":18944,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":19876,"parent_record_offset":19852,"ancestor_record_offset":18944,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":19488,"parent_record_offset":19460,"ancestor_record_offset":18944,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":19852,"parent_record_offset":19828,"ancestor_record_offset":18944,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":19692,"parent_record_offset":19664,"ancestor_record_offset":18944,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":19300,"parent_record_offset":19240,"ancestor_record_offset":18944,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":19172,"parent_record_offset":19144,"ancestor_record_offset":18944,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":19804,"parent_record_offset":19780,"ancestor_record_offset":18944,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":19828,"parent_record_offset":19804,"ancestor_record_offset":18944,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":19460,"parent_record_offset":19420,"ancestor_record_offset":18944,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":19664,"parent_record_offset":19612,"ancestor_record_offset":18944,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":19372,"parent_record_offset":19240,"ancestor_record_offset":18944,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":19588,"parent_record_offset":19420,"ancestor_record_offset":18944,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":19396,"parent_record_offset":19372,"ancestor_record_offset":18944,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":19420,"parent_record_offset":19396,"ancestor_record_offset":18944,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":19612,"parent_record_offset":19588,"ancestor_record_offset":18944,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":19948,"parent_record_offset":19612,"ancestor_record_offset":18944,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":19920,"parent_record_offset":19612,"ancestor_record_offset":18944,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":19724,"parent_record_offset":19612,"ancestor_record_offset":18944,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":19240,"parent_record_offset":19208,"ancestor_record_offset":18944,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<ptr_mut$<usdl_sys::video::uSDL_Window> (*)(ptr_const$<i8>,i32,i32,i32,i32,u32)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":15600,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":15864,"parent_record_offset":15600,"ancestor_record_offset":15600,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":15800,"parent_record_offset":15600,"ancestor_record_offset":15600,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":16200,"parent_record_offset":16172,"ancestor_record_offset":15600,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":16172,"parent_record_offset":16144,"ancestor_record_offset":15600,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":16532,"parent_record_offset":16508,"ancestor_record_offset":15600,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":16144,"parent_record_offset":16116,"ancestor_record_offset":15600,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":16508,"parent_record_offset":16484,"ancestor_record_offset":15600,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":16348,"parent_record_offset":16320,"ancestor_record_offset":15600,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":15956,"parent_record_offset":15896,"ancestor_record_offset":15600,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":15828,"parent_record_offset":15800,"ancestor_record_offset":15600,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":16460,"parent_record_offset":16436,"ancestor_record_offset":15600,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":16484,"parent_record_offset":16460,"ancestor_record_offset":15600,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":16116,"parent_record_offset":16076,"ancestor_record_offset":15600,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":16320,"parent_record_offset":16268,"ancestor_record_offset":15600,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":16028,"parent_record_offset":15896,"ancestor_record_offset":15600,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":16244,"parent_record_offset":16076,"ancestor_record_offset":15600,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":16052,"parent_record_offset":16028,"ancestor_record_offset":15600,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":16076,"parent_record_offset":16052,"ancestor_record_offset":15600,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":16268,"parent_record_offset":16244,"ancestor_record_offset":15600,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":16604,"parent_record_offset":16268,"ancestor_record_offset":15600,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":16576,"parent_record_offset":16268,"ancestor_record_offset":15600,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":16380,"parent_record_offset":16268,"ancestor_record_offset":15600,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":15896,"parent_record_offset":15864,"ancestor_record_offset":15600,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<u16 (*)()>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":12436,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":12632,"parent_record_offset":12436,"ancestor_record_offset":12436,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":12568,"parent_record_offset":12436,"ancestor_record_offset":12436,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":12968,"parent_record_offset":12940,"ancestor_record_offset":12436,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":12940,"parent_record_offset":12912,"ancestor_record_offset":12436,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":13300,"parent_record_offset":13276,"ancestor_record_offset":12436,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":12912,"parent_record_offset":12884,"ancestor_record_offset":12436,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":13276,"parent_record_offset":13252,"ancestor_record_offset":12436,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":13116,"parent_record_offset":13088,"ancestor_record_offset":12436,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":12724,"parent_record_offset":12664,"ancestor_record_offset":12436,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":12596,"parent_record_offset":12568,"ancestor_record_offset":12436,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":13228,"parent_record_offset":13204,"ancestor_record_offset":12436,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":13252,"parent_record_offset":13228,"ancestor_record_offset":12436,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":12884,"parent_record_offset":12844,"ancestor_record_offset":12436,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":13088,"parent_record_offset":13036,"ancestor_record_offset":12436,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":12796,"parent_record_offset":12664,"ancestor_record_offset":12436,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":13012,"parent_record_offset":12844,"ancestor_record_offset":12436,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":12820,"parent_record_offset":12796,"ancestor_record_offset":12436,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":12844,"parent_record_offset":12820,"ancestor_record_offset":12436,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":13036,"parent_record_offset":13012,"ancestor_record_offset":12436,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":13372,"parent_record_offset":13036,"ancestor_record_offset":12436,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":13344,"parent_record_offset":13036,"ancestor_record_offset":12436,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":13148,"parent_record_offset":13036,"ancestor_record_offset":12436,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":12664,"parent_record_offset":12632,"ancestor_record_offset":12436,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<u32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":2568,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":2800,"parent_record_offset":2568,"ancestor_record_offset":2568,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":2736,"parent_record_offset":2568,"ancestor_record_offset":2568,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":3136,"parent_record_offset":3108,"ancestor_record_offset":2568,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":3108,"parent_record_offset":3080,"ancestor_record_offset":2568,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":2568,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":3080,"parent_record_offset":3052,"ancestor_record_offset":2568,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":3444,"parent_record_offset":3420,"ancestor_record_offset":2568,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":3284,"parent_record_offset":3256,"ancestor_record_offset":2568,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":2892,"parent_record_offset":2832,"ancestor_record_offset":2568,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":2764,"parent_record_offset":2736,"ancestor_record_offset":2568,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":3396,"parent_record_offset":3372,"ancestor_record_offset":2568,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":3420,"parent_record_offset":3396,"ancestor_record_offset":2568,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":3052,"parent_record_offset":3012,"ancestor_record_offset":2568,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":3256,"parent_record_offset":3204,"ancestor_record_offset":2568,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":2964,"parent_record_offset":2832,"ancestor_record_offset":2568,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":3180,"parent_record_offset":3012,"ancestor_record_offset":2568,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":2988,"parent_record_offset":2964,"ancestor_record_offset":2568,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":3012,"parent_record_offset":2988,"ancestor_record_offset":2568,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":3204,"parent_record_offset":3180,"ancestor_record_offset":2568,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":3540,"parent_record_offset":3204,"ancestor_record_offset":2568,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":3512,"parent_record_offset":3204,"ancestor_record_offset":2568,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":3316,"parent_record_offset":3204,"ancestor_record_offset":2568,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":2832,"parent_record_offset":2800,"ancestor_record_offset":2568,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<u8 (*)(u32,i32)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":26504,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":26704,"parent_record_offset":26504,"ancestor_record_offset":26504,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":26640,"parent_record_offset":26504,"ancestor_record_offset":26504,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":27040,"parent_record_offset":27012,"ancestor_record_offset":26504,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":27012,"parent_record_offset":26984,"ancestor_record_offset":26504,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":27372,"parent_record_offset":27348,"ancestor_record_offset":26504,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":26984,"parent_record_offset":26956,"ancestor_record_offset":26504,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":27348,"parent_record_offset":27324,"ancestor_record_offset":26504,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":27188,"parent_record_offset":27160,"ancestor_record_offset":26504,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":26796,"parent_record_offset":26736,"ancestor_record_offset":26504,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":26668,"parent_record_offset":26640,"ancestor_record_offset":26504,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":27300,"parent_record_offset":27276,"ancestor_record_offset":26504,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":27324,"parent_record_offset":27300,"ancestor_record_offset":26504,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":26956,"parent_record_offset":26916,"ancestor_record_offset":26504,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":27160,"parent_record_offset":27108,"ancestor_record_offset":26504,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":26868,"parent_record_offset":26736,"ancestor_record_offset":26504,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":27084,"parent_record_offset":26916,"ancestor_record_offset":26504,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":26892,"parent_record_offset":26868,"ancestor_record_offset":26504,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":26916,"parent_record_offset":26892,"ancestor_record_offset":26504,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":27108,"parent_record_offset":27084,"ancestor_record_offset":26504,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":27444,"parent_record_offset":27108,"ancestor_record_offset":26504,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":27416,"parent_record_offset":27108,"ancestor_record_offset":26504,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":27220,"parent_record_offset":27108,"ancestor_record_offset":26504,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":26736,"parent_record_offset":26704,"ancestor_record_offset":26504,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<void (*)()>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":14548,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":14744,"parent_record_offset":14548,"ancestor_record_offset":14548,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":14680,"parent_record_offset":14548,"ancestor_record_offset":14548,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":15080,"parent_record_offset":15052,"ancestor_record_offset":14548,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":15052,"parent_record_offset":15024,"ancestor_record_offset":14548,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":15412,"parent_record_offset":15388,"ancestor_record_offset":14548,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":15024,"parent_record_offset":14996,"ancestor_record_offset":14548,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":15388,"parent_record_offset":15364,"ancestor_record_offset":14548,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":15228,"parent_record_offset":15200,"ancestor_record_offset":14548,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":14836,"parent_record_offset":14776,"ancestor_record_offset":14548,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":14708,"parent_record_offset":14680,"ancestor_record_offset":14548,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":15340,"parent_record_offset":15316,"ancestor_record_offset":14548,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":15364,"parent_record_offset":15340,"ancestor_record_offset":14548,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":14996,"parent_record_offset":14956,"ancestor_record_offset":14548,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":15200,"parent_record_offset":15148,"ancestor_record_offset":14548,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":14908,"parent_record_offset":14776,"ancestor_record_offset":14548,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":15124,"parent_record_offset":14956,"ancestor_record_offset":14548,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":14932,"parent_record_offset":14908,"ancestor_record_offset":14548,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":14956,"parent_record_offset":14932,"ancestor_record_offset":14548,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":15148,"parent_record_offset":15124,"ancestor_record_offset":14548,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":15484,"parent_record_offset":15148,"ancestor_record_offset":14548,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":15456,"parent_record_offset":15148,"ancestor_record_offset":14548,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":15260,"parent_record_offset":15148,"ancestor_record_offset":14548,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":14776,"parent_record_offset":14744,"ancestor_record_offset":14548,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<void (*)(ptr_mut$<core::ffi::c_void>)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":27560,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":27784,"parent_record_offset":27560,"ancestor_record_offset":27560,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":27720,"parent_record_offset":27560,"ancestor_record_offset":27560,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":28120,"parent_record_offset":28092,"ancestor_record_offset":27560,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":28092,"parent_record_offset":28064,"ancestor_record_offset":27560,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":28452,"parent_record_offset":28428,"ancestor_record_offset":27560,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":28064,"parent_record_offset":28036,"ancestor_record_offset":27560,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":28428,"parent_record_offset":28404,"ancestor_record_offset":27560,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":28268,"parent_record_offset":28240,"ancestor_record_offset":27560,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":27876,"parent_record_offset":27816,"ancestor_record_offset":27560,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":27748,"parent_record_offset":27720,"ancestor_record_offset":27560,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":28380,"parent_record_offset":28356,"ancestor_record_offset":27560,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":28404,"parent_record_offset":28380,"ancestor_record_offset":27560,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":28036,"parent_record_offset":27996,"ancestor_record_offset":27560,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":28240,"parent_record_offset":28188,"ancestor_record_offset":27560,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":27948,"parent_record_offset":27816,"ancestor_record_offset":27560,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":28164,"parent_record_offset":27996,"ancestor_record_offset":27560,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":27972,"parent_record_offset":27948,"ancestor_record_offset":27560,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":27996,"parent_record_offset":27972,"ancestor_record_offset":27560,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":28188,"parent_record_offset":28164,"ancestor_record_offset":27560,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":28524,"parent_record_offset":28188,"ancestor_record_offset":27560,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":28496,"parent_record_offset":28188,"ancestor_record_offset":27560,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":28300,"parent_record_offset":28188,"ancestor_record_offset":27560,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":27816,"parent_record_offset":27784,"ancestor_record_offset":27560,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<void (*)(ptr_mut$<usdl_sys::mouse::uSDL_Cursor>)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":17856,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":18088,"parent_record_offset":17856,"ancestor_record_offset":17856,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":18024,"parent_record_offset":17856,"ancestor_record_offset":17856,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":18424,"parent_record_offset":18396,"ancestor_record_offset":17856,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":18396,"parent_record_offset":18368,"ancestor_record_offset":17856,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":18756,"parent_record_offset":18732,"ancestor_record_offset":17856,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":18368,"parent_record_offset":18340,"ancestor_record_offset":17856,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":18732,"parent_record_offset":18708,"ancestor_record_offset":17856,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":18572,"parent_record_offset":18544,"ancestor_record_offset":17856,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":18180,"parent_record_offset":18120,"ancestor_record_offset":17856,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":18052,"parent_record_offset":18024,"ancestor_record_offset":17856,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":18684,"parent_record_offset":18660,"ancestor_record_offset":17856,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":18708,"parent_record_offset":18684,"ancestor_record_offset":17856,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":18340,"parent_record_offset":18300,"ancestor_record_offset":17856,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":18544,"parent_record_offset":18492,"ancestor_record_offset":17856,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":18252,"parent_record_offset":18120,"ancestor_record_offset":17856,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":18468,"parent_record_offset":18300,"ancestor_record_offset":17856,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":18276,"parent_record_offset":18252,"ancestor_record_offset":17856,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":18300,"parent_record_offset":18276,"ancestor_record_offset":17856,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":18492,"parent_record_offset":18468,"ancestor_record_offset":17856,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":18828,"parent_record_offset":18492,"ancestor_record_offset":17856,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":18800,"parent_record_offset":18492,"ancestor_record_offset":17856,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":18604,"parent_record_offset":18492,"ancestor_record_offset":17856,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":18120,"parent_record_offset":18088,"ancestor_record_offset":17856,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<void (*)(ptr_mut$<usdl_sys::syswm::uSDL_version>)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":29756,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":29992,"parent_record_offset":29756,"ancestor_record_offset":29756,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":29928,"parent_record_offset":29756,"ancestor_record_offset":29756,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":30328,"parent_record_offset":30300,"ancestor_record_offset":29756,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":30300,"parent_record_offset":30272,"ancestor_record_offset":29756,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":30660,"parent_record_offset":30636,"ancestor_record_offset":29756,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":30272,"parent_record_offset":30244,"ancestor_record_offset":29756,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":30636,"parent_record_offset":30612,"ancestor_record_offset":29756,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":30476,"parent_record_offset":30448,"ancestor_record_offset":29756,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":30084,"parent_record_offset":30024,"ancestor_record_offset":29756,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":29956,"parent_record_offset":29928,"ancestor_record_offset":29756,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":30588,"parent_record_offset":30564,"ancestor_record_offset":29756,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":30612,"parent_record_offset":30588,"ancestor_record_offset":29756,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":30244,"parent_record_offset":30204,"ancestor_record_offset":29756,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":30448,"parent_record_offset":30396,"ancestor_record_offset":29756,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":30156,"parent_record_offset":30024,"ancestor_record_offset":29756,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":30372,"parent_record_offset":30204,"ancestor_record_offset":29756,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":30180,"parent_record_offset":30156,"ancestor_record_offset":29756,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":30204,"parent_record_offset":30180,"ancestor_record_offset":29756,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":30396,"parent_record_offset":30372,"ancestor_record_offset":29756,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":30732,"parent_record_offset":30396,"ancestor_record_offset":29756,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":30704,"parent_record_offset":30396,"ancestor_record_offset":29756,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":30508,"parent_record_offset":30396,"ancestor_record_offset":29756,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":30024,"parent_record_offset":29992,"ancestor_record_offset":29756,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<void (*)(ptr_mut$<usdl_sys::video::uSDL_Window>)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":25416,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":25648,"parent_record_offset":25416,"ancestor_record_offset":25416,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":25584,"parent_record_offset":25416,"ancestor_record_offset":25416,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":25984,"parent_record_offset":25956,"ancestor_record_offset":25416,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":25956,"parent_record_offset":25928,"ancestor_record_offset":25416,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":26316,"parent_record_offset":26292,"ancestor_record_offset":25416,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":25928,"parent_record_offset":25900,"ancestor_record_offset":25416,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":26292,"parent_record_offset":26268,"ancestor_record_offset":25416,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":26132,"parent_record_offset":26104,"ancestor_record_offset":25416,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":25740,"parent_record_offset":25680,"ancestor_record_offset":25416,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":25612,"parent_record_offset":25584,"ancestor_record_offset":25416,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":26244,"parent_record_offset":26220,"ancestor_record_offset":25416,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":26268,"parent_record_offset":26244,"ancestor_record_offset":25416,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":25900,"parent_record_offset":25860,"ancestor_record_offset":25416,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":26104,"parent_record_offset":26052,"ancestor_record_offset":25416,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":25812,"parent_record_offset":25680,"ancestor_record_offset":25416,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":26028,"parent_record_offset":25860,"ancestor_record_offset":25416,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":25836,"parent_record_offset":25812,"ancestor_record_offset":25416,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":25860,"parent_record_offset":25836,"ancestor_record_offset":25416,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":26052,"parent_record_offset":26028,"ancestor_record_offset":25416,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":26388,"parent_record_offset":26052,"ancestor_record_offset":25416,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":26360,"parent_record_offset":26052,"ancestor_record_offset":25416,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":26164,"parent_record_offset":26052,"ancestor_record_offset":25416,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":25680,"parent_record_offset":25648,"ancestor_record_offset":25416,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<void (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,i32)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":11344,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":11580,"parent_record_offset":11344,"ancestor_record_offset":11344,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":11516,"parent_record_offset":11344,"ancestor_record_offset":11344,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":11916,"parent_record_offset":11888,"ancestor_record_offset":11344,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":11888,"parent_record_offset":11860,"ancestor_record_offset":11344,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":12248,"parent_record_offset":12224,"ancestor_record_offset":11344,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":11860,"parent_record_offset":11832,"ancestor_record_offset":11344,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":12224,"parent_record_offset":12200,"ancestor_record_offset":11344,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":12064,"parent_record_offset":12036,"ancestor_record_offset":11344,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":11672,"parent_record_offset":11612,"ancestor_record_offset":11344,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":11544,"parent_record_offset":11516,"ancestor_record_offset":11344,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":12176,"parent_record_offset":12152,"ancestor_record_offset":11344,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":12200,"parent_record_offset":12176,"ancestor_record_offset":11344,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":11832,"parent_record_offset":11792,"ancestor_record_offset":11344,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":12036,"parent_record_offset":11984,"ancestor_record_offset":11344,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":11744,"parent_record_offset":11612,"ancestor_record_offset":11344,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":11960,"parent_record_offset":11792,"ancestor_record_offset":11344,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":11768,"parent_record_offset":11744,"ancestor_record_offset":11344,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":11792,"parent_record_offset":11768,"ancestor_record_offset":11344,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":11984,"parent_record_offset":11960,"ancestor_record_offset":11344,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":12320,"parent_record_offset":11984,"ancestor_record_offset":11344,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":12292,"parent_record_offset":11984,"ancestor_record_offset":11344,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":12096,"parent_record_offset":11984,"ancestor_record_offset":11344,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":11612,"parent_record_offset":11580,"ancestor_record_offset":11344,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<void (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,i32,i32)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":8064,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":8304,"parent_record_offset":8064,"ancestor_record_offset":8064,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":8240,"parent_record_offset":8064,"ancestor_record_offset":8064,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":8640,"parent_record_offset":8612,"ancestor_record_offset":8064,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":8612,"parent_record_offset":8584,"ancestor_record_offset":8064,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":8972,"parent_record_offset":8948,"ancestor_record_offset":8064,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":8584,"parent_record_offset":8556,"ancestor_record_offset":8064,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":8948,"parent_record_offset":8924,"ancestor_record_offset":8064,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":8788,"parent_record_offset":8760,"ancestor_record_offset":8064,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":8396,"parent_record_offset":8336,"ancestor_record_offset":8064,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":8268,"parent_record_offset":8240,"ancestor_record_offset":8064,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":8900,"parent_record_offset":8876,"ancestor_record_offset":8064,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":8924,"parent_record_offset":8900,"ancestor_record_offset":8064,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":8556,"parent_record_offset":8516,"ancestor_record_offset":8064,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":8760,"parent_record_offset":8708,"ancestor_record_offset":8064,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":8468,"parent_record_offset":8336,"ancestor_record_offset":8064,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":8684,"parent_record_offset":8516,"ancestor_record_offset":8064,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":8492,"parent_record_offset":8468,"ancestor_record_offset":8064,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":8516,"parent_record_offset":8492,"ancestor_record_offset":8064,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":8708,"parent_record_offset":8684,"ancestor_record_offset":8064,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":9044,"parent_record_offset":8708,"ancestor_record_offset":8064,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":9016,"parent_record_offset":8708,"ancestor_record_offset":8064,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":8820,"parent_record_offset":8708,"ancestor_record_offset":8064,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":8336,"parent_record_offset":8304,"ancestor_record_offset":8064,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<void (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,ptr_mut$<i32>,ptr_mut$<i32>)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":28640,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":28900,"parent_record_offset":28640,"ancestor_record_offset":28640,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":28836,"parent_record_offset":28640,"ancestor_record_offset":28640,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":29236,"parent_record_offset":29208,"ancestor_record_offset":28640,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":29208,"parent_record_offset":29180,"ancestor_record_offset":28640,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":29568,"parent_record_offset":29544,"ancestor_record_offset":28640,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":29180,"parent_record_offset":29152,"ancestor_record_offset":28640,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":29544,"parent_record_offset":29520,"ancestor_record_offset":28640,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":29384,"parent_record_offset":29356,"ancestor_record_offset":28640,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":28992,"parent_record_offset":28932,"ancestor_record_offset":28640,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":28864,"parent_record_offset":28836,"ancestor_record_offset":28640,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":29496,"parent_record_offset":29472,"ancestor_record_offset":28640,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":29520,"parent_record_offset":29496,"ancestor_record_offset":28640,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":29152,"parent_record_offset":29112,"ancestor_record_offset":28640,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":29356,"parent_record_offset":29304,"ancestor_record_offset":28640,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":29064,"parent_record_offset":28932,"ancestor_record_offset":28640,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":29280,"parent_record_offset":29112,"ancestor_record_offset":28640,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":29088,"parent_record_offset":29064,"ancestor_record_offset":28640,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":29112,"parent_record_offset":29088,"ancestor_record_offset":28640,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":29304,"parent_record_offset":29280,"ancestor_record_offset":28640,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":29640,"parent_record_offset":29304,"ancestor_record_offset":28640,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":29612,"parent_record_offset":29304,"ancestor_record_offset":28640,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":29416,"parent_record_offset":29304,"ancestor_record_offset":28640,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":28932,"parent_record_offset":28900,"ancestor_record_offset":28640,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol<void (*)(u32)>
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1021,"record_offset":24360,"rva":16786816,"va":5385495936}
// 编译实例 RVAs: 0x1002580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::from_bytes_with_nul
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89f8df5ff","size":5,"soft_fail":false,"target_va":5384811424,"va":5385496060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::ffi::c_str::CStr>::to_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-684125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e8a38ff5ff","size":5,"soft_fail":false,"target_va":5384811968,"va":5385496088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141002510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e88ffdffff","size":5,"soft_fail":false,"target_va":5385495824,"va":5385496444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31125281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e821efda01","size":5,"soft_fail":false,"target_va":5416621632,"va":5385496346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libloading::os::windows::Library::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libloading","version":"0.7.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385495936,"raw_hex":"e89ffcffff","size":5,"soft_fail":false,"target_va":5385495120,"va":5385495980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":24560,"parent_record_offset":24360,"ancestor_record_offset":24360,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] libloading::safe::Library::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"libloading","version":"0.8.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libloading-0.8.3/src/safe.rs
//     evidence={"module":1021,"record_offset":24496,"parent_record_offset":24360,"ancestor_record_offset":24360,"inlinee":18184,"source_line":147,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":24896,"parent_record_offset":24868,"ancestor_record_offset":24360,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":24868,"parent_record_offset":24840,"ancestor_record_offset":24360,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":25228,"parent_record_offset":25204,"ancestor_record_offset":24360,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":24840,"parent_record_offset":24812,"ancestor_record_offset":24360,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":25204,"parent_record_offset":25180,"ancestor_record_offset":24360,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":25044,"parent_record_offset":25016,"ancestor_record_offset":24360,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":24652,"parent_record_offset":24592,"ancestor_record_offset":24360,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":24524,"parent_record_offset":24496,"ancestor_record_offset":24360,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":25156,"parent_record_offset":25132,"ancestor_record_offset":24360,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":25180,"parent_record_offset":25156,"ancestor_record_offset":24360,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":24812,"parent_record_offset":24772,"ancestor_record_offset":24360,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":25016,"parent_record_offset":24964,"ancestor_record_offset":24360,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":24724,"parent_record_offset":24592,"ancestor_record_offset":24360,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":24940,"parent_record_offset":24772,"ancestor_record_offset":24360,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":24748,"parent_record_offset":24724,"ancestor_record_offset":24360,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":24772,"parent_record_offset":24748,"ancestor_record_offset":24360,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":24964,"parent_record_offset":24940,"ancestor_record_offset":24360,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":25300,"parent_record_offset":24964,"ancestor_record_offset":24360,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":25272,"parent_record_offset":24964,"ancestor_record_offset":24360,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":25076,"parent_record_offset":24964,"ancestor_record_offset":24360,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] usdl_sys::backend::load_symbol::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1021,"record_offset":24592,"parent_record_offset":24560,"ancestor_record_offset":24360,"inlinee":64845,"source_line":302,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::backend::load_symbol::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=56
// 源码行: 302
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFD4D
// 首条位置: {"module":1021,"inlinee":64845}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=56
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1021,"record_offset":1804,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::trim_end_matches
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1021,"record_offset":1876,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":23496,"source_line":2627,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1021,"record_offset":2048,"parent_record_offset":2020,"ancestor_record_offset":1744,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1021,"record_offset":2020,"parent_record_offset":1992,"ancestor_record_offset":1744,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":2380,"parent_record_offset":2356,"ancestor_record_offset":1744,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":1992,"parent_record_offset":1964,"ancestor_record_offset":1744,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":2356,"parent_record_offset":2332,"ancestor_record_offset":1744,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1021,"record_offset":2196,"parent_record_offset":2168,"ancestor_record_offset":1744,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":2308,"parent_record_offset":2284,"ancestor_record_offset":1744,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":2332,"parent_record_offset":2308,"ancestor_record_offset":1744,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":1964,"parent_record_offset":1924,"ancestor_record_offset":1744,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=56
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1021,"record_offset":2168,"parent_record_offset":2116,"ancestor_record_offset":1744,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1021,"record_offset":2092,"parent_record_offset":1924,"ancestor_record_offset":1744,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_reject_back
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":1900,"parent_record_offset":1876,"ancestor_record_offset":1744,"inlinee":8312,"source_line":328,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_back
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1021,"record_offset":1924,"parent_record_offset":1900,"ancestor_record_offset":1744,"inlinee":21320,"source_line":479,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":1744,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":2452,"parent_record_offset":2116,"ancestor_record_offset":1744,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":2424,"parent_record_offset":2116,"ancestor_record_offset":1744,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=56
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1021,"record_offset":2228,"parent_record_offset":2116,"ancestor_record_offset":1744,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01021.jsonl.gz"}
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
// 完整结构及原始类型记录: backend.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] usdl_sys::backend::DynamicBackend
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::impl$0::try_insert::closure_env$0<usdl_sys::backend::DynamicBackend>", "constructor": "std::sync::once_lock::impl$0::try_insert::closure_env$0", "arguments": ["usdl_sys::backend::DynamicBackend"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::impl$0::get_or_init::closure_env$0<usdl_sys::backend::DynamicBackend,std::sync::once_lock::impl$0::try_insert::closure_env$0<usdl_sys::backend::DynamicBackend> >", "constructor": "std::sync::once_lock::impl$0::get_or_init::closure_env$0", "arguments": ["usdl_sys::backend::DynamicBackend", "std::sync::once_lock::impl$0::try_insert::closure_env$0<usdl_sys::backend::DynamicBackend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::impl$0::initialize::closure_env$0<usdl_sys::backend::DynamicBackend,std::sync::once_lock::impl$0::get_or_init::closure_env$0<usdl_sys::backend::DynamicBackend,std::sync::once_lock::impl$0::try_insert::closure_env$0<usdl_sys::backend::DynamicBackend> >,never$>", "constructor": "std::sync::once_lock::impl$0::initialize::closure_env$0", "arguments": ["usdl_sys::backend::DynamicBackend", "std::sync::once_lock::impl$0::get_or_init::closure_env$0<usdl_sys::backend::DynamicBackend,std::sync::once_lock::impl$0::try_insert::closure_env$0<usdl_sys::backend::DynamicBackend> >", "never$"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once::impl$2::call_once_force::closure$0<std::sync::once_lock::impl$0::initialize::closure_env$0<usdl_sys::backend::DynamicBackend,std::sync::once_lock::impl$0::get_or_init::closure_env$0<usdl_sys::backend::DynamicBackend,std::sync::once_lock::impl$0::try_insert::closure_env$0<usdl_sys::backend::DynamicBackend> >,never$> >", "constructor": "std::sync::once::impl$2::call_once_force::closure$0", "arguments": ["std::sync::once_lock::impl$0::initialize::closure_env$0<usdl_sys::backend::DynamicBackend,std::sync::once_lock::impl$0::get_or_init::closure_env$0<usdl_sys::backend::DynamicBackend,std::sync::once_lock::impl$0::try_insert::closure_env$0<usdl_sys::backend::DynamicBackend> >,never$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::OnceLock::initialize<usdl_sys::backend::DynamicBackend,std::sync::once_lock::impl$0::get_or_init::closure_env$0<usdl_sys::backend::DynamicBackend,std::sync::once_lock::impl$0::try_insert::closure_env$0<usdl_sys::backend::DynamicBackend> >,never$>", "constructor": "std::sync::once_lock::OnceLock::initialize", "arguments": ["usdl_sys::backend::DynamicBackend", "std::sync::once_lock::impl$0::get_or_init::closure_env$0<usdl_sys::backend::DynamicBackend,std::sync::once_lock::impl$0::try_insert::closure_env$0<usdl_sys::backend::DynamicBackend> >", "never$"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<usdl_sys::backend::DynamicBackend,std::sync::once_lock::impl$0::get_or_init::closure_env$0<usdl_sys::backend::DynamicBackend,std::sync::once_lock::impl$0::try_insert::closure_env$0<usdl_sys::backend::DynamicBackend> >,never$> >", "constructor": "std::sync::once::impl$2::call_once_force::closure_env$0", "arguments": ["std::sync::once_lock::impl$0::initialize::closure_env$0<usdl_sys::backend::DynamicBackend,std::sync::once_lock::impl$0::get_or_init::closure_env$0<usdl_sys::backend::DynamicBackend,std::sync::once_lock::impl$0::try_insert::closure_env$0<usdl_sys::backend::DynamicBackend> >,never$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<usdl_sys::backend::DynamicBackend,std::sync::once_lock::impl$0::get_or_init::closure_env$0<usdl_sys::backend::DynamicBackend,std::sync::once_lock::impl$0::try_insert::closure_env$0<usdl_sys::backend::DynamicBackend> >,never$> >,tuple$<ref$<std::sync::once::OnceState> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<usdl_sys::backend::DynamicBackend,std::sync::once_lock::impl$0::get_or_init::closure_env$0<usdl_sys::backend::DynamicBackend,std::sync::once_lock::impl$0::try_insert::closure_env$0<usdl_sys::backend::DynamicBackend> >,never$> >", "tuple$<ref$<std::sync::once::OnceState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] usdl_sys::backend::LoadLibraryError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 22
// [PDB 泛型关系，非字段关系] {"type": "enum2$<usdl_sys::backend::LoadLibraryError>", "constructor": "enum2$", "arguments": ["usdl_sys::backend::LoadLibraryError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> >", "constructor": "anyhow::error::ContextError", "arguments": ["ref$<str$>", "enum2$<usdl_sys::backend::LoadLibraryError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::ErrorImpl<anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> > >", "constructor": "anyhow::error::ErrorImpl", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<anyhow::error::ErrorImpl<anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["anyhow::error::ErrorImpl<anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<usdl_sys::backend::LoadLibraryError> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<usdl_sys::backend::LoadLibraryError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::cause<enum2$<usdl_sys::backend::LoadLibraryError> >", "constructor": "core::error::Error::cause", "arguments": ["enum2$<usdl_sys::backend::LoadLibraryError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::type_id<anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> > >", "constructor": "core::error::Error::type_id", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::type_id<enum2$<usdl_sys::backend::LoadLibraryError> >", "constructor": "core::error::Error::type_id", "arguments": ["enum2$<usdl_sys::backend::LoadLibraryError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::type_id<anyhow::error::ErrorImpl<anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> > > >", "constructor": "core::error::Error::type_id", "arguments": ["anyhow::error::ErrorImpl<anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_ref<anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> > >", "constructor": "anyhow::error::object_ref", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_boxed<anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> > >", "constructor": "anyhow::error::object_boxed", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::context_downcast<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> >", "constructor": "anyhow::error::context_downcast", "arguments": ["ref$<str$>", "enum2$<usdl_sys::backend::LoadLibraryError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::Error::construct<anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> > >", "constructor": "anyhow::Error::construct", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$4::source<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> >", "constructor": "anyhow::context::impl$4::source", "arguments": ["ref$<str$>", "enum2$<usdl_sys::backend::LoadLibraryError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::cause<anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> > >", "constructor": "core::error::Error::cause", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_drop<anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> > >", "constructor": "anyhow::error::object_drop", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::context_drop_rest<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> >", "constructor": "anyhow::error::context_drop_rest", "arguments": ["ref$<str$>", "enum2$<usdl_sys::backend::LoadLibraryError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_reallocate_boxed<anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> > >", "constructor": "anyhow::error::object_reallocate_boxed", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$2::fmt<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> >", "constructor": "anyhow::context::impl$2::fmt", "arguments": ["ref$<str$>", "enum2$<usdl_sys::backend::LoadLibraryError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::impl$10::fmt<anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> > >", "constructor": "anyhow::error::impl$10::fmt", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<usdl_sys::backend::LoadLibraryError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: backend.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: backend.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 28; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] i32 (*)(ptr_mut$<usdl_sys::event::uSDL_Event>)
// 候选参数类型: ["ptr_mut$<usdl_sys::event::uSDL_Event>"]
// [文件级接口候选，具体函数归属 unknown] u32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>)
// 候选参数类型: ["ptr_mut$<usdl_sys::video::uSDL_Window>"]
// [文件级接口候选，具体函数归属 unknown] i32 (*)(i32,i32,ptr_mut$<usdl_sys::video::uSDL_DisplayMode>)
// 候选参数类型: ["i32", "i32", "ptr_mut$<usdl_sys::video::uSDL_DisplayMode>"]
// [文件级接口候选，具体函数归属 unknown] i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,ptr_mut$<usdl_sys::syswm::uSDL_SysWMinfo>)
// 候选参数类型: ["ptr_mut$<usdl_sys::video::uSDL_Window>", "ptr_mut$<usdl_sys::syswm::uSDL_SysWMinfo>"]
// [文件级接口候选，具体函数归属 unknown] i32 (*)(u32)
// 候选参数类型: ["u32"]
// [文件级接口候选，具体函数归属 unknown] i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,usdl_sys::video::uSDL_FullscreenType)
// 候选参数类型: ["ptr_mut$<usdl_sys::video::uSDL_Window>", "usdl_sys::video::uSDL_FullscreenType"]
// [文件级接口候选，具体函数归属 unknown] void (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,i32,i32)
// 候选参数类型: ["ptr_mut$<usdl_sys::video::uSDL_Window>", "i32", "i32"]
// [文件级接口候选，具体函数归属 unknown] i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>)
// 候选参数类型: ["ptr_mut$<usdl_sys::video::uSDL_Window>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::DynamicBackend::load
// 信息 ID: 5cc3d4044087b2676f67c283
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::load_library<ref$<ref$<str$> > > @ 0x1002814 -> 0x6cb980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::active_api
// 信息 ID: 7737e5e0fd620bba65c78ae1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 41 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] usdl_sys::uSDL_GetError; module=1005, inline_record=372; 不是独立调用边。
// [PDB 内联宿主] usdl_sys::uSDL_InitSubSystem; module=1005, inline_record=824; 不是独立调用边。
// [PDB 内联宿主] usdl_sys::uSDL_QuitSubSystem; module=1005, inline_record=1276; 不是独立调用边。
// [PDB 内联宿主] usdl_sys::uSDL_Init; module=1005, inline_record=1720; 不是独立调用边。
// [PDB 内联宿主] usdl_sys::uSDL_Quit; module=1005, inline_record=2164; 不是独立调用边。
// [PDB 内联宿主] usdl_sys::event::uSDL_PollEvent; module=1006, inline_record=488; 不是独立调用边。
// [PDB 内联宿主] usdl_sys::event::uSDL_EventState; module=1006, inline_record=944; 不是独立调用边。
// [PDB 内联宿主] usdl_sys::event::uSDL_WaitEventTimeout; module=1006, inline_record=1524; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::impl$13::fmt
// 信息 ID: b0f6d33d45a0a2e66c5fed5d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::impl$2::fmt
// 信息 ID: 8f333f46592f7136afb109e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_default_library
// 信息 ID: 67b9715278bbe50dda4ca704
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::sdl::load_default_library @ 0x6ca180 -> 0x6ccad0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_library::closure$0
// 信息 ID: cd353f7045318b75c6703b09
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "usdl_sys::backend::load_library", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] usdl_sys::backend::load_library<ref$<ref$<str$> > >; module=1021, inline_record=32892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_library::closure$1
// 信息 ID: ac30a44e87ef995a194813a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "usdl_sys::backend::load_library", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] usdl_sys::backend::load_library<ref$<ref$<str$> > >; module=1021, inline_record=33772; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_library<ref$<ref$<str$> > >
// 信息 ID: 0b91b233b07042c5067b5cab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["ref$<str$>"]; 字段布局 = unknown。
// [类型关系] usdl_sys::backend::load_library -> ["ref$<ref$<str$> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::load_default_library @ 0x6ccb9e -> 0x1002790 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol::closure$1
// 信息 ID: d9b6b438983d5a827a578cba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0a6e3189a0f93c5a9956b2c1: i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,ptr_const$<usdl_sys::video::uSDL_DisplayMode>)
// [泛型宿主/闭包/直接引用方参数候选] 0b145ed7fe4212f8840f111e: i32 (*)(u32)
// [泛型宿主/闭包/直接引用方参数候选] 11c20c1cf1bb362702bf0277: i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,ptr_mut$<usdl_sys::syswm::uSDL_SysWMinfo>)
// [泛型宿主/闭包/直接引用方参数候选] 172d7af34f4f21ca51a02e7a: i32 (*)(ptr_mut$<usdl_sys::event::uSDL_Event>,i32)
// [泛型宿主/闭包/直接引用方参数候选] 1ec8b60634d4163cb6901f58: void (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,i32)
// [泛型宿主/闭包/直接引用方参数候选] 21cd1f931c01a5a83ccda16e: void (*)(ptr_mut$<usdl_sys::syswm::uSDL_version>)
// [泛型宿主/闭包/直接引用方参数候选] 274df399be507b4a5f002e71: void (*)(ptr_mut$<usdl_sys::video::uSDL_Window>)
// [泛型宿主/闭包/直接引用方参数候选] 2887111b9fc1eeb4704debe8: void (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,ptr_mut$<i32>,ptr_mut$<i32>)
// 其余候选接口见完整附档，共 28 条。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "usdl_sys::backend::load_symbol", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 28 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] usdl_sys::backend::load_symbol<i32 (*)(ptr_mut$<usdl_sys::event::uSDL_Event>)>; module=1021, inline_record=1744; 不是独立调用边。
// [PDB 内联宿主] usdl_sys::backend::load_symbol<u32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>)>; module=1021, inline_record=2832; 不是独立调用边。
// [PDB 内联宿主] usdl_sys::backend::load_symbol<i32 (*)(i32,i32,ptr_mut$<usdl_sys::video::uSDL_DisplayMode>)>; module=1021, inline_record=3932; 不是独立调用边。
// [PDB 内联宿主] usdl_sys::backend::load_symbol<i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,ptr_mut$<usdl_sys::syswm::uSDL_SysWMinfo>)>; module=1021, inline_record=5064; 不是独立调用边。
// [PDB 内联宿主] usdl_sys::backend::load_symbol<i32 (*)(u32)>; module=1021, inline_record=6116; 不是独立调用边。
// [PDB 内联宿主] usdl_sys::backend::load_symbol<i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,usdl_sys::video::uSDL_FullscreenType)>; module=1021, inline_record=7240; 不是独立调用边。
// [PDB 内联宿主] usdl_sys::backend::load_symbol<void (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,i32,i32)>; module=1021, inline_record=8336; 不是独立调用边。
// [PDB 内联宿主] usdl_sys::backend::load_symbol<i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>)>; module=1021, inline_record=9424; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<i32 (*)()>
// 信息 ID: d5a9007f5b834ab0d6c321ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] i32 (*)()
// 参数类型拼写: []
// [类型关系] usdl_sys::backend::load_symbol -> ["i32 (*)()"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<i32 (*)(i32)>
// 信息 ID: b6c2d6125da1270b8608d5f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] i32 (*)(i32)
// 参数类型拼写: ["i32"]
// [类型关系] usdl_sys::backend::load_symbol -> ["i32 (*)(i32)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<i32 (*)(i32,i32,ptr_mut$<usdl_sys::video::uSDL_DisplayMode>)>
// 信息 ID: 2227be8612a17e60af361fc8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] i32 (*)(i32,i32,ptr_mut$<usdl_sys::video::uSDL_DisplayMode>)
// 参数类型拼写: ["i32", "i32", "ptr_mut$<usdl_sys::video::uSDL_DisplayMode>"]
// [类型关系] ptr_mut$ -> ["usdl_sys::video::uSDL_DisplayMode"]; 字段布局 = unknown。
// [类型关系] usdl_sys::backend::load_symbol -> ["i32 (*)(i32,i32,ptr_mut$<usdl_sys::video::uSDL_DisplayMode>)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<i32 (*)(i32,ptr_mut$<usdl_sys::rect::uSDL_Rect>)>
// 信息 ID: 41bd8e2668c5e1cf71fb6243
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] i32 (*)(i32,ptr_mut$<usdl_sys::rect::uSDL_Rect>)
// 参数类型拼写: ["i32", "ptr_mut$<usdl_sys::rect::uSDL_Rect>"]
// [类型关系] ptr_mut$ -> ["usdl_sys::rect::uSDL_Rect"]; 字段布局 = unknown。
// [类型关系] usdl_sys::backend::load_symbol -> ["i32 (*)(i32,ptr_mut$<usdl_sys::rect::uSDL_Rect>)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<i32 (*)(i32,ptr_mut$<usdl_sys::video::uSDL_DisplayMode>)>
// 信息 ID: 4d95237a92fa1a3c5c166046
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] i32 (*)(i32,ptr_mut$<usdl_sys::video::uSDL_DisplayMode>)
// 参数类型拼写: ["i32", "ptr_mut$<usdl_sys::video::uSDL_DisplayMode>"]
// [类型关系] ptr_mut$ -> ["usdl_sys::video::uSDL_DisplayMode"]; 字段布局 = unknown。
// [类型关系] usdl_sys::backend::load_symbol -> ["i32 (*)(i32,ptr_mut$<usdl_sys::video::uSDL_DisplayMode>)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<i32 (*)(ptr_mut$<usdl_sys::event::uSDL_Event>)>
// 信息 ID: f16835cd571f61eea53cb148
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] i32 (*)(ptr_mut$<usdl_sys::event::uSDL_Event>)
// 参数类型拼写: ["ptr_mut$<usdl_sys::event::uSDL_Event>"]
// [类型关系] ptr_mut$ -> ["usdl_sys::event::uSDL_Event"]; 字段布局 = unknown。
// [类型关系] usdl_sys::backend::load_symbol -> ["i32 (*)(ptr_mut$<usdl_sys::event::uSDL_Event>)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<i32 (*)(ptr_mut$<usdl_sys::event::uSDL_Event>,i32)>
// 信息 ID: 863c131cd96e2ae3cd142e37
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] i32 (*)(ptr_mut$<usdl_sys::event::uSDL_Event>,i32)
// 参数类型拼写: ["ptr_mut$<usdl_sys::event::uSDL_Event>", "i32"]
// [类型关系] ptr_mut$ -> ["usdl_sys::event::uSDL_Event"]; 字段布局 = unknown。
// [类型关系] usdl_sys::backend::load_symbol -> ["i32 (*)(ptr_mut$<usdl_sys::event::uSDL_Event>,i32)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>)>
// 信息 ID: 88e04e436cc574a170c63202
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>)
// 参数类型拼写: ["ptr_mut$<usdl_sys::video::uSDL_Window>"]
// [类型关系] ptr_mut$ -> ["usdl_sys::video::uSDL_Window"]; 字段布局 = unknown。
// [类型关系] usdl_sys::backend::load_symbol -> ["i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,ptr_const$<usdl_sys::video::uSDL_DisplayMode>)>
// 信息 ID: a085a0ca37b586a0b3b8f286
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,ptr_const$<usdl_sys::video::uSDL_DisplayMode>)
// 参数类型拼写: ["ptr_mut$<usdl_sys::video::uSDL_Window>", "ptr_const$<usdl_sys::video::uSDL_DisplayMode>"]
// [类型关系] ptr_mut$ -> ["usdl_sys::video::uSDL_Window"]; 字段布局 = unknown。
// [类型关系] ptr_const$ -> ["usdl_sys::video::uSDL_DisplayMode"]; 字段布局 = unknown。
// [类型关系] usdl_sys::backend::load_symbol -> ["i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,ptr_const$<usdl_sys::video::uSDL_DisplayMode>)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,ptr_mut$<usdl_sys::syswm::uSDL_SysWMinfo>)>
// 信息 ID: 3efd415664ab47bd8df22b63
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,ptr_mut$<usdl_sys::syswm::uSDL_SysWMinfo>)
// 参数类型拼写: ["ptr_mut$<usdl_sys::video::uSDL_Window>", "ptr_mut$<usdl_sys::syswm::uSDL_SysWMinfo>"]
// [类型关系] ptr_mut$ -> ["usdl_sys::video::uSDL_Window"]; 字段布局 = unknown。
// [类型关系] ptr_mut$ -> ["usdl_sys::syswm::uSDL_SysWMinfo"]; 字段布局 = unknown。
// [类型关系] usdl_sys::backend::load_symbol -> ["i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,ptr_mut$<usdl_sys::syswm::uSDL_SysWMinfo>)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,usdl_sys::video::uSDL_FullscreenType)>
// 信息 ID: 825e2190e2a814e70825b8a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,usdl_sys::video::uSDL_FullscreenType)
// 参数类型拼写: ["ptr_mut$<usdl_sys::video::uSDL_Window>", "usdl_sys::video::uSDL_FullscreenType"]
// [类型关系] ptr_mut$ -> ["usdl_sys::video::uSDL_Window"]; 字段布局 = unknown。
// [类型关系] usdl_sys::backend::load_symbol -> ["i32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,usdl_sys::video::uSDL_FullscreenType)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<i32 (*)(u32)>
// 信息 ID: 1c563b9877612b30c353f3dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] i32 (*)(u32)
// 参数类型拼写: ["u32"]
// [类型关系] usdl_sys::backend::load_symbol -> ["i32 (*)(u32)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<ptr_const$<i8> (*)()>
// 信息 ID: d0cdfddf44d2ead9cd6e14ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] ptr_const$<i8> (*)()
// 参数类型拼写: []
// [类型关系] ptr_const$ -> ["i8"]; 字段布局 = unknown。
// [类型关系] usdl_sys::backend::load_symbol -> ["ptr_const$<i8> (*)()"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<ptr_const$<i8> (*)(i32)>
// 信息 ID: 070050d51971437aad06783f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] ptr_const$<i8> (*)(i32)
// 参数类型拼写: ["i32"]
// [类型关系] ptr_const$ -> ["i8"]; 字段布局 = unknown。
// [类型关系] usdl_sys::backend::load_symbol -> ["ptr_const$<i8> (*)(i32)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<ptr_mut$<usdl_sys::mouse::uSDL_Cursor> (*)(usdl_sys::mouse::uSDL_SystemCursor)>
// 信息 ID: 3301184b8f3ef9f2403bcdda
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] ptr_mut$<usdl_sys::mouse::uSDL_Cursor> (*)(usdl_sys::mouse::uSDL_SystemCursor)
// 参数类型拼写: ["usdl_sys::mouse::uSDL_SystemCursor"]
// [类型关系] ptr_mut$ -> ["usdl_sys::mouse::uSDL_Cursor"]; 字段布局 = unknown。
// [类型关系] usdl_sys::backend::load_symbol -> ["ptr_mut$<usdl_sys::mouse::uSDL_Cursor> (*)(usdl_sys::mouse::uSDL_SystemCursor)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<ptr_mut$<usdl_sys::video::uSDL_Window> (*)(ptr_const$<i8>,i32,i32,i32,i32,u32)>
// 信息 ID: 43a581384d78d991d7ddf982
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] ptr_mut$<usdl_sys::video::uSDL_Window> (*)(ptr_const$<i8>,i32,i32,i32,i32,u32)
// 参数类型拼写: ["ptr_const$<i8>", "i32", "i32", "i32", "i32", "u32"]
// [类型关系] ptr_mut$ -> ["usdl_sys::video::uSDL_Window"]; 字段布局 = unknown。
// [类型关系] ptr_const$ -> ["i8"]; 字段布局 = unknown。
// [类型关系] usdl_sys::backend::load_symbol -> ["ptr_mut$<usdl_sys::video::uSDL_Window> (*)(ptr_const$<i8>,i32,i32,i32,i32,u32)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<u16 (*)()>
// 信息 ID: 7d358b5bb32e870693d6425b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] u16 (*)()
// 参数类型拼写: []
// [类型关系] usdl_sys::backend::load_symbol -> ["u16 (*)()"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<u32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>)>
// 信息 ID: 92a92363dcf87be82a8904c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] u32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>)
// 参数类型拼写: ["ptr_mut$<usdl_sys::video::uSDL_Window>"]
// [类型关系] ptr_mut$ -> ["usdl_sys::video::uSDL_Window"]; 字段布局 = unknown。
// [类型关系] usdl_sys::backend::load_symbol -> ["u32 (*)(ptr_mut$<usdl_sys::video::uSDL_Window>)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<u8 (*)(u32,i32)>
// 信息 ID: fc67a239a61546885a50a262
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] u8 (*)(u32,i32)
// 参数类型拼写: ["u32", "i32"]
// [类型关系] usdl_sys::backend::load_symbol -> ["u8 (*)(u32,i32)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<void (*)()>
// 信息 ID: 884e43d0d46ce57ade8db4be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] void (*)()
// 参数类型拼写: []
// [类型关系] usdl_sys::backend::load_symbol -> ["void (*)()"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<void (*)(ptr_mut$<core::ffi::c_void>)>
// 信息 ID: 7fee69be49258f4f29a758ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] void (*)(ptr_mut$<core::ffi::c_void>)
// 参数类型拼写: ["ptr_mut$<core::ffi::c_void>"]
// [类型关系] ptr_mut$ -> ["core::ffi::c_void"]; 字段布局 = unknown。
// [类型关系] usdl_sys::backend::load_symbol -> ["void (*)(ptr_mut$<core::ffi::c_void>)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<void (*)(ptr_mut$<usdl_sys::mouse::uSDL_Cursor>)>
// 信息 ID: 355c5b5e4866b10bcf74861a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] void (*)(ptr_mut$<usdl_sys::mouse::uSDL_Cursor>)
// 参数类型拼写: ["ptr_mut$<usdl_sys::mouse::uSDL_Cursor>"]
// [类型关系] ptr_mut$ -> ["usdl_sys::mouse::uSDL_Cursor"]; 字段布局 = unknown。
// [类型关系] usdl_sys::backend::load_symbol -> ["void (*)(ptr_mut$<usdl_sys::mouse::uSDL_Cursor>)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<void (*)(ptr_mut$<usdl_sys::syswm::uSDL_version>)>
// 信息 ID: 2fdea74e3764f21da0ab3c6d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] void (*)(ptr_mut$<usdl_sys::syswm::uSDL_version>)
// 参数类型拼写: ["ptr_mut$<usdl_sys::syswm::uSDL_version>"]
// [类型关系] ptr_mut$ -> ["usdl_sys::syswm::uSDL_version"]; 字段布局 = unknown。
// [类型关系] usdl_sys::backend::load_symbol -> ["void (*)(ptr_mut$<usdl_sys::syswm::uSDL_version>)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<void (*)(ptr_mut$<usdl_sys::video::uSDL_Window>)>
// 信息 ID: bae4c2c3527f608e556313b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] void (*)(ptr_mut$<usdl_sys::video::uSDL_Window>)
// 参数类型拼写: ["ptr_mut$<usdl_sys::video::uSDL_Window>"]
// [类型关系] ptr_mut$ -> ["usdl_sys::video::uSDL_Window"]; 字段布局 = unknown。
// [类型关系] usdl_sys::backend::load_symbol -> ["void (*)(ptr_mut$<usdl_sys::video::uSDL_Window>)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<void (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,i32)>
// 信息 ID: e15627d9178f3d48790112db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] void (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,i32)
// 参数类型拼写: ["ptr_mut$<usdl_sys::video::uSDL_Window>", "i32"]
// [类型关系] ptr_mut$ -> ["usdl_sys::video::uSDL_Window"]; 字段布局 = unknown。
// [类型关系] usdl_sys::backend::load_symbol -> ["void (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,i32)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<void (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,i32,i32)>
// 信息 ID: 492d3d1ff9a52aa007409137
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] void (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,i32,i32)
// 参数类型拼写: ["ptr_mut$<usdl_sys::video::uSDL_Window>", "i32", "i32"]
// [类型关系] ptr_mut$ -> ["usdl_sys::video::uSDL_Window"]; 字段布局 = unknown。
// [类型关系] usdl_sys::backend::load_symbol -> ["void (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,i32,i32)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<void (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,ptr_mut$<i32>,ptr_mut$<i32>)>
// 信息 ID: 96f400f35ffeeb7246c3934a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] void (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,ptr_mut$<i32>,ptr_mut$<i32>)
// 参数类型拼写: ["ptr_mut$<usdl_sys::video::uSDL_Window>", "ptr_mut$<i32>", "ptr_mut$<i32>"]
// [类型关系] ptr_mut$ -> ["usdl_sys::video::uSDL_Window"]; 字段布局 = unknown。
// [类型关系] ptr_mut$ -> ["i32"]; 字段布局 = unknown。
// [类型关系] ptr_mut$ -> ["i32"]; 字段布局 = unknown。
// [类型关系] usdl_sys::backend::load_symbol -> ["void (*)(ptr_mut$<usdl_sys::video::uSDL_Window>,ptr_mut$<i32>,ptr_mut$<i32>)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::backend::load_symbol<void (*)(u32)>
// 信息 ID: 9edba21bc80d05a58612ca09
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] void (*)(u32)
// 参数类型拼写: ["u32"]
// [类型关系] usdl_sys::backend::load_symbol -> ["void (*)(u32)"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cb9fb -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba3d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cba75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbaad -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbae5 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb1d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb59 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbb95 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbbd1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc0d -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc49 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbc85 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcc1 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbcfd -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd39 -> 0x1002580 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl_sys::backend::DynamicBackend::load @ 0x6cbd75 -> 0x1002580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
