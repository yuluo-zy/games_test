// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/crash_pickup/minidump_to_sentry.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 4a7e0a9327873b16e19e69836684b2cb8c032ea9a6f2bea98344ea6d2016d1b6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: minidump_to_sentry.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 175, 190
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use futures_lite; // namespace candidate
//   package identities: [{"name":"futures-lite","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use minidump; // namespace candidate
//   package identities: [{"name":"minidump","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.21.1"}]
// use minidump_unwind; // namespace candidate
//   package identities: [{"name":"minidump-unwind","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.21.1"}]
// use sentry_types; // namespace candidate
//   package identities: [{"name":"sentry-types","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.31.7"}]
// use std; // namespace candidate
//   package identities: []
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
// 首条位置: {"module":190,"record_offset":21832,"rva":4927408,"va":5373636528}
// 编译实例 RVAs: 0x4b2fb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b6960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2084750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e87230e0ff","size":5,"soft_fail":false,"target_va":5371554144,"va":5373638889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b25d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1051510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e88af4efff","size":5,"soft_fail":false,"target_va":5372585424,"va":5373636929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b2750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1051371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e815f5efff","size":5,"soft_fail":false,"target_va":5372585808,"va":5373637174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b28d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1051448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e8c8f4efff","size":5,"soft_fail":false,"target_va":5372586192,"va":5373637635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b2a50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1050110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e802faefff","size":5,"soft_fail":false,"target_va":5372586576,"va":5373636681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b2bd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1050725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e89bf7efff","size":5,"soft_fail":false,"target_va":5372586960,"va":5373637680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b2d50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1049652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e8ccfbefff","size":5,"soft_fail":false,"target_va":5372587344,"va":5373636991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b3060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1048476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e86400f0ff","size":5,"soft_fail":false,"target_va":5372588128,"va":5373636599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bba70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1082447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e84f841000","size":5,"soft_fail":false,"target_va":5374720624,"va":5373638172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405cfe90
//     owner=unresolved_namespace; count=2
//     target_aliases=40 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1164515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e8e3c41100","size":5,"soft_fail":false,"target_va":5374803600,"va":5373639080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f0e00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1300779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e82bd91300","size":5,"soft_fail":false,"target_va":5374938624,"va":5373637840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f4270
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1314577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e8110f1400","size":5,"soft_fail":false,"target_va":5374952048,"va":5373637466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f43b0
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1313212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e8bc091400","size":5,"soft_fail":false,"target_va":5374952368,"va":5373639151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f6310
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1321687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e8d72a1400","size":5,"soft_fail":false,"target_va":5374960400,"va":5373638708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fce30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1348656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e830941400","size":5,"soft_fail":false,"target_va":5374987824,"va":5373639163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fd7d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1350453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e8359b1400","size":5,"soft_fail":false,"target_va":5374990288,"va":5373639830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140603c60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1375831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e857fe1400","size":5,"soft_fail":false,"target_va":5375016032,"va":5373640196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140607800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1391589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e8e53b1500","size":5,"soft_fail":false,"target_va":5375031296,"va":5373639702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1443998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e89e081600","size":5,"soft_fail":false,"target_va":5375081504,"va":5373637501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14289aa40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37649018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e87a7a3e02","size":5,"soft_fail":false,"target_va":5411285568,"va":5373636545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a3890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37684552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e848053f02","size":5,"soft_fail":false,"target_va":5411322000,"va":5373637443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a3a50
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37685668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e8a4093f02","size":5,"soft_fail":false,"target_va":5411322448,"va":5373636775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38210515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e8d30b4702","size":5,"soft_fail":false,"target_va":5411847168,"va":5373636648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e86de0ffff","size":5,"soft_fail":false,"target_va":5373630272,"va":5373638350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::borrow::Cow$LT$B$GT$::into_owned
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2494691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e81defd9ff","size":5,"soft_fail":false,"target_va":5371143824,"va":5373638510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<minidump::minidump::UnifiedMemoryList> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e824dbffff","size":5,"soft_fail":false,"target_va":5373629696,"va":5373639127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] futures_lite::future::block_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"futures-lite","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t598738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e8d2220900","size":5,"soft_fail":false,"target_va":5374237088,"va":5373638345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump::context::MinidumpContext::get_stack_pointer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"minidump","version":"0.21.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-7776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e8a0e1ffff","size":5,"soft_fail":false,"target_va":5373630336,"va":5373638107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump::minidump::MinidumpException::context
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"minidump","version":"0.21.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2674445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e80dcf2800","size":5,"soft_fail":false,"target_va":5376311840,"va":5373637390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump::minidump::MinidumpException::get_crash_reason
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"minidump","version":"0.21.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2673135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e8efc92800","size":5,"soft_fail":false,"target_va":5376311968,"va":5373638828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump::minidump::MinidumpSystemInfo::cpu_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"minidump","version":"0.21.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2664435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e8f3a72800","size":5,"soft_fail":false,"target_va":5376302896,"va":5373638456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump::minidump::UnifiedMemoryList::memory_at_address
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"minidump","version":"0.21.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2656762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e8fa892800","size":5,"soft_fail":false,"target_va":5376294896,"va":5373638129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump_unwind::CallStack::with_context
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2082817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e801c81f00","size":5,"soft_fail":false,"target_va":5375721072,"va":5373638250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump_unwind::symbols::MultiSymbolProvider::add
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2051725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e88d4e1f00","size":5,"soft_fail":false,"target_va":5375690144,"va":5373638414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] sentry_types::protocol::v7::Stacktrace::from_frames_reversed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"sentry-types","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1824728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e8d8d71b00","size":5,"soft_fail":false,"target_va":5375463648,"va":5373638915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t9057917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373636528,"raw_hex":"e87d368a00","size":5,"soft_fail":false,"target_va":5382694656,"va":5373636734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::borrow::Cow::into_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\borrow.rs
//     evidence={"module":190,"record_offset":23616,"parent_record_offset":21832,"ancestor_record_offset":21832,"inlinee":10890,"source_line":320,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] alloc::borrow::impl$4::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\borrow.rs
//     evidence={"module":190,"record_offset":23588,"parent_record_offset":21832,"ancestor_record_offset":21832,"inlinee":10898,"source_line":335,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":190,"record_offset":23736,"parent_record_offset":21832,"ancestor_record_offset":21832,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":190,"record_offset":23864,"parent_record_offset":21832,"ancestor_record_offset":21832,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":190,"record_offset":22056,"parent_record_offset":21832,"ancestor_record_offset":21832,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$1::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":190,"record_offset":22604,"parent_record_offset":21832,"ancestor_record_offset":21832,"inlinee":12564,"source_line":91,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":190,"record_offset":23976,"parent_record_offset":21832,"ancestor_record_offset":21832,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":190,"record_offset":23920,"parent_record_offset":21832,"ancestor_record_offset":21832,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":190,"record_offset":22576,"parent_record_offset":21832,"ancestor_record_offset":21832,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":190,"record_offset":23772,"parent_record_offset":21832,"ancestor_record_offset":21832,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":190,"record_offset":22716,"parent_record_offset":21832,"ancestor_record_offset":21832,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":190,"record_offset":23260,"parent_record_offset":21832,"ancestor_record_offset":21832,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":190,"record_offset":22304,"parent_record_offset":21832,"ancestor_record_offset":21832,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":190,"record_offset":23296,"parent_record_offset":21832,"ancestor_record_offset":21832,"inlinee":14342,"source_line":1412,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":190,"record_offset":22276,"parent_record_offset":21832,"ancestor_record_offset":21832,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] minidump_unwind::symbols::MultiSymbolProvider::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/minidump-unwind-0.21.1/src/symbols/mod.rs
//     evidence={"module":190,"record_offset":23652,"parent_record_offset":21832,"ancestor_record_offset":21832,"inlinee":14345,"source_line":154,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] minidump_unwind::symbols::debuginfo::DebugInfoSymbolProviderBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/minidump-unwind-0.21.1/src/symbols/debuginfo.rs
//     evidence={"module":190,"record_offset":23708,"parent_record_offset":21832,"ancestor_record_offset":21832,"inlinee":14349,"source_line":406,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] minidump_unwind::walk_stack
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/minidump-unwind-0.21.1/src/lib.rs
//     evidence={"module":190,"record_offset":23836,"parent_record_offset":21832,"ancestor_record_offset":21832,"inlinee":14352,"source_line":725,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":22940,"parent_record_offset":22916,"ancestor_record_offset":21832,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":22916,"parent_record_offset":22860,"ancestor_record_offset":21832,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":22888,"parent_record_offset":22860,"ancestor_record_offset":21832,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":22860,"parent_record_offset":22836,"ancestor_record_offset":21832,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":22836,"parent_record_offset":22812,"ancestor_record_offset":21832,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":190,"record_offset":24000,"parent_record_offset":23976,"ancestor_record_offset":21832,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":190,"record_offset":22128,"parent_record_offset":22096,"ancestor_record_offset":21832,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_display
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":190,"record_offset":22656,"parent_record_offset":22604,"ancestor_record_offset":21832,"inlinee":12565,"source_line":143,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":190,"record_offset":22096,"parent_record_offset":22056,"ancestor_record_offset":21832,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":190,"record_offset":23944,"parent_record_offset":23920,"ancestor_record_offset":21832,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":190,"record_offset":22740,"parent_record_offset":22716,"ancestor_record_offset":21832,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":190,"record_offset":23888,"parent_record_offset":23864,"ancestor_record_offset":21832,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":190,"record_offset":23352,"parent_record_offset":23324,"ancestor_record_offset":21832,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] minidump_unwind::symbols::impl$1::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/minidump-unwind-0.21.1/src/symbols/mod.rs
//     evidence={"module":190,"record_offset":23676,"parent_record_offset":23652,"ancestor_record_offset":21832,"inlinee":14347,"source_line":148,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/minidump_to_sentry.rs
//     evidence={"module":190,"record_offset":23324,"parent_record_offset":23296,"ancestor_record_offset":21832,"inlinee":14343,"source_line":78,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/minidump_to_sentry.rs
//     evidence={"module":190,"record_offset":23804,"parent_record_offset":23772,"ancestor_record_offset":21832,"inlinee":14350,"source_line":101,"archive":"_pdb/B/modules/00190.jsonl.gz"}
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
// 首条位置: {"module":190,"record_offset":14096,"rva":4921296,"va":5373630416}
// 编译实例 RVAs: 0x4b17d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$minidump..minidump..CrashReason$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2672674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373630416,"raw_hex":"e822c82800","size":5,"soft_fail":false,"target_va":5376305168,"va":5373632489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$sentry_types..protocol..v7..Event$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1837769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373630416,"raw_hex":"e8c90a1c00","size":5,"soft_fail":false,"target_va":5375469104,"va":5373631330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405cfaf0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1167183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373630416,"raw_hex":"e84fcf1100","size":5,"soft_fail":false,"target_va":5374802672,"va":5373635484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f3380
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1315910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373630416,"raw_hex":"e846141400","size":5,"soft_fail":false,"target_va":5374948224,"va":5373632309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f3520
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1314356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373630416,"raw_hex":"e8340e1400","size":5,"soft_fail":false,"target_va":5374948640,"va":5373634279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f4de0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1320552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373630416,"raw_hex":"e868261400","size":5,"soft_fail":false,"target_va":5374954976,"va":5373634419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1450989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373630416,"raw_hex":"e8ed231600","size":5,"soft_fail":false,"target_va":5375081488,"va":5373630494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=20
//     target_aliases=1 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1447308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373630416,"raw_hex":"e88c151600","size":5,"soft_fail":false,"target_va":5375081504,"va":5373634191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38215928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373630416,"raw_hex":"e8f8204702","size":5,"soft_fail":false,"target_va":5411847168,"va":5373631235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37906500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373630416,"raw_hex":"e844684202","size":5,"soft_fail":false,"target_va":5411540992,"va":5373634487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9209331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373630416,"raw_hex":"e8f3858c00","size":5,"soft_fail":false,"target_va":5382840208,"va":5373630872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t37906444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373630416,"raw_hex":"e80c684202","size":5,"soft_fail":false,"target_va":5411540963,"va":5373634514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9020728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373630416,"raw_hex":"e838a58900","size":5,"soft_fail":false,"target_va":5382655024,"va":5373634291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37908313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373630416,"raw_hex":"e8596f4202","size":5,"soft_fail":false,"target_va":5411542880,"va":5373634562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":14284,"parent_record_offset":14096,"ancestor_record_offset":14096,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":190,"record_offset":19592,"parent_record_offset":14096,"ancestor_record_offset":14096,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":190,"record_offset":15504,"parent_record_offset":14096,"ancestor_record_offset":14096,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":190,"record_offset":16244,"parent_record_offset":14096,"ancestor_record_offset":14096,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":190,"record_offset":14432,"parent_record_offset":14096,"ancestor_record_offset":14096,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":190,"record_offset":15588,"parent_record_offset":14096,"ancestor_record_offset":14096,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":190,"record_offset":15700,"parent_record_offset":14096,"ancestor_record_offset":14096,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] sentry_types::protocol::v7::impl$170::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"sentry-types","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-types-0.31.7/src/protocol/v7.rs
//     evidence={"module":190,"record_offset":15616,"parent_record_offset":14096,"ancestor_record_offset":14096,"inlinee":8120,"source_line":619,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":14340,"parent_record_offset":14316,"ancestor_record_offset":14096,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":14364,"parent_record_offset":14340,"ancestor_record_offset":14096,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":15988,"parent_record_offset":15964,"ancestor_record_offset":14096,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":14316,"parent_record_offset":14284,"ancestor_record_offset":14096,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":15964,"parent_record_offset":15876,"ancestor_record_offset":14096,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":190,"record_offset":15552,"parent_record_offset":15528,"ancestor_record_offset":14096,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":14692,"parent_record_offset":14664,"ancestor_record_offset":14096,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":15904,"parent_record_offset":15876,"ancestor_record_offset":14096,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":15876,"parent_record_offset":15852,"ancestor_record_offset":14096,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":14752,"parent_record_offset":14720,"ancestor_record_offset":14096,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":14720,"parent_record_offset":14692,"ancestor_record_offset":14096,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":15852,"parent_record_offset":15828,"ancestor_record_offset":14096,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":14628,"parent_record_offset":14600,"ancestor_record_offset":14096,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":14600,"parent_record_offset":14572,"ancestor_record_offset":14096,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":14544,"parent_record_offset":14516,"ancestor_record_offset":14096,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":14572,"parent_record_offset":14544,"ancestor_record_offset":14096,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":14516,"parent_record_offset":14488,"ancestor_record_offset":14096,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":190,"record_offset":14488,"parent_record_offset":14460,"ancestor_record_offset":14096,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":190,"record_offset":16332,"parent_record_offset":16280,"ancestor_record_offset":14096,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":190,"record_offset":16280,"parent_record_offset":16244,"ancestor_record_offset":14096,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":190,"record_offset":14460,"parent_record_offset":14432,"ancestor_record_offset":14096,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":190,"record_offset":14664,"parent_record_offset":14628,"ancestor_record_offset":14096,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":190,"record_offset":15764,"parent_record_offset":15732,"ancestor_record_offset":14096,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":190,"record_offset":16360,"parent_record_offset":16280,"ancestor_record_offset":14096,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":190,"record_offset":14904,"parent_record_offset":14880,"ancestor_record_offset":14096,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":190,"record_offset":15932,"parent_record_offset":15904,"ancestor_record_offset":14096,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":190,"record_offset":15528,"parent_record_offset":15504,"ancestor_record_offset":14096,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":190,"record_offset":14880,"parent_record_offset":14628,"ancestor_record_offset":14096,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":190,"record_offset":15732,"parent_record_offset":15700,"ancestor_record_offset":14096,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":190,"record_offset":14388,"parent_record_offset":14364,"ancestor_record_offset":14096,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":190,"record_offset":16388,"parent_record_offset":16280,"ancestor_record_offset":14096,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] sentry_types::protocol::v7::impl$2::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"sentry-types","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-types-0.31.7/src/protocol/v7.rs
//     evidence={"module":190,"record_offset":15668,"parent_record_offset":15644,"ancestor_record_offset":14096,"inlinee":8122,"source_line":84,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3807
// 首条位置: {"module":190,"inlinee":14343}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":190,"record_offset":23352,"parent_record_offset":23324,"ancestor_record_offset":23324,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x380E
// 首条位置: {"module":190,"inlinee":14350}
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
// IPI IDs: 0x36DD
// 首条位置: {"module":175,"inlinee":14045}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":175,"record_offset":5412,"parent_record_offset":5356,"ancestor_record_offset":5356,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":175,"record_offset":5440,"parent_record_offset":5356,"ancestor_record_offset":5356,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":175,"record_offset":5504,"parent_record_offset":5356,"ancestor_record_offset":5356,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":175,"record_offset":5732,"parent_record_offset":5708,"ancestor_record_offset":5356,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":175,"record_offset":5708,"parent_record_offset":5652,"ancestor_record_offset":5356,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":175,"record_offset":5680,"parent_record_offset":5652,"ancestor_record_offset":5356,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":175,"record_offset":5652,"parent_record_offset":5628,"ancestor_record_offset":5356,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":175,"record_offset":5628,"parent_record_offset":5604,"ancestor_record_offset":5356,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":175,"record_offset":5988,"parent_record_offset":5964,"ancestor_record_offset":5356,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":175,"record_offset":7104,"parent_record_offset":7080,"ancestor_record_offset":5356,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":175,"record_offset":6688,"parent_record_offset":6660,"ancestor_record_offset":5356,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":175,"record_offset":5532,"parent_record_offset":5504,"ancestor_record_offset":5356,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":175,"record_offset":7136,"parent_record_offset":7048,"ancestor_record_offset":5356,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":175,"record_offset":7048,"parent_record_offset":7016,"ancestor_record_offset":5356,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":175,"record_offset":6960,"parent_record_offset":6936,"ancestor_record_offset":5356,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":175,"record_offset":7016,"parent_record_offset":6960,"ancestor_record_offset":5356,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":175,"record_offset":6988,"parent_record_offset":6960,"ancestor_record_offset":5356,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":175,"record_offset":7080,"parent_record_offset":7048,"ancestor_record_offset":5356,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":175,"record_offset":6936,"parent_record_offset":6912,"ancestor_record_offset":5356,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$2::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/minidump_to_sentry.rs
//     evidence={"module":175,"record_offset":5472,"parent_record_offset":5440,"ancestor_record_offset":5356,"inlinee":14047,"source_line":109,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$2::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x36DF
// 首条位置: {"module":175,"inlinee":14047}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)
// 候选参数类型: ["ref$<tuple$<i64,winit::monitor::VideoMode> >", "ref$<tuple$<i64,winit::monitor::VideoMode> >"]

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::minidump_to_sentry::crash_trace_to_sentry_event
// 信息 ID: ed33d992b6091909ae908d7e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > > @ 0xd902f -> 0x4b17d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace
// 信息 ID: 48e1ea881c2a272127d80122
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x3affb4 -> 0x4b2fb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$0
// 信息 ID: db3406c16a005d182d3c800a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace; module=190, inline_record=23324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$1
// 信息 ID: 8909b10e363b54f95036a621
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace; module=190, inline_record=23804; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$2
// 信息 ID: 6a48b3dc0dfc781d9f8e8e25
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2 <- alloc::vec::in_place_collect::impl$1::from_iter<sentry_types::protocol::v7::Frame,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<minidump_unwind::StackFrame,alloc::alloc::Global>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2 <- alloc::vec::into_iter::impl$5::fold<minidump_unwind::StackFrame,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<minidump_unwind::StackFrame,sentry_types::protocol::v7::Frame,tuple$<>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<sentry_types::protocol::v7::Frame,alloc::vec::impl$20::extend_trusted::closure_env$0<sentry_types::protocol::v7::Frame,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<minidump_unwind::StackFrame,alloc::alloc::Global>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2 <- alloc::vec::into_iter::impl$5::fold<minidump_unwind::StackFrame,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<minidump_unwind::StackFrame,sentry_types::protocol::v7::Frame,tuple$<>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<sentry_types::protocol::v7::Frame,alloc::vec::impl$20::extend_trusted::closure_env$0<sentry_types::protocol::v7::Frame,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<minidump_unwind::StackFrame,alloc::alloc::Global>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$5::fold<minidump_unwind::StackFrame,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<minidump_unwind::StackFrame,sentry_types::protocol::v7::Frame,tuple$<>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<sentry_types::protocol::v7::Frame,alloc::vec::impl$20::extend_trusted::closure_env$0<sentry_types::protocol::v7::Frame,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<minidump_unwind::StackFrame,alloc::alloc::Global>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2> > > > >; module=175, inline_record=5356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$2::closure$0
// 信息 ID: f33fb3ba5c88c2323e80b640
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$5::fold<minidump_unwind::StackFrame,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<minidump_unwind::StackFrame,sentry_types::protocol::v7::Frame,tuple$<>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<sentry_types::protocol::v7::Frame,alloc::vec::impl$20::extend_trusted::closure_env$0<sentry_types::protocol::v7::Frame,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<minidump_unwind::StackFrame,alloc::alloc::Global>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2> > > > >; module=175, inline_record=5472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
