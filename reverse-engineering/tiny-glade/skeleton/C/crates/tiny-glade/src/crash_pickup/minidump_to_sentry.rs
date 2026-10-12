// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/crash_pickup/minidump_to_sentry.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 4a7e0a9327873b16e19e69836684b2cb8c032ea9a6f2bea98344ea6d2016d1b6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: minidump_to_sentry.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 8, 187
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use core; // namespace candidate
//   package identities: []
// use futures_lite; // namespace candidate
//   package identities: [{"name":"futures-lite","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use minidump; // namespace candidate
//   package identities: [{"name":"minidump","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.21.1"}]
// use minidump_unwind; // namespace candidate
//   package identities: [{"name":"minidump-unwind","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.21.1"}]
// use sentry_types; // namespace candidate
//   package identities: [{"name":"sentry-types","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.31.7"}]
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":187,"record_offset":1896,"rva":5473792,"va":5374182912}
// 编译实例 RVAs: 0x538600
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10475620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e864d89f00","size":5,"soft_fail":false,"target_va":5384659264,"va":5374183639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t1494975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e8bfcf1600","size":5,"soft_fail":false,"target_va":5375678704,"va":5374183724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007e380
//     owner=unresolved_namespace; count=2
//     target_aliases=39 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4959086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e89254b4ff","size":5,"soft_fail":false,"target_va":5369226112,"va":5374185193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a5470
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4797932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e814cab6ff","size":5,"soft_fail":false,"target_va":5369386096,"va":5374184023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a9980
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4779917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e87310b7ff","size":5,"soft_fail":false,"target_va":5369403776,"va":5374183688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a9ad0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4781160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e8980bb7ff","size":5,"soft_fail":false,"target_va":5369404112,"va":5374185267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ac7a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4769245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e8233ab7ff","size":5,"soft_fail":false,"target_va":5369415584,"va":5374184824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b4fe0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4734821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e89bc0b7ff","size":5,"soft_fail":false,"target_va":5369450464,"va":5374185280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b56c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4733627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e845c5b7ff","size":5,"soft_fail":false,"target_va":5369452224,"va":5374185846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400bc2e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4706393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e8a72fb8ff","size":5,"soft_fail":false,"target_va":5369479904,"va":5374186292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400bfe80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4690555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e8856db8ff","size":5,"soft_fail":false,"target_va":5369495168,"va":5374185718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402faa60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2352077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e8331cdcff","size":5,"soft_fail":false,"target_va":5371832928,"va":5374185000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404057e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1258709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e82bcbecff","size":5,"soft_fail":false,"target_va":5372925920,"va":5374184624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140405880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1258260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e8ecccecff","size":5,"soft_fail":false,"target_va":5372926080,"va":5374184335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405188f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-130678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e88a01feff","size":5,"soft_fail":false,"target_va":5374052592,"va":5374183265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140518a30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-130992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e85000feff","size":5,"soft_fail":false,"target_va":5374052912,"va":5374183899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140518b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-129819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e8e504feff","size":5,"soft_fail":false,"target_va":5374053248,"va":5374183062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140518cd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-129747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e82d05feff","size":5,"soft_fail":false,"target_va":5374053584,"va":5374183326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140518e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-129537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e8ff05feff","size":5,"soft_fail":false,"target_va":5374053920,"va":5374183452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140518f60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-129622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e8aa05feff","size":5,"soft_fail":false,"target_va":5374054240,"va":5374183857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405190b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-128409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e8670afeff","size":5,"soft_fail":false,"target_va":5374054576,"va":5374182980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d65fb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42129818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e89ad98202","size":5,"soft_fail":false,"target_va":5416312752,"va":5374182929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d672b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42133947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e8bbe98202","size":5,"soft_fail":false,"target_va":5416317616,"va":5374183664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d71740
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42176629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e875908302","size":5,"soft_fail":false,"target_va":5416359744,"va":5374183110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42874022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e8a6348e02","size":5,"soft_fail":false,"target_va":5417057056,"va":5374183029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e88ef9ffff","size":5,"soft_fail":false,"target_va":5374182816,"va":5374184461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<minidump::minidump::UnifiedMemoryList> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t13250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e8c2330000","size":5,"soft_fail":false,"target_va":5374198496,"va":5374185241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] futures_lite::future::block_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"futures-lite","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-3914540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e8d444c4ff","size":5,"soft_fail":false,"target_va":5370269920,"va":5374184455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump::context::MinidumpContext::get_stack_pointer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"minidump","version":"0.21.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e8db3a0000","size":5,"soft_fail":false,"target_va":5374199328,"va":5374184256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump::minidump::MinidumpException::context
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"minidump","version":"0.21.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2779040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e8a0672a00","size":5,"soft_fail":false,"target_va":5376962656,"va":5374183611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump::minidump::MinidumpException::get_crash_reason
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"minidump","version":"0.21.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2777546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e8ca612a00","size":5,"soft_fail":false,"target_va":5376962496,"va":5374184945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump::minidump::MinidumpSystemInfo::cpu_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"minidump","version":"0.21.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2778658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e822662a00","size":5,"soft_fail":false,"target_va":5376963232,"va":5374184569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump::minidump::UnifiedMemoryList::memory_at_address
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"minidump","version":"0.21.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2778501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e885652a00","size":5,"soft_fail":false,"target_va":5376962784,"va":5374184278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump_unwind::CallStack::with_context
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2204337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e8b1a22100","size":5,"soft_fail":false,"target_va":5376388704,"va":5374184362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump_unwind::symbols::MultiSymbolProvider::add
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2167837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e81d142100","size":5,"soft_fail":false,"target_va":5376352368,"va":5374184526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] sentry_types::protocol::v7::Stacktrace::from_frames_reversed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"sentry-types","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1940280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182912,"raw_hex":"e8389b1d00","size":5,"soft_fail":false,"target_va":5376125312,"va":5374185027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::borrow::impl$4::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":187,"record_offset":3416,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":6483,"source_line":349,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":187,"record_offset":3528,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":3656,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":187,"record_offset":2116,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$1::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":187,"record_offset":2392,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":10143,"source_line":91,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":187,"record_offset":3768,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":187,"record_offset":3712,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":187,"record_offset":2364,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":187,"record_offset":3564,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":187,"record_offset":2504,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":187,"record_offset":3160,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":187,"record_offset":2228,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":187,"record_offset":3196,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":15644,"source_line":1559,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":187,"record_offset":2200,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] minidump_unwind::symbols::MultiSymbolProvider::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/minidump-unwind-0.21.1/src/symbols/mod.rs
//     evidence={"module":187,"record_offset":3444,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":15647,"source_line":154,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] minidump_unwind::symbols::debuginfo::DebugInfoSymbolProviderBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/minidump-unwind-0.21.1/src/symbols/debuginfo.rs
//     evidence={"module":187,"record_offset":3500,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":15651,"source_line":406,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] minidump_unwind::walk_stack
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/minidump-unwind-0.21.1/src/lib.rs
//     evidence={"module":187,"record_offset":3628,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":15654,"source_line":725,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":2728,"parent_record_offset":2704,"ancestor_record_offset":1896,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":1896,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":1896,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":2704,"parent_record_offset":2648,"ancestor_record_offset":1896,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":187,"record_offset":2676,"parent_record_offset":2648,"ancestor_record_offset":1896,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":187,"record_offset":2648,"parent_record_offset":2624,"ancestor_record_offset":1896,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":187,"record_offset":2624,"parent_record_offset":2600,"ancestor_record_offset":1896,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":3792,"parent_record_offset":3768,"ancestor_record_offset":1896,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_display
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":187,"record_offset":2444,"parent_record_offset":2392,"ancestor_record_offset":1896,"inlinee":6351,"source_line":212,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":187,"record_offset":3736,"parent_record_offset":3712,"ancestor_record_offset":1896,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":187,"record_offset":2528,"parent_record_offset":2504,"ancestor_record_offset":1896,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":187,"record_offset":3680,"parent_record_offset":3656,"ancestor_record_offset":1896,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":187,"record_offset":3252,"parent_record_offset":3224,"ancestor_record_offset":1896,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] minidump_unwind::symbols::impl$1::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/minidump-unwind-0.21.1/src/symbols/mod.rs
//     evidence={"module":187,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":1896,"inlinee":15649,"source_line":148,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/minidump_to_sentry.rs
//     evidence={"module":187,"record_offset":3224,"parent_record_offset":3196,"ancestor_record_offset":1896,"inlinee":15645,"source_line":78,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/minidump_to_sentry.rs
//     evidence={"module":187,"record_offset":3596,"parent_record_offset":3564,"ancestor_record_offset":1896,"inlinee":15652,"source_line":101,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::minidump_to_sentry::crash_trace_to_sentry_event
// 记录形式: compiled_procedure; instances=1
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":187,"record_offset":4480,"rva":5477216,"va":5374186336}
// 编译实例 RVAs: 0x539360
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$minidump..minidump..CrashReason$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2686814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374186336,"raw_hex":"e85eff2800","size":5,"soft_fail":false,"target_va":5376875072,"va":5374188253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$sentry_types..protocol..v7..Event$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1940178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374186336,"raw_hex":"e8d29a1d00","size":5,"soft_fail":false,"target_va":5376127312,"va":5374187129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t1492268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374186336,"raw_hex":"e82cc51600","size":5,"soft_fail":false,"target_va":5375678688,"va":5374186415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=20
//     evidence={"assembly":"\tcallq\t1488763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374186336,"raw_hex":"e87bb71600","size":5,"soft_fail":false,"target_va":5375678704,"va":5374189936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=6
//     target_aliases=1699 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5464213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374186336,"raw_hex":"e86b9facff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374186400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007df10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4966225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374186336,"raw_hex":"e8af38b4ff","size":5,"soft_fail":false,"target_va":5369224976,"va":5374191196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a8660
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4789226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374186336,"raw_hex":"e816ecb6ff","size":5,"soft_fail":false,"target_va":5369398880,"va":5374188101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a87f0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4790749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374186336,"raw_hex":"e823e6b6ff","size":5,"soft_fail":false,"target_va":5369399280,"va":5374190024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400aad60
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4781306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374186336,"raw_hex":"e8060bb7ff","size":5,"soft_fail":false,"target_va":5369408864,"va":5374190165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42870017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374186336,"raw_hex":"e801258e02","size":5,"soft_fail":false,"target_va":5417057056,"va":5374187034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42430529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374186336,"raw_hex":"e841708702","size":5,"soft_fail":false,"target_va":5416620769,"va":5374190235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10608195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374186336,"raw_hex":"e843dea100","size":5,"soft_fail":false,"target_va":5384794864,"va":5374186664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t42430462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374186336,"raw_hex":"e8fe6f8702","size":5,"soft_fail":false,"target_va":5416620794,"va":5374190327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10434038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374186336,"raw_hex":"e8f6359f00","size":5,"soft_fail":false,"target_va":5384624080,"va":5374190037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42431343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374186336,"raw_hex":"e86f738702","size":5,"soft_fail":false,"target_va":5416621632,"va":5374190284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":187,"record_offset":4664,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":187,"record_offset":5908,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":187,"record_offset":6760,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":187,"record_offset":4844,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":187,"record_offset":5992,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":187,"record_offset":6104,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] sentry_types::protocol::v7::impl$172::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"sentry-types","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-types-0.31.7/src/protocol/v7.rs
//     evidence={"module":187,"record_offset":6020,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":10152,"source_line":619,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":4748,"parent_record_offset":4724,"ancestor_record_offset":4480,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":4772,"parent_record_offset":4748,"ancestor_record_offset":4480,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":6392,"parent_record_offset":6368,"ancestor_record_offset":4480,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":6416,"parent_record_offset":6392,"ancestor_record_offset":4480,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":4796,"parent_record_offset":4772,"ancestor_record_offset":4480,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":6440,"parent_record_offset":6416,"ancestor_record_offset":4480,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":4724,"parent_record_offset":4692,"ancestor_record_offset":4480,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":6368,"parent_record_offset":6280,"ancestor_record_offset":4480,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":187,"record_offset":4692,"parent_record_offset":4664,"ancestor_record_offset":4480,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":187,"record_offset":10548,"parent_record_offset":10524,"ancestor_record_offset":4480,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":187,"record_offset":5956,"parent_record_offset":5932,"ancestor_record_offset":4480,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":187,"record_offset":5076,"parent_record_offset":5048,"ancestor_record_offset":4480,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":187,"record_offset":6308,"parent_record_offset":6280,"ancestor_record_offset":4480,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":187,"record_offset":6280,"parent_record_offset":6256,"ancestor_record_offset":4480,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":187,"record_offset":5136,"parent_record_offset":5104,"ancestor_record_offset":4480,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":187,"record_offset":5104,"parent_record_offset":5076,"ancestor_record_offset":4480,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":187,"record_offset":6256,"parent_record_offset":6232,"ancestor_record_offset":4480,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":187,"record_offset":4956,"parent_record_offset":4928,"ancestor_record_offset":4480,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":187,"record_offset":4984,"parent_record_offset":4956,"ancestor_record_offset":4480,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":187,"record_offset":5012,"parent_record_offset":4984,"ancestor_record_offset":4480,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":187,"record_offset":4928,"parent_record_offset":4900,"ancestor_record_offset":4480,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":187,"record_offset":4900,"parent_record_offset":4872,"ancestor_record_offset":4480,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":187,"record_offset":6848,"parent_record_offset":6796,"ancestor_record_offset":4480,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":187,"record_offset":6796,"parent_record_offset":6760,"ancestor_record_offset":4480,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":187,"record_offset":4872,"parent_record_offset":4844,"ancestor_record_offset":4480,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":5048,"parent_record_offset":5012,"ancestor_record_offset":4480,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":6168,"parent_record_offset":6136,"ancestor_record_offset":4480,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":187,"record_offset":6876,"parent_record_offset":6796,"ancestor_record_offset":4480,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":187,"record_offset":6336,"parent_record_offset":6308,"ancestor_record_offset":4480,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":187,"record_offset":5932,"parent_record_offset":5908,"ancestor_record_offset":4480,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":187,"record_offset":5292,"parent_record_offset":5012,"ancestor_record_offset":4480,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":187,"record_offset":5316,"parent_record_offset":5292,"ancestor_record_offset":4480,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":187,"record_offset":6136,"parent_record_offset":6104,"ancestor_record_offset":4480,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":187,"record_offset":6904,"parent_record_offset":6796,"ancestor_record_offset":4480,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] sentry_types::protocol::v7::impl$2::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"sentry-types","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-types-0.31.7/src/protocol/v7.rs
//     evidence={"module":187,"record_offset":6072,"parent_record_offset":6048,"ancestor_record_offset":4480,"inlinee":10154,"source_line":84,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D1D
// 首条位置: {"module":187,"inlinee":15645}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":187,"record_offset":3252,"parent_record_offset":3224,"ancestor_record_offset":3224,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00187.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D24
// 首条位置: {"module":187,"inlinee":15652}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1516
// 首条位置: {"module":8,"inlinee":5398}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":8,"record_offset":1344,"parent_record_offset":1288,"ancestor_record_offset":1288,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":8,"record_offset":1372,"parent_record_offset":1288,"ancestor_record_offset":1288,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":8,"record_offset":1436,"parent_record_offset":1288,"ancestor_record_offset":1288,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":8,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":1288,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":8,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":1288,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":8,"record_offset":1712,"parent_record_offset":1688,"ancestor_record_offset":1288,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":8,"record_offset":1640,"parent_record_offset":1584,"ancestor_record_offset":1288,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":8,"record_offset":1612,"parent_record_offset":1584,"ancestor_record_offset":1288,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":8,"record_offset":1584,"parent_record_offset":1560,"ancestor_record_offset":1288,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":8,"record_offset":1560,"parent_record_offset":1536,"ancestor_record_offset":1288,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":8,"record_offset":2032,"parent_record_offset":2008,"ancestor_record_offset":1288,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":8,"record_offset":3320,"parent_record_offset":3292,"ancestor_record_offset":1288,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":8,"record_offset":3348,"parent_record_offset":3320,"ancestor_record_offset":1288,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":8,"record_offset":2844,"parent_record_offset":2816,"ancestor_record_offset":1288,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":8,"record_offset":1464,"parent_record_offset":1436,"ancestor_record_offset":1288,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":8,"record_offset":3384,"parent_record_offset":3260,"ancestor_record_offset":1288,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":8,"record_offset":3260,"parent_record_offset":3228,"ancestor_record_offset":1288,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":8,"record_offset":3172,"parent_record_offset":3148,"ancestor_record_offset":1288,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":8,"record_offset":3228,"parent_record_offset":3172,"ancestor_record_offset":1288,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":8,"record_offset":3200,"parent_record_offset":3172,"ancestor_record_offset":1288,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":8,"record_offset":3292,"parent_record_offset":3260,"ancestor_record_offset":1288,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":8,"record_offset":3148,"parent_record_offset":3124,"ancestor_record_offset":1288,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$2::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/minidump_to_sentry.rs
//     evidence={"module":8,"record_offset":1404,"parent_record_offset":1372,"ancestor_record_offset":1288,"inlinee":5400,"source_line":109,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$2::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1518
// 首条位置: {"module":8,"inlinee":5400}
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
// 完整结构及原始类型记录: minidump_to_sentry.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::minidump_to_sentry::CrashTrace
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tiny_glade::crash_pickup::minidump_to_sentry::CrashTrace,anyhow::Error>", "constructor": "core::result::Result", "arguments": ["tiny_glade::crash_pickup::minidump_to_sentry::CrashTrace", "anyhow::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tiny_glade::crash_pickup::minidump_to_sentry::CrashTrace,anyhow::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<tiny_glade::crash_pickup::minidump_to_sentry::CrashTrace,anyhow::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<enum2$<core::result::Result<tiny_glade::crash_pickup::minidump_to_sentry::CrashTrace,anyhow::Error> > >", "constructor": "core::option::Option", "arguments": ["enum2$<core::result::Result<tiny_glade::crash_pickup::minidump_to_sentry::CrashTrace,anyhow::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<enum2$<core::result::Result<tiny_glade::crash_pickup::minidump_to_sentry::CrashTrace,anyhow::Error> > > >", "constructor": "enum2$", "arguments": ["core::option::Option<enum2$<core::result::Result<tiny_glade::crash_pickup::minidump_to_sentry::CrashTrace,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<core::result::Result<tiny_glade::crash_pickup::minidump_to_sentry::CrashTrace,anyhow::Error> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<enum2$<core::result::Result<tiny_glade::crash_pickup::minidump_to_sentry::CrashTrace,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: minidump_to_sentry.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: minidump_to_sentry.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<i64,usdl::video::DisplayMode> >,ref$<tuple$<i64,usdl::video::DisplayMode> >)
// 候选参数类型: ["ref$<tuple$<i64,usdl::video::DisplayMode> >", "ref$<tuple$<i64,usdl::video::DisplayMode> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)
// 候选参数类型: ["ref$<tuple$<i64,winit::monitor::VideoMode> >", "ref$<tuple$<i64,winit::monitor::VideoMode> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::system::query::Query<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::minidump_to_sentry::crash_trace_to_sentry_event
// 信息 ID: aae56a14b7911bdb9ee707f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > > @ 0x5e8694 -> 0x539360 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace
// 信息 ID: 082dc15dac208ca8e2642921
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x5433c5 -> 0x538600 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$0
// 信息 ID: 3f42469e76d87ca766876273
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace; module=187, inline_record=3224; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$1
// 信息 ID: c27ee222d71a533410d70b13
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace; module=187, inline_record=3596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$2
// 信息 ID: aa608328ae27b05b4d685e46
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2 <- alloc::vec::into_iter::impl$6::fold<minidump_unwind::StackFrame,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<minidump_unwind::StackFrame,sentry_types::protocol::v7::Frame,tuple$<>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<sentry_types::protocol::v7::Frame,alloc::vec::impl$21::extend_trusted::closure_env$0<sentry_types::protocol::v7::Frame,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<minidump_unwind::StackFrame,alloc::alloc::Global>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2 <- alloc::vec::into_iter::impl$6::fold<minidump_unwind::StackFrame,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<minidump_unwind::StackFrame,sentry_types::protocol::v7::Frame,tuple$<>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<sentry_types::protocol::v7::Frame,alloc::vec::impl$21::extend_trusted::closure_env$0<sentry_types::protocol::v7::Frame,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<minidump_unwind::StackFrame,alloc::alloc::Global>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2 <- alloc::vec::in_place_collect::impl$1::from_iter<sentry_types::protocol::v7::Frame,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<minidump_unwind::StackFrame,alloc::alloc::Global>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$6::fold<minidump_unwind::StackFrame,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<minidump_unwind::StackFrame,sentry_types::protocol::v7::Frame,tuple$<>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<sentry_types::protocol::v7::Frame,alloc::vec::impl$21::extend_trusted::closure_env$0<sentry_types::protocol::v7::Frame,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<minidump_unwind::StackFrame,alloc::alloc::Global>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2> > > > >; module=8, inline_record=1288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$2::closure$0
// 信息 ID: 2f6ffaf18bcf3e2d44051134
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$6::fold<minidump_unwind::StackFrame,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<minidump_unwind::StackFrame,sentry_types::protocol::v7::Frame,tuple$<>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<sentry_types::protocol::v7::Frame,alloc::vec::impl$21::extend_trusted::closure_env$0<sentry_types::protocol::v7::Frame,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<minidump_unwind::StackFrame,alloc::alloc::Global>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2> > > > >; module=8, inline_record=1404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
