// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/main.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: aa1312e7023adf5a6613c7798ba17f9d9d662132905a1b3af4922aa6bf5edb4c, cfce01971d251ff3b67df3325b8d8f385c4f7da6d6accd40e2ab3070c3f6de36
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: main.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 89, 109, 117, 127, 236
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use fern; // namespace candidate
//   package identities: [{"name":"fern","source":"PDB_registry_path_only","version":"0.6.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use minidump; // namespace candidate
//   package identities: [{"name":"minidump","source":"PDB_registry_path_only","version":"0.21.1"}]
// use minidump_common; // namespace candidate
//   package identities: [{"name":"minidump-common","source":"PDB_registry_path_only","version":"0.21.1"}]
// use minidump_unwind; // namespace candidate
//   package identities: [{"name":"minidump-unwind","source":"PDB_registry_path_only","version":"0.21.1"},{"name":"tracing","source":"PDB_registry_path_only","version":"0.1.40"}]
// use rfd; // namespace candidate
//   package identities: [{"name":"rfd","source":"PDB_registry_path_only","version":"0.10.0"}]
// use sentry; // namespace candidate
//   package identities: [{"name":"sentry","source":"PDB_registry_path_only","version":"0.31.7"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: []
// use tracing_core; // namespace candidate
//   package identities: [{"name":"tracing-core","source":"PDB_registry_path_only","version":"0.1.32"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: minidump_unwind::mips::get_caller_frame::async_fn$0<minidump_unwind::symbols::MultiSymbolProvider>
// 记录形式: compiled_procedure; instances=2
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":89,"record_offset":77612,"rva":1192624,"va":5369901744}
// 编译实例 RVAs: 0x1232b0, 0x197110
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1703952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e810001a00","size":5,"soft_fail":false,"target_va":5371621248,"va":5369917291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$minidump_common..format..CONTEXT_MIPS$u20$as$u20$minidump..context..CpuContext$GT$::get_register_always
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t3839433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8c9953a00","size":5,"soft_fail":false,"target_va":5373744208,"va":5369904770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$minidump_common..format..CONTEXT_MIPS$u20$as$u20$minidump..context..CpuContext$GT$::set_register
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t3827396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8c4663a00","size":5,"soft_fail":false,"target_va":5373744592,"va":5369917191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$minidump_unwind..mips..Mips32Context$u20$as$u20$core..convert..TryFrom$LT$minidump_common..format..CONTEXT_MIPS$GT$$GT$::try_from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3384029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8dda23300","size":5,"soft_fail":false,"target_va":5373286096,"va":5369902062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$minidump_unwind..mips..Mips32Context$u20$as$u20$minidump..context..CpuContext$GT$::get_register_always
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t3379280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e850903300","size":5,"soft_fail":false,"target_va":5373286064,"va":5369906779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$minidump_unwind..symbols..MultiSymbolProvider$u20$as$u20$minidump_unwind..symbols..SymbolProvider$GT$::walk_frame
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t3410563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8830a3400","size":5,"soft_fail":false,"target_va":5373315136,"va":5369904568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140045d50
//     owner=unresolved_namespace; count=8
//     target_aliases=101 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-925615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e851e0f1ff","size":5,"soft_fail":false,"target_va":5368995152,"va":5369920762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401a69b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t535877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8452d0800","size":5,"soft_fail":false,"target_va":5370440112,"va":5369904230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da370
//     owner=unresolved_namespace; count=8
//     target_aliases=37 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1779187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8f3251b00","size":5,"soft_fail":false,"target_va":5371700080,"va":5369920888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140350a40
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2278906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8fac52200","size":5,"soft_fail":false,"target_va":5372185152,"va":5369906241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140373730
//     owner=unresolved_namespace; count=14
//     target_aliases=2 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2419236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e824ea2400","size":5,"soft_fail":false,"target_va":5372327728,"va":5369908487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=2
//     target_aliases=16 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2705109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8d5462900","size":5,"soft_fail":false,"target_va":5372625024,"va":5369919910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=20
//     target_aliases=1 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2845009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e851692b00","size":5,"soft_fail":false,"target_va":5372749712,"va":5369904698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209b690
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32998346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8ca83f701","size":5,"soft_fail":false,"target_va":5402900112,"va":5369901761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=86
//     target_aliases=1 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33691447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e837170202","size":5,"soft_fail":false,"target_va":5403593360,"va":5369901908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33676257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8e1db0102","size":5,"soft_fail":false,"target_va":5403593392,"va":5369917130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=40
//     evidence={"assembly":"\tcallq\t33066402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8a28df801","size":5,"soft_fail":false,"target_va":5402978912,"va":5369912505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=20
//     evidence={"assembly":"\tcallq\t33067062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e83690f801","size":5,"soft_fail":false,"target_va":5402979152,"va":5369912085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<minidump_unwind::arm64::get_caller_by_scan::async_fn_env$0<minidump_unwind::symbols::MultiSymbolProvider> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e858220000","size":5,"soft_fail":false,"target_va":5369931888,"va":5369923091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<minidump_unwind::mips::get_caller_by_cfi::async_fn_env$0<minidump_common::format::CONTEXT_MIPS,minidump_unwind::symbols::MultiSymbolProvider> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t11576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8382d0000","size":5,"soft_fail":false,"target_va":5369933392,"va":5369921811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<minidump_unwind::mips::get_caller_by_cfi::async_fn_env$0<minidump_unwind::mips::Mips32Context,minidump_unwind::symbols::MultiSymbolProvider> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t12040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8082f0000","size":5,"soft_fail":false,"target_va":5369933152,"va":5369921107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<minidump_unwind::mips::get_caller_by_scan32::async_fn_env$0<minidump_unwind::symbols::MultiSymbolProvider> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e888250000","size":5,"soft_fail":false,"target_va":5369932064,"va":5369922451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::map::HashMap$LT$K$C$V$C$S$C$A$GT$::insert
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t1708326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e826111a00","size":5,"soft_fail":false,"target_va":5371625696,"va":5369917365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::logger
//     owner=unresolved_namespace; count=30
//     evidence={"assembly":"\tcallq\t7451116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8ecb17100","size":5,"soft_fail":false,"target_va":5377353728,"va":5369902607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump_unwind::CfiStackWalker$LT$C$GT$::from_ctx_and_args
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t2322864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8b0712300","size":5,"soft_fail":false,"target_va":5372227168,"va":5369904299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump_unwind::StackFrame::from_context
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t3398352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8d0da3300","size":5,"soft_fail":false,"target_va":5373306672,"va":5369908315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump_unwind::mips::get_caller_by_cfi::async_fn$0::closure$1<minidump_common::format::CONTEXT_MIPS,minidump_unwind::symbols::MultiSymbolProvider>
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t15326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8de3b0000","size":5,"soft_fail":false,"target_va":5369923472,"va":5369908141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump_unwind::mips::get_caller_by_scan32::async_fn$0::closure$1<minidump_unwind::symbols::MultiSymbolProvider>
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8ff190000","size":5,"soft_fail":false,"target_va":5369923648,"va":5369916988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump_unwind::mips::get_caller_by_scan64::async_fn$0::closure$1<minidump_unwind::symbols::MultiSymbolProvider>
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e883100000","size":5,"soft_fail":false,"target_va":5369923824,"va":5369919592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump_unwind::mips::get_caller_frame::async_fn$0::closure$1<minidump_unwind::symbols::MultiSymbolProvider>
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8ec1f0000","size":5,"soft_fail":false,"target_va":5369923296,"va":5369915119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump_unwind::mips::instruction_seems_valid::async_fn$0<minidump_unwind::symbols::MultiSymbolProvider>
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t8536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e858210000","size":5,"soft_fail":false,"target_va":5369924000,"va":5369915459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7446359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8579f7100","size":5,"soft_fail":false,"target_va":5377366224,"va":5369919860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tracing::__macro_support::__is_enabled
//     owner=unresolved_namespace; count=20
//     evidence={"assembly":"\tcallq\t7447007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8dfa17100","size":5,"soft_fail":false,"target_va":5377350704,"va":5369903692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tracing::__macro_support::__tracing_log
//     owner=unresolved_namespace; count=30
//     evidence={"assembly":"\tcallq\t7447787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8eba47100","size":5,"soft_fail":false,"target_va":5377350736,"va":5369902944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tracing_core::callsite::DefaultCallsite::register
//     owner=unresolved_namespace; count=20
//     evidence={"assembly":"\tcallq\t33069006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e8ce97f801","size":5,"soft_fail":false,"target_va":5402972656,"va":5369903645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tracing_core::event::Event::dispatch
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t7453556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369901744,"raw_hex":"e874bb7100","size":5,"soft_fail":false,"target_va":5377357488,"va":5369903927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=42
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5369901744,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5369902660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":89,"record_offset":95600,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":89,"record_offset":95944,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":89,"record_offset":95908,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":89,"record_offset":95124,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$40::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":89,"record_offset":95180,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":89,"record_offset":95060,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] log::MetadataBuilder::build
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":89,"record_offset":95656,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":7154,"source_line":1115,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":89,"record_offset":95516,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] minidump::context::MinidumpContext::get_instruction_pointer
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"minidump","version":"0.21.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/minidump-0.21.1/src/context.rs
//     evidence={"module":89,"record_offset":95216,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":9237,"source_line":1180,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] minidump::context::MinidumpContext::get_stack_pointer
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"minidump","version":"0.21.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/minidump-0.21.1/src/context.rs
//     evidence={"module":89,"record_offset":96092,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":9238,"source_line":1196,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] minidump_common::format::impl$256::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"minidump-common","version":"0.21.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/minidump-common-0.21.1/src/format.rs
//     evidence={"module":89,"record_offset":77888,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":9255,"source_line":1191,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] minidump_unwind::mips::get_caller_by_cfi::async_fn$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/minidump-unwind-0.21.1/src/mips.rs
//     evidence={"module":89,"record_offset":87104,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":9263,"source_line":32,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] minidump_unwind::mips::get_caller_by_scan32::async_fn$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/minidump-unwind-0.21.1/src/mips.rs
//     evidence={"module":89,"record_offset":82948,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":9261,"source_line":78,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] minidump_unwind::mips::get_caller_by_scan64::async_fn$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/minidump-unwind-0.21.1/src/mips.rs
//     evidence={"module":89,"record_offset":78220,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":9259,"source_line":146,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] minidump_unwind::mips::get_caller_frame::async_fn$0::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tracing","version":"0.1.40","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-0.1.40/src/macros.rs
//     evidence={"module":89,"record_offset":97232,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":9277,"source_line":872,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] tracing_core::callsite::DefaultCallsite::interest
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-core-0.1.32/src/callsite.rs
//     evidence={"module":89,"record_offset":96000,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":7140,"source_line":349,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] tracing_core::callsite::impl$1::metadata
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-core-0.1.32/src/callsite.rs
//     evidence={"module":89,"record_offset":95628,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":7143,"source_line":370,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] tracing_core::dispatcher::has_been_set
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-core-0.1.32/src/dispatcher.rs
//     evidence={"module":89,"record_offset":95432,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":7150,"source_line":345,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] tracing_core::field::FieldSet::iter
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-core-0.1.32/src/field.rs
//     evidence={"module":89,"record_offset":95740,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":7145,"source_line":870,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] tracing_core::field::FieldSet::value_set
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-core-0.1.32/src/field.rs
//     evidence={"module":89,"record_offset":95972,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":7157,"source_line":883,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] tracing_core::field::impl$51::next
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-core-0.1.32/src/field.rs
//     evidence={"module":89,"record_offset":95796,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":7148,"source_line":971,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] tracing_core::metadata::LevelFilter::current
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-core-0.1.32/src/metadata.rs
//     evidence={"module":89,"record_offset":95292,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":7134,"source_line":697,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] tracing_core::metadata::impl$23::le
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-core-0.1.32/src/metadata.rs
//     evidence={"module":89,"record_offset":95376,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":7136,"source_line":957,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_direct] tracing_core::subscriber::Interest::is_never
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-core-0.1.32/src/subscriber.rs
//     evidence={"module":89,"record_offset":96916,"parent_record_offset":77612,"ancestor_record_offset":77612,"inlinee":9197,"source_line":637,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":89,"record_offset":80552,"parent_record_offset":80524,"ancestor_record_offset":77612,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":89,"record_offset":80524,"parent_record_offset":80440,"ancestor_record_offset":77612,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":89,"record_offset":80440,"parent_record_offset":80412,"ancestor_record_offset":77612,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":89,"record_offset":80468,"parent_record_offset":80440,"ancestor_record_offset":77612,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":89,"record_offset":77988,"parent_record_offset":77956,"ancestor_record_offset":77612,"inlinee":9213,"source_line":403,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$22::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":89,"record_offset":78012,"parent_record_offset":77988,"ancestor_record_offset":77612,"inlinee":9215,"source_line":426,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$29::default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":89,"record_offset":82124,"parent_record_offset":82092,"ancestor_record_offset":77612,"inlinee":9250,"source_line":439,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":89,"record_offset":82544,"parent_record_offset":82516,"ancestor_record_offset":77612,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":89,"record_offset":82604,"parent_record_offset":82576,"ancestor_record_offset":77612,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":89,"record_offset":82576,"parent_record_offset":82516,"ancestor_record_offset":77612,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$8::clone
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":89,"record_offset":78044,"parent_record_offset":77956,"ancestor_record_offset":77612,"inlinee":7772,"source_line":219,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$9::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":89,"record_offset":78104,"parent_record_offset":77888,"ancestor_record_offset":77612,"inlinee":7764,"source_line":219,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":89,"record_offset":84792,"parent_record_offset":82948,"ancestor_record_offset":77612,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":89,"record_offset":78788,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":89,"record_offset":79056,"parent_record_offset":79032,"ancestor_record_offset":77612,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$55::cmp
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":89,"record_offset":79844,"parent_record_offset":79820,"ancestor_record_offset":77612,"inlinee":7631,"source_line":1565,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":89,"record_offset":84964,"parent_record_offset":84932,"ancestor_record_offset":77612,"inlinee":5261,"source_line":1553,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$62::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":89,"record_offset":80128,"parent_record_offset":80088,"ancestor_record_offset":77612,"inlinee":6123,"source_line":1553,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":89,"record_offset":79132,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":89,"record_offset":81572,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::future::future::impl$1::poll
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\future\future.rs
//     evidence={"module":89,"record_offset":87356,"parent_record_offset":87104,"ancestor_record_offset":77612,"inlinee":8296,"source_line":122,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":89,"record_offset":84996,"parent_record_offset":84932,"ancestor_record_offset":77612,"inlinee":5988,"source_line":207,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$41::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":89,"record_offset":80160,"parent_record_offset":80088,"ancestor_record_offset":77612,"inlinee":6125,"source_line":207,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":89,"record_offset":79032,"parent_record_offset":79008,"ancestor_record_offset":77612,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":89,"record_offset":79008,"parent_record_offset":78984,"ancestor_record_offset":77612,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":89,"record_offset":82632,"parent_record_offset":82604,"ancestor_record_offset":77612,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":89,"record_offset":80492,"parent_record_offset":80468,"ancestor_record_offset":77612,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":89,"record_offset":90424,"parent_record_offset":90396,"ancestor_record_offset":77612,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":89,"record_offset":90448,"parent_record_offset":90424,"ancestor_record_offset":77612,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::checked_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":89,"record_offset":84848,"parent_record_offset":82948,"ancestor_record_offset":77612,"inlinee":9234,"source_line":460,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::overflowing_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":89,"record_offset":84872,"parent_record_offset":84848,"ancestor_record_offset":77612,"inlinee":9235,"source_line":1975,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":89,"record_offset":85020,"parent_record_offset":84996,"ancestor_record_offset":77612,"inlinee":5989,"source_line":520,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":89,"record_offset":80228,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":9241,"source_line":460,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":89,"record_offset":80252,"parent_record_offset":80228,"ancestor_record_offset":77612,"inlinee":9242,"source_line":1975,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":89,"record_offset":80184,"parent_record_offset":80160,"ancestor_record_offset":77612,"inlinee":6126,"source_line":520,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":89,"record_offset":82472,"parent_record_offset":82444,"ancestor_record_offset":77612,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":89,"record_offset":79096,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":89,"record_offset":95148,"parent_record_offset":95124,"ancestor_record_offset":77612,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":89,"record_offset":80024,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::as_deref_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\pin.rs
//     evidence={"module":89,"record_offset":87384,"parent_record_offset":87356,"ancestor_record_offset":77612,"inlinee":8285,"source_line":1629,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\pin.rs
//     evidence={"module":89,"record_offset":87408,"parent_record_offset":87384,"ancestor_record_offset":77612,"inlinee":8286,"source_line":1433,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=52
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":89,"record_offset":80312,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":89,"record_offset":90512,"parent_record_offset":90488,"ancestor_record_offset":77612,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":89,"record_offset":90488,"parent_record_offset":90364,"ancestor_record_offset":77612,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":89,"record_offset":82660,"parent_record_offset":82632,"ancestor_record_offset":77612,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":89,"record_offset":78644,"parent_record_offset":78620,"ancestor_record_offset":77612,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::load
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":89,"record_offset":79220,"parent_record_offset":79188,"ancestor_record_offset":77612,"inlinee":7141,"source_line":2413,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=50
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":89,"record_offset":78504,"parent_record_offset":78480,"ancestor_record_offset":77612,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=100
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":89,"record_offset":78528,"parent_record_offset":78504,"ancestor_record_offset":77612,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$8::default
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":89,"record_offset":82244,"parent_record_offset":82216,"ancestor_record_offset":77612,"inlinee":8749,"source_line":2227,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":89,"record_offset":90364,"parent_record_offset":90332,"ancestor_record_offset":77612,"inlinee":6232,"source_line":3289,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":89,"record_offset":90276,"parent_record_offset":90252,"ancestor_record_offset":77612,"inlinee":6229,"source_line":2430,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":89,"record_offset":90332,"parent_record_offset":90276,"ancestor_record_offset":77612,"inlinee":6231,"source_line":3259,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":89,"record_offset":90304,"parent_record_offset":90276,"ancestor_record_offset":77612,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":89,"record_offset":90396,"parent_record_offset":90364,"ancestor_record_offset":77612,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":89,"record_offset":90252,"parent_record_offset":90228,"ancestor_record_offset":77612,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":89,"record_offset":82768,"parent_record_offset":82740,"ancestor_record_offset":77612,"inlinee":6708,"source_line":1114,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$13::default
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":89,"record_offset":82216,"parent_record_offset":82188,"ancestor_record_offset":77612,"inlinee":8747,"source_line":1402,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] log::MetadataBuilder::build
//     owner=registry_dependency_definition; count=26
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":89,"record_offset":78844,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":7154,"source_line":1115,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$1::partial_cmp
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":89,"record_offset":79820,"parent_record_offset":79732,"ancestor_record_offset":77612,"inlinee":8730,"source_line":474,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=30
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":89,"record_offset":78868,"parent_record_offset":78844,"ancestor_record_offset":77612,"inlinee":7156,"source_line":1035,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=26
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":89,"record_offset":78704,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] minidump::minidump::UnifiedMemory::get_memory_at_address
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"minidump","version":"0.21.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/minidump-0.21.1/src/minidump.rs
//     evidence={"module":89,"record_offset":80284,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":8736,"source_line":2076,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] minidump_common::format::impl$249::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"minidump-common","version":"0.21.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/minidump-common-0.21.1/src/format.rs
//     evidence={"module":89,"record_offset":77956,"parent_record_offset":77888,"ancestor_record_offset":77612,"inlinee":9257,"source_line":1180,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] minidump_common::format::impl$255::default
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"minidump-common","version":"0.21.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/minidump-common-0.21.1/src/format.rs
//     evidence={"module":89,"record_offset":82092,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":9279,"source_line":1191,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] minidump_unwind::GetCallerFrameArgs::valid
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/minidump-unwind-0.21.1/src/lib.rs
//     evidence={"module":89,"record_offset":79996,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":9203,"source_line":41,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] minidump_unwind::impl$15::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/minidump-unwind-0.21.1/src/lib.rs
//     evidence={"module":89,"record_offset":84816,"parent_record_offset":84792,"ancestor_record_offset":77612,"inlinee":9275,"source_line":55,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] minidump_unwind::mips::get_caller_by_cfi::async_fn$0::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"tracing","version":"0.1.40","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-0.1.40/src/macros.rs
//     evidence={"module":89,"record_offset":88560,"parent_record_offset":87104,"ancestor_record_offset":77612,"inlinee":9265,"source_line":872,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] minidump_unwind::mips::get_caller_by_scan32::async_fn$0::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tracing","version":"0.1.40","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-0.1.40/src/macros.rs
//     evidence={"module":89,"record_offset":84336,"parent_record_offset":82948,"ancestor_record_offset":77612,"inlinee":9273,"source_line":872,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] minidump_unwind::mips::get_caller_by_scan64::async_fn$0::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tracing","version":"0.1.40","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-0.1.40/src/macros.rs
//     evidence={"module":89,"record_offset":79596,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":9271,"source_line":872,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] minidump_unwind::mips::impl$1::into_ctx
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/minidump-unwind-0.21.1/src/mips.rs
//     evidence={"module":89,"record_offset":91052,"parent_record_offset":87104,"ancestor_record_offset":77612,"inlinee":9269,"source_line":301,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] minidump_unwind::mips::impl$2::into_ctx
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/minidump-unwind-0.21.1/src/mips.rs
//     evidence={"module":89,"record_offset":95028,"parent_record_offset":91084,"ancestor_record_offset":77612,"inlinee":9267,"source_line":311,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":89,"record_offset":82740,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":8705,"source_line":889,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":89,"record_offset":82160,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":8743,"source_line":126,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":89,"record_offset":82188,"parent_record_offset":82160,"ancestor_record_offset":77612,"inlinee":8745,"source_line":1116,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":89,"record_offset":82300,"parent_record_offset":82272,"ancestor_record_offset":77612,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":89,"record_offset":82388,"parent_record_offset":82356,"ancestor_record_offset":77612,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":89,"record_offset":82516,"parent_record_offset":82356,"ancestor_record_offset":77612,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":89,"record_offset":82272,"parent_record_offset":82244,"ancestor_record_offset":77612,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":89,"record_offset":82416,"parent_record_offset":82388,"ancestor_record_offset":77612,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":89,"record_offset":82444,"parent_record_offset":82416,"ancestor_record_offset":77612,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":89,"record_offset":82356,"parent_record_offset":82328,"ancestor_record_offset":77612,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":89,"record_offset":82328,"parent_record_offset":82300,"ancestor_record_offset":77612,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] tracing_core::callsite::DefaultCallsite::interest
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-core-0.1.32/src/callsite.rs
//     evidence={"module":89,"record_offset":79188,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":7140,"source_line":349,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] tracing_core::callsite::impl$1::metadata
//     owner=registry_dependency_definition; count=74
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-core-0.1.32/src/callsite.rs
//     evidence={"module":89,"record_offset":78816,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":7143,"source_line":370,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] tracing_core::dispatcher::has_been_set
//     owner=registry_dependency_definition; count=26
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-core-0.1.32/src/dispatcher.rs
//     evidence={"module":89,"record_offset":78620,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":7150,"source_line":345,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] tracing_core::field::FieldSet::iter
//     owner=registry_dependency_definition; count=32
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-core-0.1.32/src/field.rs
//     evidence={"module":89,"record_offset":78928,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":7145,"source_line":870,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] tracing_core::field::FieldSet::len
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-core-0.1.32/src/field.rs
//     evidence={"module":89,"record_offset":78952,"parent_record_offset":78928,"ancestor_record_offset":77612,"inlinee":7146,"source_line":895,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] tracing_core::field::FieldSet::value_set
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-core-0.1.32/src/field.rs
//     evidence={"module":89,"record_offset":79160,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":7157,"source_line":883,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] tracing_core::field::impl$51::next
//     owner=registry_dependency_definition; count=32
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-core-0.1.32/src/field.rs
//     evidence={"module":89,"record_offset":78984,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":7148,"source_line":971,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] tracing_core::metadata::LevelFilter::current
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-core-0.1.32/src/metadata.rs
//     evidence={"module":89,"record_offset":78480,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":7134,"source_line":697,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] tracing_core::metadata::Metadata::target
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-core-0.1.32/src/metadata.rs
//     evidence={"module":89,"record_offset":79908,"parent_record_offset":79596,"ancestor_record_offset":77612,"inlinee":7152,"source_line":296,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] tracing_core::metadata::filter_as_usize
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-core-0.1.32/src/metadata.rs
//     evidence={"module":89,"record_offset":78588,"parent_record_offset":78564,"ancestor_record_offset":77612,"inlinee":7138,"source_line":973,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] tracing_core::metadata::impl$23::le
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-core-0.1.32/src/metadata.rs
//     evidence={"module":89,"record_offset":78564,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":7136,"source_line":957,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   [pdb_inline_transitive] tracing_core::subscriber::Interest::is_never
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-core-0.1.32/src/subscriber.rs
//     evidence={"module":89,"record_offset":79280,"parent_record_offset":78220,"ancestor_record_offset":77612,"inlinee":9197,"source_line":637,"archive":"_pdb/A/modules/00089.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::log_time_to_first_frame
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x29C7
// 首条位置: {"module":127,"inlinee":10695}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":123160,"parent_record_offset":122904,"ancestor_record_offset":122904,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":127,"record_offset":123192,"parent_record_offset":122904,"ancestor_record_offset":122904,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":122936,"parent_record_offset":122904,"ancestor_record_offset":122904,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":127,"record_offset":123224,"parent_record_offset":122904,"ancestor_record_offset":122904,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":127,"record_offset":123064,"parent_record_offset":122904,"ancestor_record_offset":122904,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":127,"record_offset":122964,"parent_record_offset":122936,"ancestor_record_offset":122904,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":122992,"parent_record_offset":122964,"ancestor_record_offset":122904,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":123024,"parent_record_offset":122964,"ancestor_record_offset":122904,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":123092,"parent_record_offset":123064,"ancestor_record_offset":122904,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":123120,"parent_record_offset":123092,"ancestor_record_offset":122904,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::main
// 记录形式: compiled_procedure; instances=1
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":236,"record_offset":4280,"rva":3770448,"va":5372479568}
// 编译实例 RVAs: 0x398850
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$sentry..init..ClientInitGuard$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t494914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8428d0700","size":5,"soft_fail":false,"target_va":5372978640,"va":5372483721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce150
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-832647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8794bf3ff","size":5,"soft_fail":false,"target_va":5371650384,"va":5372483026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da640
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-789057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8bff5f3ff","size":5,"soft_fail":false,"target_va":5371700800,"va":5372489852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=5
//     target_aliases=89 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-166262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e88a76fdff","size":5,"soft_fail":false,"target_va":5372314208,"va":5372480465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371310
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-168827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8856cfdff","size":5,"soft_fail":false,"target_va":5372318480,"va":5372487302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140375ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-150100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8acb5fdff","size":5,"soft_fail":false,"target_va":5372337904,"va":5372487999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037ae10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-132877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8f3f8fdff","size":5,"soft_fail":false,"target_va":5372358160,"va":5372491032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037f2c0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-110396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8c450feff","size":5,"soft_fail":false,"target_va":5372375744,"va":5372486135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140386740
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-80910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8f2c3feff","size":5,"soft_fail":false,"target_va":5372405568,"va":5372486473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-77280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e820d2feff","size":5,"soft_fail":false,"target_va":5372408976,"va":5372486251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140389560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-70835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e84debfeff","size":5,"soft_fail":false,"target_va":5372417376,"va":5372488206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t267349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e855140400","size":5,"soft_fail":false,"target_va":5372749696,"va":5372482342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=36
//     target_aliases=1 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t269921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8611e0400","size":5,"soft_fail":false,"target_va":5372749712,"va":5372479786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14042abc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t596054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e856180900","size":5,"soft_fail":false,"target_va":5373078464,"va":5372482405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=2
//     target_aliases=28 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2691367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e827112900","size":5,"soft_fail":false,"target_va":5375171056,"va":5372479684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31110561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8a1b5da01","size":5,"soft_fail":false,"target_va":5403593360,"va":5372482794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30491217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e85142d101","size":5,"soft_fail":false,"target_va":5402978144,"va":5372486922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5048629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e835094d00","size":5,"soft_fail":false,"target_va":5377529888,"va":5372481254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-1174234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e82615eeff","size":5,"soft_fail":false,"target_va":5371305968,"va":5372480197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-826037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e84b65f3ff","size":5,"soft_fail":false,"target_va":5371657728,"va":5372483760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t30464505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8f9d9d001","size":5,"soft_fail":false,"target_va":5402945776,"va":5372481266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4879630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e80e754a00","size":5,"soft_fail":false,"target_va":5377364336,"va":5372484701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<fern::builders::OutputInner> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-8733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8e3ddffff","size":5,"soft_fail":false,"target_va":5372478480,"va":5372487208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<tiny_glade::crash_pickup::Error> >
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-4168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8b8efffff","size":5,"soft_fail":false,"target_va":5372479184,"va":5372483347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::steam::AppSteamIntegration::try_new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2245782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e896442200","size":5,"soft_fail":false,"target_va":5374725696,"va":5372479909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::steam::AppSteamIntegration::user_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2242186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e88a362200","size":5,"soft_fail":false,"target_va":5374725984,"va":5372483793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fern::builders::Dispatch::apply
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t657152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e800070a00","size":5,"soft_fail":false,"target_va":5373139216,"va":5372482059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t4871482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e83a554a00","size":5,"soft_fail":false,"target_va":5377353776,"va":5372482289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::initialize_paths
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3670964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8b4033800","size":5,"soft_fail":false,"target_va":5376154832,"va":5372483863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rfd::message_dialog::MessageDialog::set_description
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3512084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e814973500","size":5,"soft_fail":false,"target_va":5375996736,"va":5372484647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rfd::message_dialog::MessageDialog::set_title
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3511973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8a5963500","size":5,"soft_fail":false,"target_va":5375996496,"va":5372484518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rfd::message_dialog::MessageDialog::show
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3512520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8c8983500","size":5,"soft_fail":false,"target_va":5375997184,"va":5372484659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] sentry::init::init
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3665742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8b210c8ff","size":5,"soft_fail":false,"target_va":5368817184,"va":5372482921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] sentry_core::client::Client::close
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t606862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e88e420900","size":5,"soft_fail":false,"target_va":5373090576,"va":5372483709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::_set_var
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4921316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8e4174b00","size":5,"soft_fail":false,"target_va":5377401200,"va":5372479879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::temp_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4922035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8b31a4b00","size":5,"soft_fail":false,"target_va":5377401712,"va":5372479672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4928320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e840334b00","size":5,"soft_fail":false,"target_va":5377408752,"va":5372480427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4927105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8812e4b00","size":5,"soft_fail":false,"target_va":5377408016,"va":5372480906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::stdio::_eprint
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4937779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e833584b00","size":5,"soft_fail":false,"target_va":5377418800,"va":5372481016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::stdio::stdout
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4933700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e844484b00","size":5,"soft_fail":false,"target_va":5377413824,"va":5372480119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4960050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e832af4b00","size":5,"soft_fail":false,"target_va":5377439776,"va":5372479721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4963326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8febb4b00","size":5,"soft_fail":false,"target_va":5377442944,"va":5372479613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::app_config::get_app_config
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-474277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e85bc3f8ff","size":5,"soft_fail":false,"target_va":5372009632,"va":5372483904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::app_config::get_cli_args
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-471395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e89dcef8ff","size":5,"soft_fail":false,"target_va":5372008256,"va":5372479646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::app_config::update_vulkan_allowlist_defaults
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-473783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e849c5f8ff","size":5,"soft_fail":false,"target_va":5372012112,"va":5372485890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::crash_pickup::CrashReporter::spawn_server
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t109492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8b4ab0100","size":5,"soft_fail":false,"target_va":5372594400,"va":5372484903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::crash_pickup::server::run
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3680023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8e9d8c7ff","size":5,"soft_fail":false,"target_va":5368803088,"va":5372483106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::integrity_report_main::verify_package_integrity_and_show_report
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-598731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e835ddf6ff","size":5,"soft_fail":false,"target_va":5371887344,"va":5372486070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::load_language_library::load_language_library
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2792212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8ec64d5ff","size":5,"soft_fail":false,"target_va":5369693808,"va":5372486015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::logging::set_up_logging
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-703312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8b044f5ff","size":5,"soft_fail":false,"target_va":5371781712,"va":5372485019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_report::on_game_crashed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3750467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e8bdc5c6ff","size":5,"soft_fail":false,"target_va":5368733216,"va":5372483678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::setup_panic_hook
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1186961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e86fe3edff","size":5,"soft_fail":false,"target_va":5371299008,"va":5372485964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::run_game::run_game
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t175985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e871af0200","size":5,"soft_fail":false,"target_va":5372662576,"va":5372486586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::vulkan_config_main::run_vulkan_configuration_app
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1400494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e852a1eaff","size":5,"soft_fail":false,"target_va":5371085872,"va":5372486361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::local_time::init_local_time
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4458380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372479568,"raw_hex":"e88c074400","size":5,"soft_fail":false,"target_va":5376938016,"va":5372479631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":236,"record_offset":20168,"parent_record_offset":4280,"ancestor_record_offset":4280,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":236,"record_offset":20056,"parent_record_offset":4280,"ancestor_record_offset":4280,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":236,"record_offset":20252,"parent_record_offset":4280,"ancestor_record_offset":4280,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] rfd::message_dialog::MessageDialog::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rfd","version":"0.10.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rfd-0.10.0/src/message_dialog.rs
//     evidence={"module":236,"record_offset":20084,"parent_record_offset":4280,"ancestor_record_offset":4280,"inlinee":8360,"source_line":27,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] rfd::message_dialog::MessageDialog::set_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rfd","version":"0.10.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rfd-0.10.0/src/message_dialog.rs
//     evidence={"module":236,"record_offset":20140,"parent_record_offset":4280,"ancestor_record_offset":4280,"inlinee":8363,"source_line":35,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::try_main
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/main.rs
//     evidence={"module":236,"record_offset":4400,"parent_record_offset":4280,"ancestor_record_offset":4280,"inlinee":13461,"source_line":63,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":236,"record_offset":8224,"parent_record_offset":8200,"ancestor_record_offset":4280,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":236,"record_offset":8248,"parent_record_offset":8224,"ancestor_record_offset":4280,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":236,"record_offset":5364,"parent_record_offset":5340,"ancestor_record_offset":4280,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":236,"record_offset":8168,"parent_record_offset":8140,"ancestor_record_offset":4280,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":236,"record_offset":8200,"parent_record_offset":8168,"ancestor_record_offset":4280,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":236,"record_offset":5340,"parent_record_offset":5284,"ancestor_record_offset":4280,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":236,"record_offset":8140,"parent_record_offset":8112,"ancestor_record_offset":4280,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":236,"record_offset":5816,"parent_record_offset":5732,"ancestor_record_offset":4280,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":236,"record_offset":5864,"parent_record_offset":5840,"ancestor_record_offset":4280,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":236,"record_offset":5312,"parent_record_offset":5284,"ancestor_record_offset":4280,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":236,"record_offset":6840,"parent_record_offset":6808,"ancestor_record_offset":4280,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":236,"record_offset":5284,"parent_record_offset":5256,"ancestor_record_offset":4280,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":236,"record_offset":9832,"parent_record_offset":9796,"ancestor_record_offset":4280,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":236,"record_offset":8112,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":236,"record_offset":14052,"parent_record_offset":14024,"ancestor_record_offset":4280,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":236,"record_offset":8644,"parent_record_offset":8616,"ancestor_record_offset":4280,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":236,"record_offset":6808,"parent_record_offset":6764,"ancestor_record_offset":4280,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":236,"record_offset":6764,"parent_record_offset":6724,"ancestor_record_offset":4280,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":236,"record_offset":8028,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":236,"record_offset":8056,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":236,"record_offset":7460,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":236,"record_offset":13884,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":236,"record_offset":6460,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":9308,"source_line":1257,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":236,"record_offset":6492,"parent_record_offset":6460,"ancestor_record_offset":4280,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":236,"record_offset":13940,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":236,"record_offset":13964,"parent_record_offset":13940,"ancestor_record_offset":4280,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":236,"record_offset":5840,"parent_record_offset":5816,"ancestor_record_offset":4280,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":236,"record_offset":6368,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":236,"record_offset":13996,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=227
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":236,"record_offset":5124,"parent_record_offset":4916,"ancestor_record_offset":4280,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":236,"record_offset":9864,"parent_record_offset":9832,"ancestor_record_offset":4280,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":236,"record_offset":8276,"parent_record_offset":8248,"ancestor_record_offset":4280,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":236,"record_offset":6872,"parent_record_offset":6764,"ancestor_record_offset":4280,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":236,"record_offset":10152,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":236,"record_offset":14076,"parent_record_offset":14052,"ancestor_record_offset":4280,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":236,"record_offset":8680,"parent_record_offset":8644,"ancestor_record_offset":4280,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":236,"record_offset":7968,"parent_record_offset":7944,"ancestor_record_offset":4280,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":236,"record_offset":14100,"parent_record_offset":14076,"ancestor_record_offset":4280,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":236,"record_offset":7992,"parent_record_offset":7968,"ancestor_record_offset":4280,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":236,"record_offset":8704,"parent_record_offset":8680,"ancestor_record_offset":4280,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":236,"record_offset":8736,"parent_record_offset":8644,"ancestor_record_offset":4280,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] fern::builders::Dispatch::chain
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"fern","version":"0.6.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fern-0.6.1/src/builders.rs
//     evidence={"module":236,"record_offset":6724,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":12420,"source_line":198,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] fern::builders::Dispatch::format
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fern","version":"0.6.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fern-0.6.1/src/builders.rs
//     evidence={"module":236,"record_offset":7516,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":12418,"source_line":171,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] fern::builders::Dispatch::level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fern","version":"0.6.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fern-0.6.1/src/builders.rs
//     evidence={"module":236,"record_offset":6696,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":12419,"source_line":224,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] fern::builders::Dispatch::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"fern","version":"0.6.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fern-0.6.1/src/builders.rs
//     evidence={"module":236,"record_offset":6668,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":12417,"source_line":127,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":236,"record_offset":8084,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":236,"record_offset":7944,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] rfd::message_dialog::impl$5::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rfd","version":"0.10.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rfd-0.10.0/src/message_dialog.rs
//     evidence={"module":236,"record_offset":20108,"parent_record_offset":20084,"ancestor_record_offset":4280,"inlinee":8362,"source_line":13,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] sentry::init::impl$0::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry","version":"0.31.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sentry-0.31.7/src/init.rs
//     evidence={"module":236,"record_offset":9716,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":13469,"source_line":17,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::env::set_var
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\env.rs
//     evidence={"module":236,"record_offset":5788,"parent_record_offset":5732,"ancestor_record_offset":4280,"inlinee":11089,"source_line":360,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":236,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":4280,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":236,"record_offset":5048,"parent_record_offset":5024,"ancestor_record_offset":4280,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":236,"record_offset":5024,"parent_record_offset":5000,"ancestor_record_offset":4280,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":236,"record_offset":6960,"parent_record_offset":6908,"ancestor_record_offset":4280,"inlinee":4771,"source_line":2561,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":236,"record_offset":6932,"parent_record_offset":6908,"ancestor_record_offset":4280,"inlinee":4770,"source_line":2537,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":236,"record_offset":7320,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":12422,"source_line":1049,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":236,"record_offset":7236,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":5267,"source_line":925,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":236,"record_offset":7432,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":5272,"source_line":1130,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":236,"record_offset":7376,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":12423,"source_line":1028,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":236,"record_offset":7264,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":12421,"source_line":963,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::create_dir_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":236,"record_offset":6908,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":4768,"source_line":2306,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":236,"record_offset":5092,"parent_record_offset":4916,"ancestor_record_offset":4280,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":236,"record_offset":4976,"parent_record_offset":4952,"ancestor_record_offset":4280,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":236,"record_offset":4952,"parent_record_offset":4916,"ancestor_record_offset":4280,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":236,"record_offset":7344,"parent_record_offset":7320,"ancestor_record_offset":4280,"inlinee":5270,"source_line":218,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":236,"record_offset":7400,"parent_record_offset":7376,"ancestor_record_offset":4280,"inlinee":5271,"source_line":215,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":236,"record_offset":7288,"parent_record_offset":7264,"ancestor_record_offset":4280,"inlinee":5269,"source_line":209,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::CrashReporter::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":236,"record_offset":4888,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":13463,"source_line":65,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::CrashReporter::run_server
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":236,"record_offset":8776,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":13467,"source_line":70,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":236,"record_offset":4916,"parent_record_offset":4888,"ancestor_record_offset":4280,"inlinee":13465,"source_line":54,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::impl$9::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":236,"record_offset":14024,"parent_record_offset":13996,"ancestor_record_offset":4280,"inlinee":13473,"source_line":31,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::find_steam_user_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/main.rs
//     evidence={"module":236,"record_offset":6424,"parent_record_offset":6400,"ancestor_record_offset":4280,"inlinee":13471,"source_line":290,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::init_steam_integration
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/main.rs
//     evidence={"module":236,"record_offset":5732,"parent_record_offset":4400,"ancestor_record_offset":4280,"inlinee":13466,"source_line":333,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::try_main::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/main.rs
//     evidence={"module":236,"record_offset":6400,"parent_record_offset":6368,"ancestor_record_offset":4280,"inlinee":13470,"source_line":186,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::try_main
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3495
// 首条位置: {"module":236,"inlinee":13461}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":236,"record_offset":8112,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":236,"record_offset":8028,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":236,"record_offset":8056,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":236,"record_offset":7460,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":236,"record_offset":13884,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":236,"record_offset":6460,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":9308,"source_line":1257,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":236,"record_offset":14228,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":236,"record_offset":13940,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":236,"record_offset":6368,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":236,"record_offset":13996,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":236,"record_offset":6992,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":236,"record_offset":10152,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] fern::builders::Dispatch::chain
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"fern","version":"0.6.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fern-0.6.1/src/builders.rs
//     evidence={"module":236,"record_offset":6724,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":12420,"source_line":198,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] fern::builders::Dispatch::format
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fern","version":"0.6.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fern-0.6.1/src/builders.rs
//     evidence={"module":236,"record_offset":7516,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":12418,"source_line":171,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] fern::builders::Dispatch::level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fern","version":"0.6.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fern-0.6.1/src/builders.rs
//     evidence={"module":236,"record_offset":6696,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":12419,"source_line":224,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] fern::builders::Dispatch::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"fern","version":"0.6.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fern-0.6.1/src/builders.rs
//     evidence={"module":236,"record_offset":6668,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":12417,"source_line":127,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":236,"record_offset":8084,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":236,"record_offset":7944,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] sentry::init::impl$0::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry","version":"0.31.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sentry-0.31.7/src/init.rs
//     evidence={"module":236,"record_offset":9716,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":13469,"source_line":17,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":236,"record_offset":7320,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":12422,"source_line":1049,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":236,"record_offset":7236,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":5267,"source_line":925,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":236,"record_offset":7432,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":5272,"source_line":1130,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":236,"record_offset":7376,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":12423,"source_line":1028,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":236,"record_offset":7264,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":12421,"source_line":963,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":236,"record_offset":6908,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":4768,"source_line":2306,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::crash_pickup::CrashReporter::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":236,"record_offset":4888,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":13463,"source_line":65,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::crash_pickup::CrashReporter::run_server
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":236,"record_offset":8776,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":13467,"source_line":70,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::init_steam_integration
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/main.rs
//     evidence={"module":236,"record_offset":5732,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":13466,"source_line":333,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":236,"record_offset":8224,"parent_record_offset":8200,"ancestor_record_offset":4400,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":236,"record_offset":8248,"parent_record_offset":8224,"ancestor_record_offset":4400,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":236,"record_offset":5364,"parent_record_offset":5340,"ancestor_record_offset":4400,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":236,"record_offset":8168,"parent_record_offset":8140,"ancestor_record_offset":4400,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":236,"record_offset":8200,"parent_record_offset":8168,"ancestor_record_offset":4400,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":236,"record_offset":5340,"parent_record_offset":5284,"ancestor_record_offset":4400,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":236,"record_offset":8140,"parent_record_offset":8112,"ancestor_record_offset":4400,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":236,"record_offset":5816,"parent_record_offset":5732,"ancestor_record_offset":4400,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":236,"record_offset":5864,"parent_record_offset":5840,"ancestor_record_offset":4400,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":236,"record_offset":5312,"parent_record_offset":5284,"ancestor_record_offset":4400,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":236,"record_offset":6840,"parent_record_offset":6808,"ancestor_record_offset":4400,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":236,"record_offset":5284,"parent_record_offset":5256,"ancestor_record_offset":4400,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":236,"record_offset":9832,"parent_record_offset":9796,"ancestor_record_offset":4400,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":236,"record_offset":14052,"parent_record_offset":14024,"ancestor_record_offset":4400,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":236,"record_offset":8644,"parent_record_offset":8616,"ancestor_record_offset":4400,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":236,"record_offset":6808,"parent_record_offset":6764,"ancestor_record_offset":4400,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":236,"record_offset":6764,"parent_record_offset":6724,"ancestor_record_offset":4400,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":236,"record_offset":6492,"parent_record_offset":6460,"ancestor_record_offset":4400,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":236,"record_offset":13964,"parent_record_offset":13940,"ancestor_record_offset":4400,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":236,"record_offset":5840,"parent_record_offset":5816,"ancestor_record_offset":4400,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=184
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":236,"record_offset":5124,"parent_record_offset":4916,"ancestor_record_offset":4400,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":236,"record_offset":9864,"parent_record_offset":9832,"ancestor_record_offset":4400,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":236,"record_offset":8276,"parent_record_offset":8248,"ancestor_record_offset":4400,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":236,"record_offset":6872,"parent_record_offset":6764,"ancestor_record_offset":4400,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":236,"record_offset":14076,"parent_record_offset":14052,"ancestor_record_offset":4400,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":236,"record_offset":8680,"parent_record_offset":8644,"ancestor_record_offset":4400,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":236,"record_offset":7968,"parent_record_offset":7944,"ancestor_record_offset":4400,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":236,"record_offset":14100,"parent_record_offset":14076,"ancestor_record_offset":4400,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":236,"record_offset":7992,"parent_record_offset":7968,"ancestor_record_offset":4400,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":236,"record_offset":8704,"parent_record_offset":8680,"ancestor_record_offset":4400,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":236,"record_offset":8736,"parent_record_offset":8644,"ancestor_record_offset":4400,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::env::set_var
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\env.rs
//     evidence={"module":236,"record_offset":5788,"parent_record_offset":5732,"ancestor_record_offset":4400,"inlinee":11089,"source_line":360,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":236,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":4400,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":236,"record_offset":5048,"parent_record_offset":5024,"ancestor_record_offset":4400,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":236,"record_offset":5024,"parent_record_offset":5000,"ancestor_record_offset":4400,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":236,"record_offset":6960,"parent_record_offset":6908,"ancestor_record_offset":4400,"inlinee":4771,"source_line":2561,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":236,"record_offset":6932,"parent_record_offset":6908,"ancestor_record_offset":4400,"inlinee":4770,"source_line":2537,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":236,"record_offset":5092,"parent_record_offset":4916,"ancestor_record_offset":4400,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":236,"record_offset":4976,"parent_record_offset":4952,"ancestor_record_offset":4400,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":236,"record_offset":4952,"parent_record_offset":4916,"ancestor_record_offset":4400,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":236,"record_offset":7344,"parent_record_offset":7320,"ancestor_record_offset":4400,"inlinee":5270,"source_line":218,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":236,"record_offset":7400,"parent_record_offset":7376,"ancestor_record_offset":4400,"inlinee":5271,"source_line":215,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":236,"record_offset":7288,"parent_record_offset":7264,"ancestor_record_offset":4400,"inlinee":5269,"source_line":209,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":236,"record_offset":4916,"parent_record_offset":4888,"ancestor_record_offset":4400,"inlinee":13465,"source_line":54,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::impl$9::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":236,"record_offset":14024,"parent_record_offset":13996,"ancestor_record_offset":4400,"inlinee":13473,"source_line":31,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::find_steam_user_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/main.rs
//     evidence={"module":236,"record_offset":6424,"parent_record_offset":6400,"ancestor_record_offset":4400,"inlinee":13471,"source_line":290,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::try_main::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/main.rs
//     evidence={"module":236,"record_offset":6400,"parent_record_offset":6368,"ancestor_record_offset":4400,"inlinee":13470,"source_line":186,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::try_main::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3494
// 首条位置: {"module":236,"inlinee":13460}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":236,"record_offset":1004,"parent_record_offset":972,"ancestor_record_offset":972,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::try_main::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":236,"record_offset":20712,"rva":3782320,"va":5372491440}
// 编译实例 RVAs: 0x39b6b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] fern::log_impl::FormatCallback::finish
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t639829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372491440,"raw_hex":"e855c30900","size":5,"soft_fail":false,"target_va":5373131440,"va":5372491606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":236,"record_offset":20828,"parent_record_offset":20712,"ancestor_record_offset":20712,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::try_main::closure$2
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 144
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3493
// 首条位置: {"module":236,"inlinee":13459}
// 编译实例 RVAs: 0x1d7d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3919745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368829904,"raw_hex":"e881cf3b00","size":5,"soft_fail":false,"target_va":5372749712,"va":5368829962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34763359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368829904,"raw_hex":"e85f721202","size":5,"soft_fail":false,"target_va":5403593360,"va":5368829996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":236,"record_offset":484,"parent_record_offset":456,"ancestor_record_offset":456,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":236,"record_offset":632,"parent_record_offset":608,"ancestor_record_offset":456,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":236,"record_offset":608,"parent_record_offset":584,"ancestor_record_offset":456,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":236,"record_offset":21108,"parent_record_offset":21080,"ancestor_record_offset":20860,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":236,"record_offset":584,"parent_record_offset":564,"ancestor_record_offset":456,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":236,"record_offset":504,"parent_record_offset":484,"ancestor_record_offset":456,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::try_main::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x349E
// 首条位置: {"module":236,"inlinee":13470}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tiny_glade::find_steam_user_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/main.rs
//     evidence={"module":236,"record_offset":6424,"parent_record_offset":6400,"ancestor_record_offset":6400,"inlinee":13471,"source_line":290,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::find_steam_user_id
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 290
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x349F
// 首条位置: {"module":236,"inlinee":13471}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::init_steam_integration
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 333
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x349A
// 首条位置: {"module":236,"inlinee":13466}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":236,"record_offset":5816,"parent_record_offset":5732,"ancestor_record_offset":5732,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":236,"record_offset":5900,"parent_record_offset":5732,"ancestor_record_offset":5732,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] std::env::set_var
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\env.rs
//     evidence={"module":236,"record_offset":5788,"parent_record_offset":5732,"ancestor_record_offset":5732,"inlinee":11089,"source_line":360,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":236,"record_offset":6080,"parent_record_offset":6056,"ancestor_record_offset":5732,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":236,"record_offset":6056,"parent_record_offset":6000,"ancestor_record_offset":5732,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":236,"record_offset":5864,"parent_record_offset":5840,"ancestor_record_offset":5732,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":236,"record_offset":6028,"parent_record_offset":6000,"ancestor_record_offset":5732,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":236,"record_offset":6000,"parent_record_offset":5972,"ancestor_record_offset":5732,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":236,"record_offset":5840,"parent_record_offset":5816,"ancestor_record_offset":5732,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":236,"record_offset":5924,"parent_record_offset":5900,"ancestor_record_offset":5732,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00236.jsonl.gz"}
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
// 完整结构及原始类型记录: main.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::vram_constants::MemoryScalingModel
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: main.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: main.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: minidump_unwind::mips::get_caller_frame::async_fn$0<minidump_unwind::symbols::MultiSymbolProvider>
// 信息 ID: 81bbd820a670901baaaa379f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] minidump_unwind::mips::get_caller_frame::async_fn$0 -> ["minidump_unwind::symbols::MultiSymbolProvider"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::find_steam_user_id
// 信息 ID: 4af0e1498a8b7550c012c5e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::main; module=236, inline_record=6424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::init_steam_integration
// 信息 ID: d9e518c3aad1f210b95eb528
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::main; module=236, inline_record=5732; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::log_time_to_first_frame
// 信息 ID: bddce63f33418b595bfe1a65
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=122904; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::main
// 信息 ID: 926f098f44c93edb18ea6cab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::try_main
// 信息 ID: caf0ac9a3f76d69c7b3c9519
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::main; module=236, inline_record=4400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::try_main::closure$0
// 信息 ID: 0d71cdeca99d84fca0ef9a75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::try_main", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::try_main::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::try_main::closure_env$0,tuple$<fern::log_impl::FormatCallback,ref$<core::fmt::Arguments>,ref$<log::Record> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::try_main::closure_env$0,tuple$<fern::log_impl::FormatCallback,ref$<core::fmt::Arguments>,ref$<log::Record> > >; module=236, inline_record=972; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::try_main::closure$1
// 信息 ID: d1803bb15ac84089e922e11e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::try_main", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::try_main::closure_env$1 <- core::ptr::drop_in_place<tiny_glade::try_main::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 1 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::try_main::closure$2
// 信息 ID: 685fb916d72e81ecfedcd916
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::try_main", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::try_main::closure_env$2 <- core::ops::function::FnOnce::call_once<tiny_glade::try_main::closure_env$2,tuple$<sentry_types::protocol::v7::Event> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::try_main::closure_env$2,tuple$<sentry_types::protocol::v7::Event> >; module=236, inline_record=456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::try_main::closure$3
// 信息 ID: e756ed40d8b495be3ffe71f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::try_main", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::main; module=236, inline_record=6400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
