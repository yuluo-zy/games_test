// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/usdl-sys/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 9350c4e73fe8f4f5aaf5f4f7a72c3528c42a20e981af50a29a76b89451c16b98
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 288, 1005, 1021, 2525, 3841
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
// 源码行: 1
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
//     target_aliases=29 candidates; complete list: lib.rs.dependencies.json
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
// 函数符号: usdl_sys::backend::load_library<ref$<ref$<str$> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
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
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
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
// 函数符号: usdl_sys::uSDL_Init
// 记录形式: compiled_procedure; instances=1
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1005,"record_offset":1616,"rva":16781696,"va":5385490816}
// 编译实例 RVAs: 0x1001180
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31131346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385490816,"raw_hex":"e8d206db01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385490889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1005,"record_offset":1720,"parent_record_offset":1616,"ancestor_record_offset":1616,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1005,"record_offset":1836,"parent_record_offset":1816,"ancestor_record_offset":1616,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1005,"record_offset":1856,"parent_record_offset":1836,"ancestor_record_offset":1616,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1005,"record_offset":1796,"parent_record_offset":1776,"ancestor_record_offset":1616,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1005,"record_offset":1756,"parent_record_offset":1720,"ancestor_record_offset":1616,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1005,"record_offset":1776,"parent_record_offset":1756,"ancestor_record_offset":1616,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1005,"record_offset":1816,"parent_record_offset":1796,"ancestor_record_offset":1616,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::uSDL_InitSubSystem
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1005,"record_offset":712,"rva":16781536,"va":5385490656}
// 编译实例 RVAs: 0x10010e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31131506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385490656,"raw_hex":"e87207db01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385490729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1005,"record_offset":824,"parent_record_offset":712,"ancestor_record_offset":712,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1005,"record_offset":940,"parent_record_offset":920,"ancestor_record_offset":712,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1005,"record_offset":960,"parent_record_offset":940,"ancestor_record_offset":712,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1005,"record_offset":900,"parent_record_offset":880,"ancestor_record_offset":712,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1005,"record_offset":860,"parent_record_offset":824,"ancestor_record_offset":712,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1005,"record_offset":880,"parent_record_offset":860,"ancestor_record_offset":712,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1005,"record_offset":920,"parent_record_offset":900,"ancestor_record_offset":712,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::uSDL_Quit
// 记录形式: compiled_procedure; instances=1
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1005,"record_offset":2060,"rva":16781776,"va":5385490896}
// 编译实例 RVAs: 0x10011d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31131266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385490896,"raw_hex":"e88206db01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385490969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1005,"record_offset":2164,"parent_record_offset":2060,"ancestor_record_offset":2060,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1005,"record_offset":2280,"parent_record_offset":2260,"ancestor_record_offset":2060,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1005,"record_offset":2300,"parent_record_offset":2280,"ancestor_record_offset":2060,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1005,"record_offset":2240,"parent_record_offset":2220,"ancestor_record_offset":2060,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1005,"record_offset":2200,"parent_record_offset":2164,"ancestor_record_offset":2060,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1005,"record_offset":2220,"parent_record_offset":2200,"ancestor_record_offset":2060,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1005,"record_offset":2260,"parent_record_offset":2240,"ancestor_record_offset":2060,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::uSDL_QuitSubSystem
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1005,"record_offset":1164,"rva":16781616,"va":5385490736}
// 编译实例 RVAs: 0x1001130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31131426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385490736,"raw_hex":"e82207db01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385490809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1005,"record_offset":1276,"parent_record_offset":1164,"ancestor_record_offset":1164,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1005,"record_offset":1392,"parent_record_offset":1372,"ancestor_record_offset":1164,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1005,"record_offset":1412,"parent_record_offset":1392,"ancestor_record_offset":1164,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1005,"record_offset":1352,"parent_record_offset":1332,"ancestor_record_offset":1164,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1005,"record_offset":1312,"parent_record_offset":1276,"ancestor_record_offset":1164,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1005,"record_offset":1332,"parent_record_offset":1312,"ancestor_record_offset":1164,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1005,"record_offset":1372,"parent_record_offset":1352,"ancestor_record_offset":1164,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl_sys::uSDL_GetError
// 记录形式: compiled_procedure; instances=1
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1005,"record_offset":264,"rva":16781456,"va":5385490576}
// 编译实例 RVAs: 0x1001090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31131586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385490576,"raw_hex":"e8c207db01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385490649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl_sys::backend::active_api
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/backend.rs
//     evidence={"module":1005,"record_offset":372,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":19212,"source_line":198,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1005,"record_offset":488,"parent_record_offset":468,"ancestor_record_offset":264,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1005,"record_offset":508,"parent_record_offset":488,"ancestor_record_offset":264,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":1005,"record_offset":448,"parent_record_offset":428,"ancestor_record_offset":264,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1005,"record_offset":408,"parent_record_offset":372,"ancestor_record_offset":264,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":1005,"record_offset":428,"parent_record_offset":408,"ancestor_record_offset":264,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/01005.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":1005,"record_offset":468,"parent_record_offset":448,"ancestor_record_offset":264,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/01005.jsonl.gz"}
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
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
// 函数: usdl_sys::uSDL_GetError
// 信息 ID: 99fc800e798c9587b4449821
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::sdl::get_error @ 0x6ca45e -> 0x1001090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::uSDL_Init
// 信息 ID: 03b2eaf722ead080570dc295
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::sdl::init @ 0x6ca391 -> 0x1001180 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::uSDL_InitSubSystem
// 信息 ID: 2e938007ffb74bba37d2c3a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::sdl::Sdl::video @ 0x6ca2d4 -> 0x10010e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::uSDL_Quit
// 信息 ID: 8f0a6921b739d5815962af0c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::sdl::Sdl::video @ 0x6ca30a -> 0x10011d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl_sys::uSDL_QuitSubSystem
// 信息 ID: 1d38cc902f5d93fcb72f885b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::video::impl$1::drop @ 0x6cb7be -> 0x1001130 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
